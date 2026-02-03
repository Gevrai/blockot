// tools/commands/move_vertices.rs - MoveVertices command implementation
//
// Moves selected vertices by an offset vector.
// Validates at construction, execute/undo are infallible.

use godot::prelude::Vector3;

use crate::error::BlockotError;
use crate::geometry::BlockotGeometry;
use crate::tools::Command;

/// Command to move vertices by an offset.
#[derive(Debug, Clone)]
pub struct MoveVertices {
    /// Indices of vertices to move
    indices: Vec<usize>,
    /// Offset to apply (for execute) / negate (for undo)
    offset: Vector3,
}

impl MoveVertices {
    /// Create a new MoveVertices command.
    ///
    /// # Errors
    /// Returns `BlockotError::EmptySelection` if indices is empty.
    pub fn new(indices: Vec<usize>, offset: Vector3) -> Result<Self, BlockotError> {
        if indices.is_empty() {
            return Err(BlockotError::EmptySelection);
        }
        Ok(Self { indices, offset })
    }

    /// Returns the indices of vertices this command affects.
    pub fn indices(&self) -> &[usize] {
        &self.indices
    }

    /// Returns the offset applied by this command.
    pub fn offset(&self) -> Vector3 {
        self.offset
    }

    /// Validate that all indices are within bounds of the given geometry.
    /// Call this before execute() if you want to catch invalid indices early.
    ///
    /// # Errors
    /// Returns `BlockotError::InvalidVertexIndex` if any index is out of bounds.
    pub fn validate_indices(&self, geo: &BlockotGeometry) -> Result<(), BlockotError> {
        for &idx in &self.indices {
            if idx >= geo.vertices.len() {
                return Err(BlockotError::InvalidVertexIndex(idx));
            }
        }
        Ok(())
    }
}

impl Command for MoveVertices {
    fn execute(&self, geo: &mut BlockotGeometry) {
        for &idx in &self.indices {
            if idx < geo.vertices.len() {
                geo.vertices[idx] += self.offset;
            }
            // Silently skip out-of-bounds indices to maintain infallibility.
            // Caller should validate indices before command creation.
        }
        geo.dirty = true;
    }

    fn undo(&self, geo: &mut BlockotGeometry) {
        for &idx in &self.indices {
            if idx < geo.vertices.len() {
                geo.vertices[idx] -= self.offset;
            }
            // Silently skip out-of-bounds indices to maintain infallibility.
        }
        geo.dirty = true;
    }

    fn name(&self) -> &'static str {
        "Move Vertices"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::unit_cube;

    #[test]
    fn test_move_vertices_roundtrip() {
        let mut geo = unit_cube();
        let original = geo.clone();

        let cmd = MoveVertices::new(vec![0], Vector3::new(1.0, 0.0, 0.0)).unwrap();

        cmd.execute(&mut geo);
        assert_ne!(geo.vertices[0], original.vertices[0]);
        assert_eq!(
            geo.vertices[0],
            original.vertices[0] + Vector3::new(1.0, 0.0, 0.0)
        );

        cmd.undo(&mut geo);
        assert_eq!(geo.vertices[0], original.vertices[0]);
    }

    #[test]
    fn test_move_vertices_empty_selection() {
        let result = MoveVertices::new(vec![], Vector3::ZERO);
        assert!(matches!(result, Err(BlockotError::EmptySelection)));
    }

    #[test]
    fn test_move_vertices_multiple() {
        let mut geo = unit_cube();
        let original = geo.clone();
        let offset = Vector3::new(0.0, 1.0, 0.0);

        let cmd = MoveVertices::new(vec![0, 1, 2], offset).unwrap();

        cmd.execute(&mut geo);
        assert_eq!(geo.vertices[0], original.vertices[0] + offset);
        assert_eq!(geo.vertices[1], original.vertices[1] + offset);
        assert_eq!(geo.vertices[2], original.vertices[2] + offset);
        // Unaffected vertex
        assert_eq!(geo.vertices[3], original.vertices[3]);

        cmd.undo(&mut geo);
        assert_eq!(geo.vertices[0], original.vertices[0]);
        assert_eq!(geo.vertices[1], original.vertices[1]);
        assert_eq!(geo.vertices[2], original.vertices[2]);
    }

    #[test]
    fn test_move_vertices_sets_dirty_flag() {
        let mut geo = unit_cube();
        geo.dirty = false;

        let cmd = MoveVertices::new(vec![0], Vector3::new(1.0, 0.0, 0.0)).unwrap();

        cmd.execute(&mut geo);
        assert!(geo.dirty);

        geo.dirty = false;
        cmd.undo(&mut geo);
        assert!(geo.dirty);
    }

    #[test]
    fn test_move_vertices_name() {
        let cmd = MoveVertices::new(vec![0], Vector3::ZERO).unwrap();
        assert_eq!(cmd.name(), "Move Vertices");
    }

    #[test]
    fn test_move_vertices_validate_indices() {
        let geo = unit_cube(); // 8 vertices (indices 0-7)

        // Valid indices should pass
        let cmd_valid = MoveVertices::new(vec![0, 1, 7], Vector3::ZERO).unwrap();
        assert!(cmd_valid.validate_indices(&geo).is_ok());

        // Invalid index should fail
        let cmd_invalid = MoveVertices::new(vec![0, 100], Vector3::ZERO).unwrap();
        assert!(matches!(
            cmd_invalid.validate_indices(&geo),
            Err(BlockotError::InvalidVertexIndex(100))
        ));
    }

    #[test]
    fn test_move_vertices_out_of_bounds_is_safe() {
        // Execute/undo should be infallible even with bad indices
        let mut geo = unit_cube();
        let original = geo.clone();

        // Command with out-of-bounds index (100 > 8 vertices)
        let cmd = MoveVertices::new(vec![0, 100], Vector3::new(1.0, 0.0, 0.0)).unwrap();

        // Should not panic, should move only valid index
        cmd.execute(&mut geo);
        assert_eq!(geo.vertices[0], original.vertices[0] + Vector3::new(1.0, 0.0, 0.0));

        // Undo should also be safe
        cmd.undo(&mut geo);
        assert_eq!(geo.vertices[0], original.vertices[0]);
    }
}
