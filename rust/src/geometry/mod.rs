// geometry/mod.rs - Public API for geometry module
//
// This module is PURE RUST - no Godot types except Vector3 (math type).
// All Godot-specific conversions happen in editor/blockot_node.rs

mod face;
mod mesh;
pub mod primitives;

pub use face::Face;
pub use mesh::BlockotGeometry;
