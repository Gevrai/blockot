// editor/blockot_node.rs - BlockotNode class (extends MeshInstance3D)
//
// This is where Godot integration happens. The BlockotNode:
// - Owns the BlockotGeometry (source of truth)
// - Rebuilds ArrayMesh when geometry is dirty
// - Provides test methods for undo spike verification

use godot::classes::mesh::ArrayType;
use godot::classes::mesh::PrimitiveType;
use godot::classes::{
    ArrayMesh, Engine, IMeshInstance3D, Material, MeshInstance3D, Object, StandardMaterial3D,
};
use godot::prelude::*;

use crate::geometry::primitives::unit_cube;
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
}

#[godot_api]
impl IMeshInstance3D for BlockotNode {
    fn init(base: Base<MeshInstance3D>) -> Self {
        Self {
            base,
            geometry: unit_cube(),
            default_material: None,
        }
    }

    fn ready(&mut self) {
        self.setup_default_material();
        self.rebuild_array_mesh();
        godot_print!(
            "BlockotNode ready with {} vertices, {} faces",
            self.geometry.vertices.len(),
            self.geometry.faces.len()
        );
    }

    fn exit_tree(&mut self) {
        // TODO: Cancel any active preview when node exits tree
        // Per architecture.md "External Modification Pattern":
        // - Preview should be cancelled on scene exit
        // - Implement when preview system is added (Epic 2)
    }
}

#[godot_api]
impl BlockotNode {
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
}
