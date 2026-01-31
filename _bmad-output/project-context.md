---
project_name: 'blockot'
user_name: 'You'
date: '2026-01-24'
sections_completed: ['technology_stack', 'language_rules', 'framework_rules', 'testing_rules', 'style_rules', 'workflow_rules', 'critical_rules']
status: 'complete'
rule_count: 35
optimized_for_llm: true
---

# Project Context for AI Agents

_Critical rules and patterns for implementing blockot. Focus on unobvious details that prevent implementation mistakes._

---

## Technology Stack & Versions

| Technology | Version | Notes |
|------------|---------|-------|
| **Rust** | stable | Use latest stable toolchain |
| **gdext (godot-rust)** | latest | Check godot-rust/gdext for current version |
| **Godot** | 4.1+ minimum, 4.2+ recommended | GDExtension binary compatibility |
| **cargo** | - | Build, test, lint via cargo commands |

**Compatibility:** gdext tracks Godot releases. Pin gdext version in Cargo.toml to match target Godot version.

---

## Critical Implementation Rules

### Rust/gdext Rules

**Godot Types at Edges Only:**
- `geometry/`, `selection/`, `tools/commands/` use pure Rust types (Vec<Vector3>, not PackedVector3Array)
- `editor/` module is the ONLY place Godot types are allowed
- Boundary conversion happens in `editor/blockot_node.rs` and `geometry/serialization.rs`
- **Why:** Pure Rust core enables testing without Godot runtime, faster iteration, and potential crate extraction

**Gd<T> Pointer Rules:**
- Godot owns nodes, Rust borrows via `Gd<T>`
- **Never store `Gd<T>` across frames** — store `NodePath` or instance IDs instead, retrieve fresh reference when needed
- Use `bind()` / `bind_mut()` for access, release promptly

**Derive Macros:**
```rust
#[derive(GodotClass)]
#[class(base=MeshInstance3D)]
struct BlockotNode { ... }
```

**Signal Naming:** Past tense, snake_case (`edit_mode_entered`, `selection_changed`)

### Command Pattern Rules

**Validation at Construction:**
```rust
impl TransformVertices {
    pub fn new(indices: Vec<usize>, transform: Transform3D) -> Result<Self, BlockotError> {
        if indices.is_empty() { return Err(BlockotError::EmptySelection); }
        Ok(Self { indices, transform, inverse: transform.inverse() })
    }
}
```

**Execute/Undo are Infallible:** Once constructed, commands NEVER fail. All validation happens in constructor.

**Commands NEVER Trigger Cache Rebuild:** Commands mutate `BlockotGeometry` only. Cache rebuild happens externally.

**Cache Rebuild Pattern:**
```rust
// In editor/blockot_node.rs - AFTER command completes
cmd.execute(&mut self.geometry);
if self.geometry.dirty {
    self.rebuild_array_mesh();  // External to command
    self.geometry.dirty = false;
}
```

**Store Inverse Transform:** For rotate/scale, store precomputed inverse (not negated delta) for exact undo.

### Preview State Rules

**Preview is Visual-Only:**
- Source `vertices` unchanged during drag
- `get_vertex_for_render()` applies preview transform
- Command created ONLY on commit

**Preview-Command Equivalence Invariant:**
- `commit_preview()` must produce identical geometry to executing a command with the same parameters directly
- If these ever diverge, it's a bug — not expected behavior

**Cancel on Exit:**
- `cancel_preview()` on Escape, right-click, scene exit, or pre-save notification
- Preview auto-clears, no command created

### Undo/Redo Integration

**Use EditorUndoRedoManager:** Bridge commands to Godot's undo system via `editor/history.rs`. Do NOT maintain internal history.

**Session-Only History:** Command history clears on scene reload. External .tscn edits invalidate history.

---

## Testing Rules

**Test Failure Principle:** If a roundtrip test fails, the command has a bug — no exceptions. Do not adjust the test to match broken behavior.

**Required Tests Per Command:**
1. Success roundtrip: `execute → undo → assert_eq!(geo, original)`
2. Validation rejection: Invalid indices return `Err(BlockotError::...)`
3. Preview equivalence: `commit_preview()` matches direct `execute()` result

**Required Integration Tests:**
- Multi-command undo chains: `execute(cmd1) → execute(cmd2) → undo() → undo() → assert_eq!(geo, original)`
- Located in `tests/undo_redo_chains.rs`

**Test Location:**
- Inline `#[cfg(test)]` in each command file
- Integration tests in `tests/` folder

**Fixtures:** Use `test_utils.rs` for shared geometry builders (`unit_cube()`, `single_face()`, etc.)

**No Godot in Tests:** Core modules test without Godot runtime. Only `editor/` tests need Godot.

---

## Code Quality & Style Rules

**Naming:**
| Element | Convention | Example |
|---------|------------|---------|
| Modules/files | snake_case | `move_vertices.rs` |
| Structs/Enums | PascalCase | `BlockotGeometry` |
| Functions | snake_case | `extrude_faces()` |
| Godot signals | Past tense | `selection_changed` |

**Error Handling:**
```rust
pub enum BlockotError {
    EmptySelection,
    InvalidFaceIndex(usize),
    InvalidVertexIndex(usize),
    // ...
}
```
- Return `Result<T, BlockotError>` for fallible operations
- `editor/` catches errors, logs to Godot console

**Logging:**
- Use `log::debug!()`, `log::warn!()` in core modules
- `editor/logging.rs` routes to `godot_print!` etc.

---

## Development Workflow Rules

**Build:**
```bash
cd rust && cargo build
```

**Lint Before Commit:**
```bash
cargo clippy && cargo test
```

**CI Pipeline:**
1. `cargo clippy` — lint
2. `cargo test` — unit tests
3. Cross-compile (Windows/Linux/macOS)

**Release Artifacts:**
- `libblockot.linux.x86_64.so`
- `blockot.windows.x86_64.dll`
- `libblockot.macos.universal.dylib`

---

## Critical Don't-Miss Rules

### NEVER Do These:
- ❌ Godot types in `geometry/`, `selection/`, `tools/` modules
- ❌ Validate in `execute()` — validate in constructor
- ❌ Rebuild cache inside command execution
- ❌ Modify source vertices during preview/drag
- ❌ Store `Gd<T>` across frames — use NodePath/IDs, retrieve fresh
- ❌ Use negated delta for rotate/scale undo (use inverse transform)
- ❌ Adjust tests to match broken command behavior

### ALWAYS Do These:
- ✅ Implement `Command` trait with `execute()`, `undo()`, `name()`
- ✅ Clone commands for undo registration
- ✅ Cancel preview on scene exit/pre-save
- ✅ Clear command history on scene reload
- ✅ Use `get_vertex_for_render()` for all rendering (includes preview)
- ✅ Write roundtrip test for every command
- ✅ Write multi-command undo chain integration tests
- ✅ Verify preview-command equivalence

### Performance Patterns:
- Vertex position updates: sub-millisecond (fast path)
- Topology changes: action start/commit only (not during drag)
- Selection affects performance, not total mesh size

### Shared Vertex Behavior:
- Shared vertices move together when any connected face/edge transformed
- No auto-vertex-split in MVP — this is documented behavior, not a bug

---

## Usage Guidelines

**For AI Agents:**
- Read this file before implementing any code
- Follow ALL rules exactly as documented
- When in doubt, prefer the more restrictive option
- Cross-reference with `architecture.md` for structural decisions

**For Humans:**
- Keep this file lean and focused on agent needs
- Update when technology stack or patterns change
- Review quarterly for outdated rules
- Remove rules that become obvious over time

---

_Last Updated: 2026-01-24_
