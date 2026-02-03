// geometry/face.rs - Face struct for n-gon support

/// Represents a face (polygon) in BlockotGeometry.
/// Supports n-gons (triangles, quads, or more vertices).
#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    /// Indices into BlockotGeometry.vertices
    pub vertex_indices: Vec<usize>,
}

impl Face {
    /// Create a new face from vertex indices
    pub fn new(indices: Vec<usize>) -> Self {
        Self {
            vertex_indices: indices,
        }
    }

    /// Create a quad face (4 vertices)
    pub fn quad(a: usize, b: usize, c: usize, d: usize) -> Self {
        Self {
            vertex_indices: vec![a, b, c, d],
        }
    }

    /// Create a triangle face (3 vertices)
    pub fn triangle(a: usize, b: usize, c: usize) -> Self {
        Self {
            vertex_indices: vec![a, b, c],
        }
    }

    /// Returns the number of vertices in this face
    pub fn vertex_count(&self) -> usize {
        self.vertex_indices.len()
    }

    /// Returns true if this face is a triangle
    pub fn is_triangle(&self) -> bool {
        self.vertex_indices.len() == 3
    }

    /// Returns true if this face is a quad
    pub fn is_quad(&self) -> bool {
        self.vertex_indices.len() == 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quad_creation() {
        let face = Face::quad(0, 1, 2, 3);
        assert_eq!(face.vertex_indices, vec![0, 1, 2, 3]);
        assert!(face.is_quad());
        assert!(!face.is_triangle());
        assert_eq!(face.vertex_count(), 4);
    }

    #[test]
    fn test_triangle_creation() {
        let face = Face::triangle(0, 1, 2);
        assert_eq!(face.vertex_indices, vec![0, 1, 2]);
        assert!(face.is_triangle());
        assert!(!face.is_quad());
        assert_eq!(face.vertex_count(), 3);
    }
}
