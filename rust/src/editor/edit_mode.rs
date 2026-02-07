// editor/edit_mode.rs - Edit mode state machine
//
// Centralized state tracking for edit mode. Uses InstanceId (i64)
// to track which BlockotNode is currently being edited.
// Only one BlockotNode can be in edit mode at a time (FR8).
//
// [Source: architecture.md#editor/edit_mode.rs]

use crate::selection::SelectionMode;

/// The current state of edit mode.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum EditModeState {
    /// No node is being edited.
    #[default]
    Inactive,
    /// A specific BlockotNode is being edited.
    Active {
        /// InstanceId of the BlockotNode being edited (i64 from Godot).
        node_instance_id: i64,
        /// Current selection mode (Vertex, Edge, Face).
        selection_mode: SelectionMode,
    },
}

impl EditModeState {
    /// Returns true if edit mode is active (any node being edited).
    pub fn is_active(&self) -> bool {
        matches!(self, EditModeState::Active { .. })
    }

    /// Returns the instance ID of the node being edited, if any.
    pub fn active_node_id(&self) -> Option<i64> {
        match self {
            EditModeState::Active { node_instance_id, .. } => Some(*node_instance_id),
            EditModeState::Inactive => None,
        }
    }

    /// Returns the current selection mode, if in edit mode.
    pub fn selection_mode(&self) -> Option<SelectionMode> {
        match self {
            EditModeState::Active { selection_mode, .. } => Some(*selection_mode),
            EditModeState::Inactive => None,
        }
    }

    /// Enter edit mode on a specific node. Default selection mode is Vertex.
    pub fn enter_edit_mode(&mut self, node_id: i64) {
        *self = EditModeState::Active {
            node_instance_id: node_id,
            selection_mode: SelectionMode::Vertex,
        };
    }

    /// Exit edit mode, returning to Inactive.
    pub fn exit_edit_mode(&mut self) {
        *self = EditModeState::Inactive;
    }

    /// Toggle edit mode for a given node.
    /// - If inactive: enters edit mode on the node.
    /// - If active on the same node: exits edit mode.
    /// - If active on a different node: switches to the new node (FR8).
    ///
    /// Returns the previous state for callers that need to notify the old node.
    pub fn toggle_for_node(&mut self, node_id: i64) -> EditModeState {
        let previous = self.clone();
        match self {
            EditModeState::Inactive => {
                self.enter_edit_mode(node_id);
            }
            EditModeState::Active { node_instance_id, .. } => {
                if *node_instance_id == node_id {
                    self.exit_edit_mode();
                } else {
                    // Switch to new node (FR8: single-node editing)
                    self.enter_edit_mode(node_id);
                }
            }
        }
        previous
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_inactive() {
        let state = EditModeState::default();
        assert_eq!(state, EditModeState::Inactive);
        assert!(!state.is_active());
        assert_eq!(state.active_node_id(), None);
        assert_eq!(state.selection_mode(), None);
    }

    #[test]
    fn test_enter_edit_mode() {
        let mut state = EditModeState::default();
        state.enter_edit_mode(42);

        assert!(state.is_active());
        assert_eq!(state.active_node_id(), Some(42));
        assert_eq!(state.selection_mode(), Some(SelectionMode::Vertex));
    }

    #[test]
    fn test_exit_edit_mode() {
        let mut state = EditModeState::default();
        state.enter_edit_mode(42);
        state.exit_edit_mode();

        assert!(!state.is_active());
        assert_eq!(state.active_node_id(), None);
    }

    #[test]
    fn test_toggle_from_inactive_enters() {
        let mut state = EditModeState::default();
        let prev = state.toggle_for_node(42);

        assert_eq!(prev, EditModeState::Inactive);
        assert!(state.is_active());
        assert_eq!(state.active_node_id(), Some(42));
    }

    #[test]
    fn test_toggle_same_node_exits() {
        let mut state = EditModeState::default();
        state.enter_edit_mode(42);
        let prev = state.toggle_for_node(42);

        assert!(prev.is_active());
        assert!(!state.is_active());
    }

    #[test]
    fn test_toggle_different_node_switches() {
        let mut state = EditModeState::default();
        state.enter_edit_mode(42);
        let prev = state.toggle_for_node(99);

        // Previous was active on 42
        assert_eq!(prev.active_node_id(), Some(42));
        // Now active on 99
        assert!(state.is_active());
        assert_eq!(state.active_node_id(), Some(99));
        assert_eq!(state.selection_mode(), Some(SelectionMode::Vertex));
    }

    #[test]
    fn test_enter_resets_selection_mode_to_vertex() {
        let mut state = EditModeState::Active {
            node_instance_id: 42,
            selection_mode: SelectionMode::Face,
        };
        // Switching to a new node should reset to Vertex mode
        state.enter_edit_mode(99);
        assert_eq!(state.selection_mode(), Some(SelectionMode::Vertex));
    }

    #[test]
    fn test_toggle_returns_previous_state() {
        let mut state = EditModeState::default();

        // Toggle on: previous should be Inactive
        let prev = state.toggle_for_node(10);
        assert_eq!(prev, EditModeState::Inactive);

        // Toggle off: previous should be Active(10)
        let prev = state.toggle_for_node(10);
        assert_eq!(prev.active_node_id(), Some(10));

        // State is now Inactive
        assert!(!state.is_active());
    }
}
