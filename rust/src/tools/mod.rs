// tools/mod.rs - Command trait and tool infrastructure
//
// CRITICAL: This module is PURE RUST - no Godot types except Vector3 (math type).
// Commands validate at construction, execute/undo are infallible.

pub mod commands;

use crate::geometry::BlockotGeometry;

/// Trait for reversible geometry commands.
///
/// # Rules
/// 1. Validate at construction (`new()` returns `Result<Self, BlockotError>`)
/// 2. `execute()` and `undo()` are INFALLIBLE - all validation happens in constructor
/// 3. Commands NEVER trigger cache rebuild - they only mutate `BlockotGeometry`
/// 4. Store inverse transform for exact undo (not negated delta)
pub trait Command: Clone + Send + Sync {
    /// Execute the command on the geometry.
    /// This is infallible - validation happens at construction.
    fn execute(&self, geo: &mut BlockotGeometry);

    /// Undo the command on the geometry.
    /// This is infallible and must exactly reverse `execute()`.
    fn undo(&self, geo: &mut BlockotGeometry);

    /// Returns a human-readable name for this command (for undo menu).
    fn name(&self) -> &'static str;
}
