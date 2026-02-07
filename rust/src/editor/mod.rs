// editor/mod.rs - Editor module (Godot integration layer)
//
// This is the ONLY module where Godot types are allowed.
// Bridges pure Rust geometry/tools to Godot's systems.

mod blockot_node;
pub mod edit_mode;
mod history;
mod plugin;

pub use blockot_node::BlockotNode;
pub use edit_mode::EditModeState;
pub use history::{execute_with_undo, execute_without_undo, undo_command};
pub use plugin::BlockotPlugin;
