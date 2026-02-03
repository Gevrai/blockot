// geometry/primitives.rs - Primitive shape generators
//
// Generates BlockotGeometry for common shapes (cube, etc.)

use godot::prelude::Vector3;

use super::{BlockotGeometry, Face};

/// Creates a unit cube (1m on each side) centered at origin.
///
/// ```text
///       4-------5
///      /|      /|
///     / |     / |
///    0-------1  |
///    |  7----|--6
///    | /     | /
///    |/      |/
///    3-------2
///
/// Vertices (1m cube centered at origin):
///   0: (-0.5, -0.5, -0.5)  front-bottom-left
///   1: ( 0.5, -0.5, -0.5)  front-bottom-right
///   2: ( 0.5, -0.5,  0.5)  back-bottom-right
///   3: (-0.5, -0.5,  0.5)  back-bottom-left
///   4: (-0.5,  0.5, -0.5)  front-top-left
///   5: ( 0.5,  0.5, -0.5)  front-top-right
///   6: ( 0.5,  0.5,  0.5)  back-top-right
///   7: (-0.5,  0.5,  0.5)  back-top-left
///
/// Faces (quads, counter-clockwise winding for outward normals):
///   Front:  0, 1, 5, 4
///   Back:   2, 3, 7, 6
///   Top:    4, 5, 6, 7
///   Bottom: 3, 2, 1, 0
///   Right:  1, 2, 6, 5
///   Left:   3, 0, 4, 7
/// ```
pub fn unit_cube() -> BlockotGeometry {
    let mut geo = BlockotGeometry::with_capacity(8, 6);

    // 8 vertices of a 1m cube centered at origin
    geo.vertices = vec![
        Vector3::new(-0.5, -0.5, -0.5), // 0: front-bottom-left
        Vector3::new(0.5, -0.5, -0.5),  // 1: front-bottom-right
        Vector3::new(0.5, -0.5, 0.5),   // 2: back-bottom-right
        Vector3::new(-0.5, -0.5, 0.5),  // 3: back-bottom-left
        Vector3::new(-0.5, 0.5, -0.5),  // 4: front-top-left
        Vector3::new(0.5, 0.5, -0.5),   // 5: front-top-right
        Vector3::new(0.5, 0.5, 0.5),    // 6: back-top-right
        Vector3::new(-0.5, 0.5, 0.5),   // 7: back-top-left
    ];

    // 6 quad faces with counter-clockwise winding (outward normals)
    geo.faces = vec![
        Face::quad(0, 1, 5, 4), // Front
        Face::quad(2, 3, 7, 6), // Back
        Face::quad(4, 5, 6, 7), // Top
        Face::quad(3, 2, 1, 0), // Bottom
        Face::quad(1, 2, 6, 5), // Right
        Face::quad(3, 0, 4, 7), // Left
    ];

    geo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_cube_geometry() {
        let cube = unit_cube();

        // Verify vertex and face counts
        assert_eq!(cube.vertex_count(), 8, "Cube should have 8 vertices");
        assert_eq!(cube.face_count(), 6, "Cube should have 6 faces");

        // Verify all faces are quads
        for (i, face) in cube.faces.iter().enumerate() {
            assert!(face.is_quad(), "Face {} should be a quad", i);
        }

        // Verify cube is centered at origin
        let center: Vector3 = cube.vertices.iter().fold(Vector3::ZERO, |acc, v| acc + *v) / 8.0;
        assert!(
            center.length() < 1e-6,
            "Cube should be centered at origin, center was {:?}",
            center
        );

        // Verify dimensions are 1m on each side
        let mut min = Vector3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vector3::new(f32::MIN, f32::MIN, f32::MIN);

        for v in &cube.vertices {
            min.x = min.x.min(v.x);
            min.y = min.y.min(v.y);
            min.z = min.z.min(v.z);
            max.x = max.x.max(v.x);
            max.y = max.y.max(v.y);
            max.z = max.z.max(v.z);
        }

        let size = max - min;
        assert!(
            (size.x - 1.0).abs() < 1e-6,
            "Cube width should be 1m, was {}",
            size.x
        );
        assert!(
            (size.y - 1.0).abs() < 1e-6,
            "Cube height should be 1m, was {}",
            size.y
        );
        assert!(
            (size.z - 1.0).abs() < 1e-6,
            "Cube depth should be 1m, was {}",
            size.z
        );
    }

    #[test]
    fn test_unit_cube_vertex_positions() {
        let cube = unit_cube();

        // Verify specific vertex positions
        assert_eq!(cube.vertices[0], Vector3::new(-0.5, -0.5, -0.5));
        assert_eq!(cube.vertices[1], Vector3::new(0.5, -0.5, -0.5));
        assert_eq!(cube.vertices[6], Vector3::new(0.5, 0.5, 0.5));
        assert_eq!(cube.vertices[7], Vector3::new(-0.5, 0.5, 0.5));
    }

    #[test]
    fn test_unit_cube_is_dirty() {
        let cube = unit_cube();
        assert!(cube.dirty, "New cube should be marked dirty");
    }
}
