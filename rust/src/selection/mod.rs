// selection/mod.rs - Selection model (vertex-canonical)
//
// Selection stores which geometric elements are currently selected.
// Uses vertex-canonical model: vertex_indices is the canonical form,
// selected_edges and selected_faces are rendering hints only.
// Pure Rust - no Godot types.
//
// [Source: architecture.md#Decision-3-Vertex-Canonical-Selection]

pub mod hit_test;
pub mod modes;

pub use hit_test::find_closest_vertex;
pub use modes::SelectionMode;

use std::collections::HashSet;

/// Vertex-canonical selection model.
///
/// Transforms always operate on `vertex_indices` — no conversion needed.
/// Mode switching preserves selection (same vertices, different visualization).
/// Rendering hints enable proper edge/face highlighting without losing canonical simplicity.
#[derive(Debug, Clone)]
pub struct Selection {
    /// Current selection mode (Vertex, Edge, Face)
    pub mode: SelectionMode,
    /// Canonical selected vertex indices — used for transforms
    pub vertex_indices: HashSet<usize>,
    /// Rendering hints: which edges are selected (for Edge mode display)
    pub selected_edges: Vec<(usize, usize)>,
    /// Rendering hints: which face indices are selected (for Face mode display)
    pub selected_faces: Vec<usize>,
}

impl Selection {
    /// Create a new empty selection in the given mode.
    pub fn new(mode: SelectionMode) -> Self {
        Self {
            mode,
            vertex_indices: HashSet::new(),
            selected_edges: Vec::new(),
            selected_faces: Vec::new(),
        }
    }

    /// Clear all selected elements.
    pub fn clear(&mut self) {
        self.vertex_indices.clear();
        self.selected_edges.clear();
        self.selected_faces.clear();
    }

    /// Returns true if nothing is selected.
    pub fn is_empty(&self) -> bool {
        self.vertex_indices.is_empty()
    }

    /// Select a single vertex, clearing any previous selection.
    ///
    /// Clears vertex_indices and rendering hints, then inserts the given index.
    pub fn select_vertex(&mut self, index: usize) {
        self.clear();
        self.vertex_indices.insert(index);
    }

    /// Toggle a vertex in the selection (add if absent, remove if present).
    ///
    /// Used for multi-select (Ctrl+click). Does not clear existing selection.
    pub fn toggle_vertex(&mut self, index: usize) {
        if !self.vertex_indices.remove(&index) {
            self.vertex_indices.insert(index);
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new(SelectionMode::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_selection_is_empty() {
        let sel = Selection::new(SelectionMode::Vertex);
        assert!(sel.is_empty());
        assert_eq!(sel.mode, SelectionMode::Vertex);
        assert!(sel.vertex_indices.is_empty());
        assert!(sel.selected_edges.is_empty());
        assert!(sel.selected_faces.is_empty());
    }

    #[test]
    fn test_new_with_different_modes() {
        let vertex_sel = Selection::new(SelectionMode::Vertex);
        assert_eq!(vertex_sel.mode, SelectionMode::Vertex);

        let edge_sel = Selection::new(SelectionMode::Edge);
        assert_eq!(edge_sel.mode, SelectionMode::Edge);

        let face_sel = Selection::new(SelectionMode::Face);
        assert_eq!(face_sel.mode, SelectionMode::Face);
    }

    #[test]
    fn test_clear_empties_all_collections() {
        let mut sel = Selection::new(SelectionMode::Vertex);
        sel.vertex_indices.insert(0);
        sel.vertex_indices.insert(1);
        sel.selected_edges.push((0, 1));
        sel.selected_faces.push(0);

        assert!(!sel.is_empty());

        sel.clear();

        assert!(sel.is_empty());
        assert!(sel.vertex_indices.is_empty());
        assert!(sel.selected_edges.is_empty());
        assert!(sel.selected_faces.is_empty());
    }

    #[test]
    fn test_is_empty_with_vertices() {
        let mut sel = Selection::new(SelectionMode::Vertex);
        assert!(sel.is_empty());

        sel.vertex_indices.insert(3);
        assert!(!sel.is_empty());
    }

    #[test]
    fn test_default_selection() {
        let sel = Selection::default();
        assert!(sel.is_empty());
        assert_eq!(sel.mode, SelectionMode::Vertex);
    }

    #[test]
    fn test_select_vertex_clears_previous() {
        let mut sel = Selection::new(SelectionMode::Vertex);
        sel.vertex_indices.insert(0);
        sel.vertex_indices.insert(1);
        sel.selected_edges.push((0, 1));

        sel.select_vertex(5);

        assert_eq!(sel.vertex_indices.len(), 1);
        assert!(sel.vertex_indices.contains(&5));
        assert!(sel.selected_edges.is_empty());
    }

    #[test]
    fn test_select_vertex_on_empty() {
        let mut sel = Selection::new(SelectionMode::Vertex);

        sel.select_vertex(3);

        assert_eq!(sel.vertex_indices.len(), 1);
        assert!(sel.vertex_indices.contains(&3));
    }

    #[test]
    fn test_toggle_vertex_adds_when_absent() {
        let mut sel = Selection::new(SelectionMode::Vertex);

        sel.toggle_vertex(2);

        assert!(sel.vertex_indices.contains(&2));
        assert_eq!(sel.vertex_indices.len(), 1);
    }

    #[test]
    fn test_toggle_vertex_removes_when_present() {
        let mut sel = Selection::new(SelectionMode::Vertex);
        sel.vertex_indices.insert(2);

        sel.toggle_vertex(2);

        assert!(!sel.vertex_indices.contains(&2));
        assert!(sel.is_empty());
    }

    #[test]
    fn test_toggle_vertex_preserves_others() {
        let mut sel = Selection::new(SelectionMode::Vertex);
        sel.vertex_indices.insert(0);
        sel.vertex_indices.insert(1);

        sel.toggle_vertex(2);

        assert_eq!(sel.vertex_indices.len(), 3);
        assert!(sel.vertex_indices.contains(&0));
        assert!(sel.vertex_indices.contains(&1));
        assert!(sel.vertex_indices.contains(&2));
    }

    #[test]
    fn test_clear_after_select_vertex() {
        let mut sel = Selection::new(SelectionMode::Vertex);
        sel.select_vertex(5);
        assert!(!sel.is_empty());

        sel.clear();
        assert!(sel.is_empty());
    }

    #[test]
    fn test_selection_clone() {
        let mut sel = Selection::new(SelectionMode::Edge);
        sel.vertex_indices.insert(0);
        sel.vertex_indices.insert(1);
        sel.selected_edges.push((0, 1));

        let cloned = sel.clone();
        assert_eq!(cloned.mode, SelectionMode::Edge);
        assert_eq!(cloned.vertex_indices.len(), 2);
        assert_eq!(cloned.selected_edges.len(), 1);
    }
}
