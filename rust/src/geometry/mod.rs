// geometry/mod.rs - Public API for geometry module
//
// This module is PURE RUST - no Godot types except Vector3 (math type).
// All Godot-specific conversions happen in editor/blockot_node.rs
//
// EXCEPTION: serialization.rs uses Godot packed arrays as it is a boundary function.
// [Source: architecture.md#Serialization-Boundary]

mod face;
mod mesh;
pub mod primitives;
pub mod serialization;

pub use face::Face;
pub use mesh::BlockotGeometry;
