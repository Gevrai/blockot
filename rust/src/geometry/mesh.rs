// geometry/mesh.rs - BlockotGeometry struct (source of truth)
//
// CRITICAL: This is PURE RUST - no Godot types except Vector3 (math type).
// The cached_mesh lives in BlockotNode (editor module), NOT here.

use godot::prelude::Vector3;

use super::Face;

/// The source of truth for blockot geometry.
/// Contains vertices and faces in a pure Rust representation.
#[derive(Debug, Clone)]
pub struct BlockotGeometry {
    /// Vertex positions
    pub vertices: Vec<Vector3>,

    /// Faces referencing vertex indices
    pub faces: Vec<Face>,

    /// Flag indicating if geometry has been modified since last cache rebuild
    pub dirty: bool,
}

impl BlockotGeometry {
    /// Create a new empty geometry
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
            dirty: true,
        }
    }

    /// Create geometry with pre-allocated capacity
    pub fn with_capacity(vertex_count: usize, face_count: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(vertex_count),
            faces: Vec::with_capacity(face_count),
            dirty: true,
        }
    }

    /// Returns the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of faces
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    /// Mark geometry as modified (requires cache rebuild)
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Mark geometry as clean (cache is up to date)
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}

impl Default for BlockotGeometry {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for BlockotGeometry {
    fn eq(&self, other: &Self) -> bool {
        // Compare vertices with floating point tolerance
        if self.vertices.len() != other.vertices.len() {
            return false;
        }
        for (a, b) in self.vertices.iter().zip(other.vertices.iter()) {
            if !vectors_equal(*a, *b) {
                return false;
            }
        }
        self.faces == other.faces
    }
}

/// Compare two Vector3 values with floating point tolerance
fn vectors_equal(a: Vector3, b: Vector3) -> bool {
    const EPSILON: f32 = 1e-6;
    (a.x - b.x).abs() < EPSILON && (a.y - b.y).abs() < EPSILON && (a.z - b.z).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_geometry_is_empty() {
        let geo = BlockotGeometry::new();
        assert_eq!(geo.vertex_count(), 0);
        assert_eq!(geo.face_count(), 0);
        assert!(geo.dirty);
    }

    #[test]
    fn test_dirty_flag() {
        let mut geo = BlockotGeometry::new();
        assert!(geo.dirty);

        geo.mark_clean();
        assert!(!geo.dirty);

        geo.mark_dirty();
        assert!(geo.dirty);
    }

    #[test]
    fn test_geometry_equality() {
        let mut geo1 = BlockotGeometry::new();
        geo1.vertices.push(Vector3::new(1.0, 2.0, 3.0));
        geo1.faces.push(Face::triangle(0, 1, 2));

        let mut geo2 = BlockotGeometry::new();
        geo2.vertices.push(Vector3::new(1.0, 2.0, 3.0));
        geo2.faces.push(Face::triangle(0, 1, 2));

        assert_eq!(geo1, geo2);
    }
}
