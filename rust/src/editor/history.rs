// editor/history.rs - EditorUndoRedoManager bridge
//
// Bridges the Command trait to Godot's undo system.
// Commands are executed immediately, then registered with EditorUndoRedoManager.
//
// NOTE: This module provides scaffolding for the undo integration pattern.
// The actual undo spike for Story 1.2 is implemented directly in BlockotNode::test_move_vertex()
// which demonstrates the working pattern. This module will be fully implemented in later stories
// when we need generic command undo support.

use godot::prelude::*;

use crate::editor::BlockotNode;
use crate::geometry::BlockotGeometry;
use crate::tools::Command;

/// Execute a command on geometry with undo/redo support.
///
/// This function:
/// 1. Optionally validates the command against the geometry
/// 2. Executes the command immediately on the geometry
/// 3. Registers the action with Godot's EditorUndoRedoManager (when available)
///
/// # Warning
/// This is scaffolding for future implementation. The undo registration is incomplete.
/// For Story 1.2, use `BlockotNode::test_move_vertex()` which has working undo support.
///
/// # Arguments
/// * `node` - The BlockotNode containing the geometry
/// * `cmd` - The command to execute
pub fn execute_with_undo<C: Command + 'static>(node: &mut BlockotNode, cmd: C) {
    // Execute immediately on the geometry
    cmd.execute(node.geometry_mut());

    godot_warn!(
        "execute_with_undo: Command '{}' executed, but undo registration is incomplete. \
         Use BlockotNode::test_move_vertex() for working undo support.",
        cmd.name()
    );
}

/// Execute a command directly on geometry without undo support.
/// Use this for testing or when undo is not needed.
pub fn execute_without_undo<C: Command>(geo: &mut BlockotGeometry, cmd: &C) {
    cmd.execute(geo);
}

/// Undo a command directly on geometry.
/// Use this for testing or direct undo operations.
pub fn undo_command<C: Command>(geo: &mut BlockotGeometry, cmd: &C) {
    cmd.undo(geo);
}

// =============================================================================
// FUTURE IMPLEMENTATION NOTES
// =============================================================================
//
// Full undo/redo implementation requires storing commands in a way that Godot's
// callable system can access. The pattern would be:
//
// ```ignore
// use std::sync::Mutex;
// use std::collections::HashMap;
//
// static COMMAND_REGISTRY: Lazy<Mutex<HashMap<u64, Box<dyn Command>>>> = ...;
//
// pub fn execute_with_undo(
//     undo_redo: Gd<EditorUndoRedoManager>,
//     node: &mut BlockotNode,
//     cmd: impl Command + 'static,
// ) {
//     let id = next_command_id();
//     COMMAND_REGISTRY.lock().unwrap().insert(id, Box::new(cmd.clone()));
//
//     cmd.execute(node.geometry_mut());
//
//     let action_name = GString::from(cmd.name());
//     undo_redo.create_action(&action_name);
//
//     // Register do/undo callbacks that retrieve command from registry
//     undo_redo.add_do_method(/* callable referencing id */);
//     undo_redo.add_undo_method(/* callable referencing id */);
//
//     undo_redo.commit_action_ex().execute(false).done();
// }
// ```
//
// The working pattern is demonstrated in BlockotNode::test_move_vertex() which uses
// object methods (_apply_vertex_move) as callbacks instead of a command registry.
// =============================================================================
