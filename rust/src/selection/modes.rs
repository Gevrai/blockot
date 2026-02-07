// selection/modes.rs - Selection mode enum
//
// Defines the three selection modes available in edit mode.
// Pure Rust - no Godot types.

/// The selection mode determines which geometric elements are selectable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionMode {
    /// Select individual vertices
    #[default]
    Vertex,
    /// Select edges (pairs of vertices)
    Edge,
    /// Select faces
    Face,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_vertex() {
        assert_eq!(SelectionMode::default(), SelectionMode::Vertex);
    }

    #[test]
    fn test_selection_mode_equality() {
        assert_eq!(SelectionMode::Vertex, SelectionMode::Vertex);
        assert_ne!(SelectionMode::Vertex, SelectionMode::Edge);
        assert_ne!(SelectionMode::Edge, SelectionMode::Face);
    }

    #[test]
    fn test_selection_mode_clone() {
        let mode = SelectionMode::Face;
        let cloned = mode;
        assert_eq!(mode, cloned);
    }
}
