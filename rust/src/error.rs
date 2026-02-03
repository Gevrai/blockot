// error.rs - BlockotError enum for error handling
//
// All fallible operations return Result<T, BlockotError>.
// Commands validate at construction and return these errors.

use std::error::Error;
use std::fmt;

/// Errors that can occur in blockot operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockotError {
    /// Operation requires a selection but none was provided
    EmptySelection,

    /// Vertex index is out of bounds
    InvalidVertexIndex(usize),

    /// Face index is out of bounds
    InvalidFaceIndex(usize),
}

impl fmt::Display for BlockotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockotError::EmptySelection => {
                write!(f, "Operation requires a selection but none was provided")
            }
            BlockotError::InvalidVertexIndex(idx) => {
                write!(f, "Invalid vertex index: {}", idx)
            }
            BlockotError::InvalidFaceIndex(idx) => {
                write!(f, "Invalid face index: {}", idx)
            }
        }
    }
}

impl Error for BlockotError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            BlockotError::EmptySelection.to_string(),
            "Operation requires a selection but none was provided"
        );
        assert_eq!(
            BlockotError::InvalidVertexIndex(5).to_string(),
            "Invalid vertex index: 5"
        );
        assert_eq!(
            BlockotError::InvalidFaceIndex(10).to_string(),
            "Invalid face index: 10"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(BlockotError::EmptySelection, BlockotError::EmptySelection);
        assert_eq!(
            BlockotError::InvalidVertexIndex(3),
            BlockotError::InvalidVertexIndex(3)
        );
        assert_ne!(
            BlockotError::InvalidVertexIndex(3),
            BlockotError::InvalidVertexIndex(4)
        );
    }
}
