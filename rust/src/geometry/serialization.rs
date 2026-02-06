// geometry/serialization.rs - Serialization functions for BlockotGeometry
//
// CRITICAL: This module is the ONE exception to "no Godot types in geometry".
// Serialization explicitly is a boundary function using Godot packed arrays.
// [Source: architecture.md#Serialization-Boundary]

use godot::prelude::*;

use super::{BlockotGeometry, Face};

/// Convert BlockotGeometry to packed arrays for Godot serialization.
///
/// Returns (vertices, face_vertex_counts, face_indices) where:
/// - vertices: All vertex positions as PackedVector3Array
/// - face_vertex_counts: Number of vertices per face (e.g., [4,4,4,4,4,4] for 6 quads)
/// - face_indices: Flattened vertex indices for all faces
///
/// This format is git-diffable when saved in .tscn files.
pub fn to_packed_arrays(
    geo: &BlockotGeometry,
) -> (PackedVector3Array, PackedInt32Array, PackedInt32Array) {
    let mut vertices = PackedVector3Array::new();
    for v in &geo.vertices {
        vertices.push(*v);
    }

    let mut face_vertex_counts = PackedInt32Array::new();
    let mut face_indices = PackedInt32Array::new();

    for face in &geo.faces {
        face_vertex_counts.push(face.vertex_indices.len() as i32);
        for &idx in &face.vertex_indices {
            face_indices.push(idx as i32);
        }
    }

    (vertices, face_vertex_counts, face_indices)
}

/// Convert packed arrays back to BlockotGeometry.
///
/// Returns None if arrays are inconsistent (invalid data).
/// On success, the returned geometry has dirty=true (needs cache rebuild).
pub fn from_packed_arrays(
    vertices: &PackedVector3Array,
    face_vertex_counts: &PackedInt32Array,
    face_indices: &PackedInt32Array,
) -> Option<BlockotGeometry> {
    let mut geo = BlockotGeometry::new();

    // Load vertices
    for i in 0..vertices.len() {
        geo.vertices.push(vertices.get(i)?);
    }

    // Load faces
    let mut idx_offset = 0usize;
    for i in 0..face_vertex_counts.len() {
        let count = face_vertex_counts.get(i)? as usize;
        let mut indices = Vec::with_capacity(count);

        for j in 0..count {
            let global_idx = idx_offset + j;
            if global_idx >= face_indices.len() {
                return None; // Invalid data - not enough indices
            }
            let vertex_idx = face_indices.get(global_idx)? as usize;
            // Validate that vertex index is within bounds
            if vertex_idx >= geo.vertices.len() {
                return None; // Invalid data - vertex index out of bounds
            }
            indices.push(vertex_idx);
        }

        geo.faces.push(Face::new(indices));
        idx_offset += count;
    }

    // Verify all indices were consumed
    if idx_offset != face_indices.len() {
        return None; // Invalid data - extra indices
    }

    geo.mark_dirty(); // Needs cache rebuild
    Some(geo)
}

// NOTE: Tests for serialization functions require Godot runtime (PackedArrays).
// These tests are located in tests/serialization.rs and marked with #[ignore] for Godot-dependent tests.
// They can be run with Godot present using: cargo test -- --ignored
