// editor/blockot_node.rs - BlockotNode class (extends MeshInstance3D)
//
// This is where Godot integration happens. The BlockotNode:
// - Owns the BlockotGeometry (source of truth)
// - Rebuilds ArrayMesh when geometry is dirty
// - Provides test methods for undo spike verification

use godot::classes::mesh::ArrayType;
use godot::classes::mesh::PrimitiveType;
use godot::classes::notify::Node3DNotification;
use godot::classes::{
    ArrayMesh, Engine, IMeshInstance3D, ImmediateMesh, Material, MeshInstance3D, Object,
    StandardMaterial3D,
};
use godot::prelude::*;

use crate::geometry::primitives::unit_cube;
use crate::geometry::serialization::{from_packed_arrays, to_packed_arrays};
use crate::geometry::BlockotGeometry;

/// A custom node for blockout geometry editing.
/// Extends MeshInstance3D and displays editable geometry.
#[derive(GodotClass)]
#[class(base=MeshInstance3D, tool)]
pub struct BlockotNode {
    base: Base<MeshInstance3D>,

    /// Source of truth for geometry (pure Rust)
    geometry: BlockotGeometry,

    /// Cached default material
    #[var]
    default_material: Option<Gd<Material>>,

    /// Whether this node is currently in edit mode
    #[export]
    is_in_edit_mode: bool,

    /// MeshInstance3D child used to render vertex handles in edit mode
    handle_mesh_instance: Option<Gd<MeshInstance3D>>,

    // Export fields for serialization (saved in .tscn files)
    // These are synced to/from geometry on save/load.
    // Format is git-diffable (text-based PackedArrays).
    // [Source: architecture.md#Decision-5-Flat-Array-Serialization]

    /// Serialized vertex positions
    #[export]
    vertices: PackedVector3Array,

    /// Number of vertices per face (e.g., [4,4,4,4,4,4] for 6 quads)
    #[export]
    face_vertex_counts: PackedInt32Array,

    /// Flattened vertex indices for all faces
    #[export]
    face_indices: PackedInt32Array,
}

#[godot_api]
impl IMeshInstance3D for BlockotNode {
    fn init(base: Base<MeshInstance3D>) -> Self {
        Self {
            base,
            geometry: BlockotGeometry::new(), // Start empty, load in ready()
            default_material: None,
            is_in_edit_mode: false,
            handle_mesh_instance: None,
            vertices: PackedVector3Array::new(),
            face_vertex_counts: PackedInt32Array::new(),
            face_indices: PackedInt32Array::new(),
        }
    }

    fn ready(&mut self) {
        // Load from export fields if available, otherwise init with cube
        if !self.vertices.is_empty() {
            self.load_geometry_from_export();
        } else {
            self.geometry = unit_cube();
            self.sync_geometry_to_export(); // Populate export fields
        }

        self.setup_default_material();
        self.rebuild_array_mesh();
        godot_print!(
            "BlockotNode ready with {} vertices, {} faces",
            self.geometry.vertices.len(),
            self.geometry.faces.len()
        );
    }

    fn exit_tree(&mut self) {
        // Exit edit mode when node leaves the tree
        if self.is_in_edit_mode {
            self.is_in_edit_mode = false;
            self.hide_vertex_handles();
        }
    }

    fn on_notification(&mut self, what: Node3DNotification) {
        // Sync geometry to export fields before scene is saved
        if what == Node3DNotification::EDITOR_PRE_SAVE {
            self.sync_geometry_to_export();
        }
    }
}

#[godot_api]
impl BlockotNode {
    #[signal]
    fn edit_mode_entered();

    #[signal]
    fn edit_mode_exited();

    /// Returns the number of vertices in the geometry.
    #[func]
    pub fn get_vertex_count(&self) -> i32 {
        self.geometry.vertices.len() as i32
    }

    /// Returns the number of faces in the geometry.
    #[func]
    pub fn get_face_count(&self) -> i32 {
        self.geometry.faces.len() as i32
    }


    /// Test method for undo spike - moves a single vertex.
    /// This is temporary for verification, will be removed/replaced in Epic 2.
    #[func]
    pub fn test_move_vertex(&mut self, index: i32, offset: Vector3) {
        use godot::classes::EditorInterface;

        let idx = index as usize;
        if idx >= self.geometry.vertices.len() {
            godot_error!("Invalid vertex index: {}", index);
            return;
        }

        // Try to use undo/redo in editor
        if Engine::singleton().is_editor_hint() {
            if let Some(mut undo_redo) = EditorInterface::singleton().get_editor_undo_redo() {
                let action_name = GString::from("Move Vertex");
                undo_redo.create_action(&action_name);

                // Get self as object for method registration
                let obj: Gd<Object> = self.base().clone().upcast();
                let method_name = StringName::from("_apply_vertex_move");

                // Register do method (apply the move)
                undo_redo.add_do_method(
                    &obj,
                    &method_name,
                    &[index.to_variant(), offset.to_variant()],
                );

                // Register undo method (reverse the move)
                undo_redo.add_undo_method(
                    &obj,
                    &method_name,
                    &[index.to_variant(), (-offset).to_variant()],
                );

                // Commit WITHOUT executing (execute=false) to avoid borrow conflict
                // We'll apply the change ourselves after registering
                undo_redo.commit_action_ex().execute(false).done();

                godot_print!("Registered undo action: Move Vertex {}", index);
            }
        }

        // Apply the move directly (either as fallback or after undo registration)
        self.geometry.vertices[idx] += offset;
        self.geometry.dirty = true;
        self.rebuild_array_mesh();
        godot_print!("Applied vertex move: {} by {:?}", index, offset);
    }

    /// Internal method called by undo/redo system to apply vertex movement.
    #[func]
    pub fn _apply_vertex_move(&mut self, index: i32, offset: Vector3) {
        let idx = index as usize;
        if idx < self.geometry.vertices.len() {
            self.geometry.vertices[idx] += offset;
            self.geometry.dirty = true;
            self.rebuild_array_mesh();
            godot_print!("Applied vertex move: {} by {:?}", index, offset);
        }
    }

    /// Force rebuild the mesh from geometry.
    #[func]
    pub fn force_rebuild_mesh(&mut self) {
        self.geometry.dirty = true;
        self.rebuild_array_mesh();
    }
}

impl BlockotNode {
    /// Enter edit mode on this node. Shows vertex handles and emits signal.
    pub fn enter_edit_mode(&mut self) {
        if self.is_in_edit_mode {
            return;
        }
        self.is_in_edit_mode = true;
        self.show_vertex_handles();
        self.base_mut()
            .emit_signal("edit_mode_entered", &[]);
        godot_print!("BlockotNode: Entered edit mode");
    }

    /// Exit edit mode on this node. Hides vertex handles and emits signal.
    pub fn exit_edit_mode(&mut self) {
        if !self.is_in_edit_mode {
            return;
        }
        self.is_in_edit_mode = false;
        self.hide_vertex_handles();
        self.base_mut()
            .emit_signal("edit_mode_exited", &[]);
        godot_print!("BlockotNode: Exited edit mode");
    }

    /// Create and show vertex handles as small points at each vertex position.
    fn show_vertex_handles(&mut self) {
        self.hide_vertex_handles(); // Clean up any existing handles

        let mut immediate_mesh = ImmediateMesh::new_gd();

        // Draw vertex points using small cross shapes for visibility
        immediate_mesh.surface_begin(PrimitiveType::LINES);

        let handle_color = Color::from_rgb(1.0, 0.5, 0.0); // Orange for visibility
        let handle_size = 0.03;

        for vertex in &self.geometry.vertices {
            immediate_mesh.surface_set_color(handle_color);

            // Draw a small 3D cross at each vertex
            // X axis
            immediate_mesh.surface_add_vertex(*vertex + Vector3::new(-handle_size, 0.0, 0.0));
            immediate_mesh.surface_add_vertex(*vertex + Vector3::new(handle_size, 0.0, 0.0));
            // Y axis
            immediate_mesh.surface_add_vertex(*vertex + Vector3::new(0.0, -handle_size, 0.0));
            immediate_mesh.surface_add_vertex(*vertex + Vector3::new(0.0, handle_size, 0.0));
            // Z axis
            immediate_mesh.surface_add_vertex(*vertex + Vector3::new(0.0, 0.0, -handle_size));
            immediate_mesh.surface_add_vertex(*vertex + Vector3::new(0.0, 0.0, handle_size));
        }

        immediate_mesh.surface_end();

        // Create a MeshInstance3D child to display the handles
        let mut mesh_instance = MeshInstance3D::new_alloc();
        mesh_instance.set_mesh(&immediate_mesh);

        // Create an unshaded material so handles are always visible
        let mut handle_material = StandardMaterial3D::new_gd();
        handle_material.set_shading_mode(
            godot::classes::base_material_3d::ShadingMode::UNSHADED,
        );
        handle_material.set_flag(
            godot::classes::base_material_3d::Flags::ALBEDO_FROM_VERTEX_COLOR,
            true,
        );
        handle_material.set_flag(
            godot::classes::base_material_3d::Flags::DISABLE_DEPTH_TEST,
            true,
        );
        mesh_instance.set_material_override(&handle_material.upcast::<Material>());

        self.base_mut().add_child(&mesh_instance);
        self.handle_mesh_instance = Some(mesh_instance);
    }

    /// Remove vertex handle visualization.
    fn hide_vertex_handles(&mut self) {
        if let Some(mut mesh_instance) = self.handle_mesh_instance.take() {
            mesh_instance.queue_free();
        }
    }

    /// Set up a default material for the mesh.
    fn setup_default_material(&mut self) {
        let mut material = StandardMaterial3D::new_gd();
        material.set_albedo(Color::from_rgb(0.8, 0.8, 0.8));
        self.default_material = Some(material.upcast());
    }

    /// Rebuild the ArrayMesh from the current geometry.
    /// Called when geometry.dirty is true.
    pub fn rebuild_array_mesh(&mut self) {
        let mut vertices = PackedVector3Array::new();
        let mut normals = PackedVector3Array::new();
        let mut indices = PackedInt32Array::new();

        let mut vertex_index: i32 = 0;

        for face in &self.geometry.faces {
            if face.vertex_indices.len() < 3 {
                continue;
            }

            // Calculate flat normal for the face
            let normal = self.calculate_face_normal(face);

            // Triangulate the face (works for triangles, quads, and n-gons)
            // Fan triangulation: (0, 1, 2), (0, 2, 3), (0, 3, 4), ...
            let first_idx = face.vertex_indices[0];
            let first_vertex = self.geometry.vertices[first_idx];

            for i in 1..(face.vertex_indices.len() - 1) {
                let idx1 = face.vertex_indices[i];
                let idx2 = face.vertex_indices[i + 1];

                let v1 = self.geometry.vertices[idx1];
                let v2 = self.geometry.vertices[idx2];

                // Add three vertices for this triangle (with flat shading normals)
                vertices.push(first_vertex);
                normals.push(normal);
                indices.push(vertex_index);
                vertex_index += 1;

                vertices.push(v1);
                normals.push(normal);
                indices.push(vertex_index);
                vertex_index += 1;

                vertices.push(v2);
                normals.push(normal);
                indices.push(vertex_index);
                vertex_index += 1;
            }
        }

        // Create the ArrayMesh
        let mut mesh = ArrayMesh::new_gd();

        if !vertices.is_empty() {
            // Create a Godot array with ArrayType::MAX elements
            let mut arrays: Array<Variant> = Array::new();
            arrays.resize(ArrayType::MAX.ord() as usize, &Variant::nil());

            arrays.set(ArrayType::VERTEX.ord() as usize, &vertices.to_variant());
            arrays.set(ArrayType::NORMAL.ord() as usize, &normals.to_variant());
            arrays.set(ArrayType::INDEX.ord() as usize, &indices.to_variant());

            mesh.add_surface_from_arrays(PrimitiveType::TRIANGLES, &arrays);

            // Apply default material if available
            if let Some(ref material) = self.default_material {
                mesh.surface_set_material(0, material);
            }
        }

        self.base_mut().set_mesh(&mesh);
        self.geometry.dirty = false;
    }

    /// Calculate the flat normal for a face.
    /// Uses counter-clockwise winding convention (when viewed from outside).
    fn calculate_face_normal(&self, face: &crate::geometry::Face) -> Vector3 {
        if face.vertex_indices.len() < 3 {
            return Vector3::UP;
        }

        let v0 = self.geometry.vertices[face.vertex_indices[0]];
        let v1 = self.geometry.vertices[face.vertex_indices[1]];
        let v2 = self.geometry.vertices[face.vertex_indices[2]];

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;

        // Cross product order: edge2 × edge1 for outward-facing normals
        // with counter-clockwise vertex winding
        let cross = edge2.cross(edge1);

        // Handle degenerate case (collinear vertices)
        let length_sq = cross.length_squared();
        if length_sq < 1e-10 {
            return Vector3::UP;
        }

        cross / length_sq.sqrt()
    }

    /// Get mutable access to geometry (for commands).
    pub fn geometry_mut(&mut self) -> &mut BlockotGeometry {
        &mut self.geometry
    }

    /// Get read access to geometry.
    pub fn geometry(&self) -> &BlockotGeometry {
        &self.geometry
    }

    /// Sync internal geometry to export fields (called before save).
    /// This populates the #[export] fields that get saved to .tscn files.
    fn sync_geometry_to_export(&mut self) {
        let (verts, counts, indices) = to_packed_arrays(&self.geometry);
        self.vertices = verts;
        self.face_vertex_counts = counts;
        self.face_indices = indices;

        // Notify Godot that export properties changed so they get saved
        self.base_mut().notify_property_list_changed();
    }

    /// Load geometry from export fields (called on scene load).
    /// Restores BlockotGeometry from the saved PackedArrays.
    fn load_geometry_from_export(&mut self) {
        if let Some(geo) = from_packed_arrays(
            &self.vertices,
            &self.face_vertex_counts,
            &self.face_indices,
        ) {
            self.geometry = geo;
            godot_print!(
                "BlockotNode: Loaded geometry from saved data ({} vertices, {} faces)",
                self.geometry.vertices.len(),
                self.geometry.faces.len()
            );
        } else {
            godot_warn!("BlockotNode: Failed to load geometry from saved data, using default cube");
            self.geometry = unit_cube();
        }
    }
}
