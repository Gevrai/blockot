// test_utils.rs - Test fixtures and helpers
//
// This module is `pub mod` (not cfg(test)) so it can be used by integration tests.
// Contains shared geometry builders and comparison helpers.

use godot::prelude::Vector3;

use crate::geometry::{BlockotGeometry, Face};

/// Creates a unit cube fixture for testing.
/// Wrapper around primitives::unit_cube() for test convenience.
pub fn unit_cube() -> BlockotGeometry {
    crate::geometry::primitives::unit_cube()
}

/// Creates a minimal single-face geometry for testing.
/// Returns a single triangle face with 3 vertices.
pub fn single_face() -> BlockotGeometry {
    let mut geo = BlockotGeometry::with_capacity(3, 1);

    geo.vertices = vec![
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.5, 1.0, 0.0),
    ];

    geo.faces = vec![Face::triangle(0, 1, 2)];

    geo
}

/// Creates a single quad face for testing.
pub fn single_quad() -> BlockotGeometry {
    let mut geo = BlockotGeometry::with_capacity(4, 1);

    geo.vertices = vec![
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(1.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    ];

    geo.faces = vec![Face::quad(0, 1, 2, 3)];

    geo
}

/// Compare two Vector3 values with floating point tolerance.
pub fn vectors_approx_equal(a: Vector3, b: Vector3, epsilon: f32) -> bool {
    (a.x - b.x).abs() < epsilon && (a.y - b.y).abs() < epsilon && (a.z - b.z).abs() < epsilon
}

/// Compare two geometries for equality (vertices and faces).
/// Uses floating point tolerance for vertex comparison.
pub fn geometries_equal(a: &BlockotGeometry, b: &BlockotGeometry) -> bool {
    if a.vertices.len() != b.vertices.len() || a.faces.len() != b.faces.len() {
        return false;
    }

    const EPSILON: f32 = 1e-6;

    for (va, vb) in a.vertices.iter().zip(b.vertices.iter()) {
        if !vectors_approx_equal(*va, *vb, EPSILON) {
            return false;
        }
    }

    a.faces == b.faces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_cube_fixture() {
        let cube = unit_cube();
        assert_eq!(cube.vertex_count(), 8);
        assert_eq!(cube.face_count(), 6);
    }

    #[test]
    fn test_single_face_fixture() {
        let geo = single_face();
        assert_eq!(geo.vertex_count(), 3);
        assert_eq!(geo.face_count(), 1);
        assert!(geo.faces[0].is_triangle());
    }

    #[test]
    fn test_single_quad_fixture() {
        let geo = single_quad();
        assert_eq!(geo.vertex_count(), 4);
        assert_eq!(geo.face_count(), 1);
        assert!(geo.faces[0].is_quad());
    }

    #[test]
    fn test_vectors_approx_equal() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(1.0 + 1e-7, 2.0, 3.0);
        assert!(vectors_approx_equal(a, b, 1e-6));

        let c = Vector3::new(1.1, 2.0, 3.0);
        assert!(!vectors_approx_equal(a, c, 1e-6));
    }

    #[test]
    fn test_geometries_equal() {
        let geo1 = unit_cube();
        let geo2 = unit_cube();
        assert!(geometries_equal(&geo1, &geo2));
    }
}
