// editor/plugin.rs - BlockotPlugin EditorPlugin
//
// Handles input forwarding and edit mode toggling for BlockotNode.
// Captures Tab key to enter/exit edit mode.
//
// Tab detection uses process() polling with the Input singleton because
// the Godot editor's GUI focus system intercepts Tab for focus navigation
// before _input() or _forward_3d_gui_input() can consume it.
//
// [Source: architecture.md#EditorPlugin-trait]

use godot::classes::editor_plugin::AfterGuiInput;
use godot::classes::{
    Camera3D, EditorInterface, EditorPlugin, IEditorPlugin, Input, InputEvent, Object,
};
use godot::global::Key;
use godot::obj::EngineEnum;
use godot::prelude::*;

use super::blockot_node::BlockotNode;
use super::edit_mode::EditModeState;

/// Editor plugin that provides edit mode for BlockotNode.
///
/// Handles Tab key input to toggle edit mode on/off.
/// Only one BlockotNode can be in edit mode at a time (FR8).
#[derive(GodotClass)]
#[class(tool, init, base=EditorPlugin)]
pub struct BlockotPlugin {
    base: Base<EditorPlugin>,
    edit_state: EditModeState,
    /// Edge detection for Tab key polling
    tab_was_pressed: bool,
}

#[godot_api]
impl IEditorPlugin for BlockotPlugin {
    fn handles(&self, object: Gd<Object>) -> bool {
        object.is_class("BlockotNode")
    }

    fn edit(&mut self, _object: Option<Gd<Object>>) {
        // Called when the editor selects/deselects a BlockotNode.
        // We handle mode changes via Tab key instead of automatic entry.
    }

    fn make_visible(&mut self, _visible: bool) {
        // Called when the plugin's edited object becomes visible/hidden.
        // If edit mode was active and the object is hidden, exit edit mode.
        if !_visible && self.edit_state.is_active() {
            self.do_exit_edit_mode();
        }
    }

    fn process(&mut self, _delta: f64) {
        // Poll Tab key state via Input singleton for reliable edge detection.
        // This bypasses the editor's GUI focus system which intercepts Tab
        // events before they reach _input() or _forward_3d_gui_input().
        let input = Input::singleton();
        let tab_pressed = input.is_key_pressed(Key::TAB);

        if tab_pressed && !self.tab_was_pressed {
            // Tab just pressed this frame
            if self.get_selected_blockot_node_id().is_some() || self.edit_state.is_active() {
                self.handle_tab_press();
            }
        }

        self.tab_was_pressed = tab_pressed;
    }

    fn forward_3d_gui_input(
        &mut self,
        _viewport_camera: Option<Gd<Camera3D>>,
        _event: Option<Gd<InputEvent>>,
    ) -> i32 {
        // Tab handling is in process(). This method remains for future
        // mouse-based interactions (vertex clicking, etc. in Story 2.2+).
        AfterGuiInput::PASS.ord()
    }
}

#[godot_api]
impl BlockotPlugin {}

impl BlockotPlugin {
    /// Handle Tab key press: toggle edit mode for the currently selected node.
    fn handle_tab_press(&mut self) {
        // Get the currently selected BlockotNode
        let Some(node_id) = self.get_selected_blockot_node_id() else {
            // No BlockotNode selected — if we're in edit mode, exit it
            if self.edit_state.is_active() {
                self.do_exit_edit_mode();
            }
            return;
        };

        let previous = self.edit_state.toggle_for_node(node_id);

        match &self.edit_state {
            EditModeState::Active {
                node_instance_id, ..
            } => {
                // Switched from a different node — exit the old one first
                if let Some(prev_id) = previous.active_node_id() {
                    if prev_id != *node_instance_id {
                        self.notify_node_exit_edit_mode(prev_id);
                    }
                }
                self.notify_node_enter_edit_mode(*node_instance_id);
                godot_print!(
                    "BlockotPlugin: Entered edit mode on node {}",
                    node_instance_id
                );
            }
            EditModeState::Inactive => {
                // Exited edit mode — notify the previously active node
                if let Some(prev_id) = previous.active_node_id() {
                    self.notify_node_exit_edit_mode(prev_id);
                }
                godot_print!("BlockotPlugin: Exited edit mode");
            }
        }
    }

    /// Exit edit mode and notify the active node.
    fn do_exit_edit_mode(&mut self) {
        if let Some(node_id) = self.edit_state.active_node_id() {
            self.edit_state.exit_edit_mode();
            self.notify_node_exit_edit_mode(node_id);
            godot_print!("BlockotPlugin: Exited edit mode (visibility change)");
        }
    }

    /// Get the InstanceId (as i64) of the currently selected BlockotNode, if any.
    fn get_selected_blockot_node_id(&self) -> Option<i64> {
        let editor = EditorInterface::singleton();
        let mut selection = editor.get_selection()?;
        let nodes = selection.get_selected_nodes();

        // Find the first selected BlockotNode
        for i in 0..nodes.len() {
            if let Some(node) = nodes.get(i) {
                if node.is_class("BlockotNode") {
                    return Some(node.instance_id().to_i64());
                }
            }
        }
        None
    }

    /// Notify a BlockotNode that it should enter edit mode.
    fn notify_node_enter_edit_mode(&self, node_id: i64) {
        if let Some(instance_id) = InstanceId::try_from_i64(node_id) {
            if let Ok(obj) = Gd::<Object>::try_from_instance_id(instance_id) {
                if let Ok(mut node) = obj.try_cast::<BlockotNode>() {
                    node.bind_mut().enter_edit_mode();
                }
            }
        }
    }

    /// Notify a BlockotNode that it should exit edit mode.
    fn notify_node_exit_edit_mode(&self, node_id: i64) {
        if let Some(instance_id) = InstanceId::try_from_i64(node_id) {
            if let Ok(obj) = Gd::<Object>::try_from_instance_id(instance_id) {
                if let Ok(mut node) = obj.try_cast::<BlockotNode>() {
                    node.bind_mut().exit_edit_mode();
                }
            }
        }
    }
}
