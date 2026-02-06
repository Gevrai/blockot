// tests/serialization.rs - Integration tests for geometry serialization
//
// NOTE: These tests require the Godot runtime to be available.
// Run with: cargo test -- --ignored (when Godot is available)
// Or run within Godot editor using gdext's test runner.

use blockot::geometry::primitives::unit_cube;
use blockot::geometry::serialization::{from_packed_arrays, to_packed_arrays};
use blockot::geometry::{BlockotGeometry, Face};
use godot::prelude::*;

/// Test: Serialize and deserialize a unit cube, verify exact match
/// Requires Godot runtime for PackedArray types.
#[test]
#[ignore = "requires Godot runtime - run in Godot editor or with --ignored flag when Godot is available"]
fn test_serialization_roundtrip_integration() {
    let original = unit_cube();
    let (verts, counts, indices) = to_packed_arrays(&original);
    let restored = from_packed_arrays(&verts, &counts, &indices).unwrap();
    assert_eq!(
        original, restored,
        "Roundtrip should preserve geometry exactly"
    );
}

/// Test: Serialize and verify packed array sizes for cube
#[test]
#[ignore = "requires Godot runtime - run in Godot editor or with --ignored flag when Godot is available"]
fn test_cube_serialization_sizes() {
    let cube = unit_cube();
    let (v, c, i) = to_packed_arrays(&cube);

    assert_eq!(v.len(), 8, "Cube should have 8 vertices");
    assert_eq!(c.len(), 6, "Cube should have 6 faces");
    assert_eq!(
        i.len(),
        24,
        "Cube should have 24 indices (6 quads * 4 verts)"
    );

    // Verify all faces are quads
    for idx in 0..c.len() {
        let count = c.get(idx).unwrap();
        assert_eq!(count, 4, "Face {} should be a quad", idx);
    }
}

/// Test: Empty geometry roundtrip
#[test]
#[ignore = "requires Godot runtime - run in Godot editor or with --ignored flag when Godot is available"]
fn test_empty_geometry_serialization() {
    let empty = BlockotGeometry::new();
    let (verts, counts, indices) = to_packed_arrays(&empty);

    assert_eq!(verts.len(), 0, "Empty geometry should have no vertices");
    assert_eq!(counts.len(), 0, "Empty geometry should have no faces");
    assert_eq!(indices.len(), 0, "Empty geometry should have no indices");

    let restored = from_packed_arrays(&verts, &counts, &indices).unwrap();
    assert_eq!(empty, restored, "Empty geometry roundtrip should work");
}

/// Test: Complex geometry with modified vertices roundtrip
#[test]
#[ignore = "requires Godot runtime - run in Godot editor or with --ignored flag when Godot is available"]
fn test_modified_geometry_roundtrip() {
    let mut geo = unit_cube();

    // Modify some vertices
    geo.vertices[0] = Vector3::new(-1.0, -1.0, -1.0);
    geo.vertices[1] = Vector3::new(1.0, -1.0, -1.0);
    geo.vertices[6] = Vector3::new(1.0, 1.0, 1.0);

    let (verts, counts, indices) = to_packed_arrays(&geo);
    let restored = from_packed_arrays(&verts, &counts, &indices).unwrap();

    assert_eq!(
        geo, restored,
        "Modified geometry should roundtrip correctly"
    );
}

/// Test: Invalid data handling
#[test]
#[ignore = "requires Godot runtime - run in Godot editor or with --ignored flag when Godot is available"]
fn test_invalid_data_returns_none() {
    // Test case: vertex index out of bounds
    let mut vertices = PackedVector3Array::new();
    vertices.push(Vector3::ZERO);

    let mut counts = PackedInt32Array::new();
    counts.push(1);

    let mut indices = PackedInt32Array::new();
    indices.push(100); // Index 100 doesn't exist

    assert!(
        from_packed_arrays(&vertices, &counts, &indices).is_none(),
        "Out of bounds vertex index should return None"
    );

    // Test case: mismatched counts and indices
    let mut vertices = PackedVector3Array::new();
    vertices.push(Vector3::ZERO);
    vertices.push(Vector3::ONE);

    let mut counts = PackedInt32Array::new();
    counts.push(3); // Claims 3 vertices in one face

    let mut indices = PackedInt32Array::new();
    indices.push(0);
    indices.push(1); // Only 2 indices provided

    assert!(
        from_packed_arrays(&vertices, &counts, &indices).is_none(),
        "Mismatched count and indices should return None"
    );
}

/// Test: Triangle and mixed face geometry
#[test]
#[ignore = "requires Godot runtime - run in Godot editor or with --ignored flag when Godot is available"]
fn test_triangle_geometry_roundtrip() {
    let mut geo = BlockotGeometry::new();

    // Create a simple triangle
    geo.vertices.push(Vector3::ZERO);
    geo.vertices.push(Vector3::RIGHT);
    geo.vertices.push(Vector3::UP);
    geo.faces.push(Face::triangle(0, 1, 2));

    let (verts, counts, indices) = to_packed_arrays(&geo);

    assert_eq!(verts.len(), 3, "Triangle should have 3 vertices");
    assert_eq!(counts.len(), 1, "Should have 1 face");
    assert_eq!(counts.get(0), Some(3), "Face should have 3 vertices");
    assert_eq!(indices.len(), 3, "Should have 3 indices");

    let restored = from_packed_arrays(&verts, &counts, &indices).unwrap();
    assert_eq!(
        geo, restored,
        "Triangle geometry should roundtrip correctly"
    );
}

/// Test: Large vertex values roundtrip correctly
#[test]
#[ignore = "requires Godot runtime - run in Godot editor or with --ignored flag when Godot is available"]
fn test_large_vertex_values() {
    let mut geo = BlockotGeometry::new();

    geo.vertices.push(Vector3::new(1000.0, -1000.0, 500.0));
    geo.vertices.push(Vector3::new(-500.0, 1000.0, -1000.0));
    geo.vertices.push(Vector3::new(0.0001, 0.00001, 0.000001));
    geo.faces.push(Face::triangle(0, 1, 2));

    let (verts, counts, indices) = to_packed_arrays(&geo);
    let restored = from_packed_arrays(&verts, &counts, &indices).unwrap();

    assert_eq!(
        geo, restored,
        "Large and small vertex values should roundtrip correctly"
    );
}
