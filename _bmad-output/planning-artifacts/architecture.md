---
stepsCompleted: [1, 2, 3, 4, 5, 6, 7, 8]
inputDocuments:
  - _bmad-output/planning-artifacts/prd.md
  - _bmad-output/planning-artifacts/product-brief-blockot-2026-01-20.md
  - _bmad-output/planning-artifacts/ux-design-specification.md
workflowType: 'architecture'
lastStep: 8
status: 'complete'
completedAt: '2026-01-24'
project_name: 'blockot'
user_name: 'You'
date: '2026-01-24'
---

# Architecture Decision Document

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

## Project Context Analysis

### Requirements Overview

**Functional Requirements:**

33 FRs across 8 categories define a focused geometry editing tool:

| Category | FRs | Architectural Implication |
|----------|-----|---------------------------|
| Geometry Creation | FR1-5 | Primitive generation, geometry data model |
| Edit Mode | FR6-8 | Modal state machine, single-node focus |
| Selection | FR9-14 | Selection state per mode, multi-select container |
| Transform Tools | FR15-19 | **Action abstraction layer**, confirm/cancel workflow |
| Geometry Ops | FR20-21 | Topology modification (extrude changes vertex count) |
| Snapping | FR22-25 | Snap engine with multiple strategies |
| Properties | FR26-29 | Face normals, collision type (convex/concave) |
| Editor Integration | FR30-33 | Godot undo/redo, scene serialization |

**Non-Functional Requirements:**

| NFR Category | Constraints | Architectural Impact |
|--------------|-------------|---------------------|
| Performance | <100ms transform, <200ms mode switch | Optimize for *affected faces*, not total mesh size |
| Reliability | Undo/redo correctness, save/load fidelity | Immutable operation history, robust serialization |
| Integration | Godot conventions, native undo, Inspector | EditorPlugin patterns, UndoRedo API |

**Scale & Complexity:**

- Primary domain: Godot EditorPlugin (3D geometry editing)
- Complexity level: Low-Medium
- Estimated architectural components: 6-8 core classes

### Foundational Architectural Requirement

**Undo/Redo Integration** is not merely cross-cutting — it is *foundational*. Every command that modifies geometry or selection state MUST integrate with Godot's UndoRedo system. No exceptions. This requirement influences:
- Data model design (must support state capture/restore)
- Command architecture (all operations as undoable actions)
- Testing strategy (undo chains are high-risk, need coverage)

### Technical Constraints & Dependencies

| Constraint | Source | Impact |
|------------|--------|--------|
| Godot 4.0+ | PRD | GDScript 2.0, EditorPlugin API |
| EditorPlugin architecture | Godot | Must extend EditorPlugin, use editor APIs |
| Addon conventions | NFR9 | `addons/blockot/plugin.cfg` structure |
| Native undo system | NFR10 | UndoRedo API — foundational, not optional |
| Collision options | Discussion | Support convex (fast) and concave (accurate) |

### Architectural Principles (from Discussion)

1. **Action Abstraction** — Input (keys, buttons) binds to named actions; actions call functions. Enables future rebinding and scripting API.
2. **Selection-Based Operations** — Performance scales with *affected geometry*, not total mesh size. Data structures must support efficient selection-based transforms.
3. **Testable Architecture** — Design for testability without over-testing. Test action functions directly; input binding tested separately.
4. **Deferred Collision Rebuild** — Collision mesh regenerates on edit mode exit or explicit request, not on every operation. User chooses convex vs concave.

### Cross-Cutting Concerns

1. **Undo/Redo** — Foundational; every geometry and selection operation
2. **State Management** — Edit mode, selection mode, active tool, current selection
3. **Input→Action Mapping** — Decoupled for rebindability and testability
4. **Serialization** — Geometry data persists correctly in .tscn files
5. **Collision Sync** — Deferred rebuild with convex/concave option

## Technology Foundation

### Language & Runtime Decision

**Selected: Rust with gdext (godot-rust)**

**Decision Rationale:**

| Factor | Assessment |
|--------|------------|
| **Performance** | Native code, zero-cost abstractions — meets NFR1-4 |
| **Memory Safety** | Ownership model prevents geometry/state bugs |
| **Undo/Redo Fit** | Immutable patterns natural for state snapshots; compiler prevents use-after-free on state restoration |
| **Testability** | `cargo test` built-in, first-class citizen |
| **Refactoring Safety** | Compiler catches breakage during iteration |
| **Learning** | User motivated; harsh compiler = good teacher |
| **Distribution** | Precompiled binaries, users just copy addon folder |

**Trade-offs Accepted:**
- Steeper initial learning curve (borrow checker)
- Community bindings (not official Godot team)
- Fewer examples/Stack Overflow answers than C++
- Potential for gdext breaking changes

**Mitigation:** Architecture transfers to C++ if needed — not a one-way door.

### Key Rust/gdext Patterns to Learn Early

- **`Gd<T>` smart pointers** — Godot owns nodes, Rust borrows via `Gd<>`
- **`#[derive(GodotClass)]`** — Macro for exposing Rust structs to Godot
- **`EditorPlugin` trait** — Entry point for editor integration

### Godot Version Target

| Target | Version |
|--------|---------|
| **Minimum** | Godot 4.1 (gdext binary compatibility floor) |
| **Recommended** | Godot 4.2+ (improved GDExtension APIs) |

### Project Structure

```
blockot/
├── godot/                    # Godot project for testing
│   ├── project.godot
│   ├── addons/
│   │   └── blockot/
│   │       ├── plugin.cfg
│   │       ├── blockot.gdextension
│   │       └── bin/          # Compiled libraries
│   └── test_scenes/
├── rust/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs            # Entry point, registers extension
│   │   ├── geometry/         # Mesh data, primitives (pure Rust at core)
│   │   ├── selection/        # Selection state, modes
│   │   ├── tools/            # Transform, extrude, cut actions
│   │   ├── input/            # Action mapping (simple match + lookup)
│   │   └── editor/           # EditorPlugin, Godot integration
│   └── tests/                # Rust unit tests
├── .github/
│   └── workflows/
│       └── build.yml         # Cross-platform CI
└── README.md
```

**Structure Notes:**
- `geometry/` designed for potential extraction — keep Godot types at boundaries
- Start as one crate, extract `blockot-core` later if needed
- `input/` stays simple — match statement + lookup table, not over-engineered

### Build & Distribution

**Development:**
```bash
cd rust && cargo build
```

**CI Pipeline (MVP scope):**
1. `cargo clippy` — lint
2. `cargo test` — unit tests
3. Cross-compile Windows/Linux/macOS — build only, no runtime tests

**Release Artifacts:**
- `addons/blockot/bin/libblockot.linux.x86_64.so`
- `addons/blockot/bin/blockot.windows.x86_64.dll`
- `addons/blockot/bin/libblockot.macos.universal.dylib`

**User Installation:**
1. Download release zip
2. Copy `addons/blockot/` to project
3. Enable plugin in Project Settings — Done

## Core Architectural Decisions

### Decision Summary

| # | Decision | Choice |
|---|----------|--------|
| 1 | **Node Type** | BlockotNode (custom node extending MeshInstance3D) |
| 2 | **Geometry Representation** | Hybrid — Rust source of truth, cached ArrayMesh |
| 3 | **Selection Model** | Vertex-Canonical with rendering hints |
| 4 | **Undo/Redo** | Command Pattern with inverse transforms |
| 5 | **Serialization** | Flat arrays inline, Resource extraction path for future |

---

### Decision 1: BlockotNode

**Choice:** Custom node extending MeshInstance3D

**Rationale:**
- Clear mental model: "Add a BlockotNode"
- Appears in Add Node dialog
- Properties directly visible in Inspector
- One node = one editable geometry

---

### Decision 2: Hybrid Geometry Representation

**Choice:** Pure Rust data structures as source of truth, cached ArrayMesh for rendering

**Architecture:**
```rust
struct BlockotGeometry {
    // Source of truth (pure Rust, testable)
    vertices: Vec<Vector3>,
    faces: Vec<Face>,

    // Render cache (rebuilt when dirty)
    cached_mesh: Option<Gd<ArrayMesh>>,
    dirty: bool,
}
```

**Rules:**
- Rust data is single source of truth
- ArrayMesh is a cached "view" for rendering
- Transform drags: fast vertex position updates (no topology rebuild)
- Topology changes: full rebuild on action start/commit only

**Performance target:** Sub-millisecond vertex updates for 500+ vertices

---

### Decision 3: Vertex-Canonical Selection with Rendering Hints

**Choice:** All selection modes reduce to vertex indices, with hints for rendering

**Architecture:**
```rust
struct Selection {
    mode: SelectionMode,              // Vertex, Edge, Face
    vertex_indices: HashSet<usize>,   // Canonical — used for transforms

    // Rendering hints (for highlight display only)
    selected_edges: Vec<(usize, usize)>,  // Edge mode: which edges
    selected_faces: Vec<usize>,           // Face mode: which face indices
}
```

**Rationale:**
- Transforms always operate on vertices — no conversion needed
- Mode switching preserves selection (same vertices, different visualization)
- Rendering hints enable proper edge/face highlighting without losing canonical simplicity

**Documented Behavior:**
- Shared vertices move together when any connected face/edge is transformed
- No auto-vertex-split in MVP (deferred feature)

---

### Decision 4: Command Pattern with Inverse Transforms

**Choice:** Every operation is a deterministic, invertible Command

**Architecture:**
```rust
trait Command: Clone {
    fn execute(&self, geo: &mut BlockotGeometry);
    fn undo(&self, geo: &mut BlockotGeometry);
}

struct TransformVertices {
    indices: Vec<usize>,
    transform: Transform3D,
    inverse: Transform3D,  // Precomputed for exact undo
}

struct ExtrudeFaces {
    face_indices: Vec<usize>,
    extrude_offset: Vector3,
    // Created elements (for undo removal)
    created_vertices: Vec<usize>,
    created_faces: Vec<usize>,
}
```

**Rules:**
- Commands ONLY mutate BlockotGeometry Rust data
- Commands NEVER trigger cache rebuild
- Cache rebuild happens AFTER command completes, outside undo/redo scope
- Rotate/scale store inverse transform (not negated delta) for exact undo
- All commands must be deterministic and testable in isolation

**Testing pattern:**
```rust
#[test]
fn test_command_roundtrip() {
    let mut geo = BlockotGeometry::unit_cube();
    let original = geo.clone();

    cmd.execute(&mut geo);
    cmd.undo(&mut geo);

    assert_eq!(geo, original);  // Exact restoration
}
```

---

### Decision 5: Flat Array Serialization with Resource Path

**Choice:** Flat arrays inline on node (MVP), extractable to Resource (future)

**MVP Format:**
```rust
#[derive(GodotClass)]
#[class(base=MeshInstance3D)]
struct BlockotNode {
    #[export] vertices: PackedVector3Array,
    #[export] face_vertex_counts: PackedInt32Array,
    #[export] face_indices: PackedInt32Array,
    #[export] face_direction: FaceDirection,
    #[export] collision_type: CollisionType,
}
```

**Future Resource:**
```rust
#[derive(GodotClass)]
#[class(base=Resource)]
struct BlockotMeshData {
    #[export] vertices: PackedVector3Array,
    #[export] face_vertex_counts: PackedInt32Array,
    #[export] face_indices: PackedInt32Array,
}
```

**Benefits:**
- Godot-native serialization (no custom format)
- Git-diffable .tscn files
- Future: "Save as Resource" for reusable geometry

---

### Architecture Rules

1. **Source of Truth:** Rust `BlockotGeometry` — never derive truth from ArrayMesh cache
2. **Cache Isolation:** Commands never trigger rebuild; rebuild is external to undo/redo
3. **Deterministic Commands:** Every operation must produce identical results given identical inputs
4. **Exact Undo:** Use inverse transforms, not negated deltas, for rotate/scale
5. **Shared Vertex Behavior:** Documented — no auto-split, shared vertices move together

## Implementation Patterns & Consistency Rules

### Naming Patterns

| Element | Convention | Example |
|---------|------------|---------|
| **Rust modules/files** | `snake_case` | `move_vertices.rs` |
| **Structs/Enums** | `PascalCase` | `BlockotGeometry`, `SelectionMode` |
| **Functions** | `snake_case` | `extrude_faces()` |
| **Godot signals** | Past tense, snake_case | `edit_mode_entered`, `selection_changed` |
| **Godot methods** | `snake_case` (matches Rust) | `enter_edit_mode()` |

### Module Organization

```
src/
├── lib.rs
├── test_utils.rs           # Test fixtures (cfg(test))
├── error.rs                # BlockotError enum
├── geometry/
│   ├── mod.rs              # Public API
│   ├── mesh.rs             # Pure Rust
│   ├── primitives.rs       # Pure Rust
│   ├── face.rs             # Pure Rust
│   └── preview.rs          # PreviewState handling
├── selection/
│   ├── mod.rs
│   └── modes.rs            # Pure Rust
├── tools/
│   ├── mod.rs              # Command trait defined here
│   └── commands/
│       ├── mod.rs          # Re-exports all commands
│       ├── move_vertices.rs
│       ├── rotate_vertices.rs
│       ├── scale_vertices.rs
│       ├── extrude_faces.rs
│       └── cut_edge.rs
└── editor/
    ├── mod.rs
    ├── plugin.rs           # EditorPlugin
    ├── blockot_node.rs     # Node impl
    ├── input_handler.rs    # Drag/preview coordination
    └── logging.rs          # Godot logger backend
```

**Rule:** Godot types only in `editor/` and public `mod.rs` boundaries.

### Command Trait

**Location:** `tools/mod.rs`

```rust
pub trait Command: Clone + Send + Sync {
    fn execute(&self, geo: &mut BlockotGeometry);
    fn undo(&self, geo: &mut BlockotGeometry);
    fn name(&self) -> &'static str;
}
```

**Validation:** At construction, not execute. Commands are infallible once created.

### Preview State Pattern

**Problem:** During drag, visuals update but command isn't committed yet.

**Solution:** Preview layer separate from committed source.

```rust
struct PreviewState {
    affected_indices: Vec<usize>,
    transform: Transform3D,
}

struct BlockotGeometry {
    // Committed source of truth
    vertices: Vec<Vector3>,
    faces: Vec<Face>,

    // Preview state (visual only, not committed)
    preview: Option<PreviewState>,

    // Render cache
    cached_mesh: Option<Gd<ArrayMesh>>,
    dirty: bool,
}

impl BlockotGeometry {
    /// Get vertex for rendering (applies preview if active)
    pub fn get_vertex_for_render(&self, idx: usize) -> Vector3 {
        let base = self.vertices[idx];
        if let Some(preview) = &self.preview {
            if preview.affected_indices.contains(&idx) {
                return preview.transform * base;
            }
        }
        base
    }

    /// Start preview (user presses G/R/S)
    pub fn begin_preview(&mut self, indices: Vec<usize>) {
        self.preview = Some(PreviewState {
            affected_indices: indices,
            transform: Transform3D::IDENTITY,
        });
    }

    /// Update preview (during drag)
    pub fn update_preview(&mut self, transform: Transform3D) {
        if let Some(preview) = &mut self.preview {
            preview.transform = transform;
            self.dirty = true;
        }
    }

    /// Commit preview (click to confirm) → returns Command for undo stack
    pub fn commit_preview(&mut self) -> Option<Box<dyn Command>> {
        let preview = self.preview.take()?;
        let cmd = TransformVertices::new_unchecked(
            preview.affected_indices,
            preview.transform,
        );
        cmd.execute(self);
        self.dirty = true;
        Some(Box::new(cmd))
    }

    /// Cancel preview (Escape/right-click)
    pub fn cancel_preview(&mut self) {
        self.preview = None;
        self.dirty = true;
    }
}
```

**Flow:**
1. User presses G → `begin_preview(selected_indices)`
2. User drags → `update_preview(current_transform)` each frame
3. User clicks → `commit_preview()` → Command returned for undo stack
4. User Esc/right-click → `cancel_preview()` → visual restores, no command

**Rules:**
- Preview is visual-only — source vertices unchanged until commit
- Commands created at commit time only
- Preview auto-cancelled on: Escape, right-click, scene exit, pre-save
- Render always uses `get_vertex_for_render()` to include preview

### External Modification Pattern

**Policy:** Command history is session-only.

**Rationale:** If user edits .tscn directly, command indices/state become invalid. Simple solution: clear history on reload.

```rust
impl BlockotNode {
    fn _ready(&mut self) {
        self.load_geometry_from_export();
        self.command_history.clear();
        log::debug!("Geometry loaded, command history reset");
    }

    fn _exit_tree(&mut self) {
        self.geometry.cancel_preview();
    }

    fn _notification(&mut self, what: i32) {
        if what == NOTIFICATION_EDITOR_PRE_SAVE {
            self.geometry.cancel_preview();
        }
    }
}
```

**User expectation:** Undo works during editing session. Scene reload clears history.

### Error Handling

```rust
#[derive(Debug, Clone)]
pub enum BlockotError {
    EmptySelection,
    InvalidFaceIndex(usize),
    InvalidVertexIndex(usize),
    InvalidEdge(usize, usize),
    TopologyError(String),
    SerializationError(String),
}
```

**Rules:**
- Command construction returns `Result<Self, BlockotError>`
- `execute`/`undo` are infallible
- `editor/` catches errors, reports to Godot console

### Test Organization

| Type | Location | Purpose |
|------|----------|---------|
| Unit tests | Inline `#[cfg(test)]` | Single command/function |
| Integration | `tests/` folder | Multi-component |
| Fixtures | `src/test_utils.rs` | Shared geometry builders |

**Required tests per command:**
- Success roundtrip (execute → undo → verify original)
- Validation rejection (invalid indices)

### Logging

**Pattern:** `log` crate interface, Godot backend injected

```rust
// Core modules
log::debug!("Starting extrude");
log::warn!("Selection empty");

// editor/logging.rs routes to godot_print!/godot_warn!/godot_error!
```

### Enforcement Summary

**All code MUST:**
1. Keep Godot types at module edges
2. Validate commands at construction
3. Use preview layer for drag operations — never modify source until commit
4. Clear command history on scene reload
5. Cancel preview on scene exit/pre-save
6. Return `Result<T, BlockotError>` for fallible operations
7. Include inline tests for each command
8. Use `log::*` macros in non-editor modules
9. Use centralized test fixtures

## Project Structure & Boundaries

### Complete Directory Structure

```
blockot/
├── README.md
├── LICENSE
├── .gitignore
├── .github/
│   └── workflows/
│       ├── build.yml              # Cross-platform build (Win/Mac/Linux)
│       ├── test.yml               # cargo clippy + cargo test
│       └── release.yml            # Build + package for release
│
├── godot/                          # Godot test project
│   ├── project.godot
│   ├── addons/
│   │   └── blockot/
│   │       ├── plugin.cfg          # Plugin metadata
│   │       ├── blockot.gdextension # GDExtension config
│   │       └── bin/                # Compiled libraries (git-ignored)
│   │           ├── libblockot.linux.x86_64.so
│   │           ├── blockot.windows.x86_64.dll
│   │           └── libblockot.macos.universal.dylib
│   └── test_scenes/
│       ├── basic_cube.tscn         # Single BlockotNode
│       ├── multiple_nodes.tscn     # Several BlockotNodes
│       └── complex_geometry.tscn   # Stress test scene
│
├── rust/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── rustfmt.toml                # Formatting config
│   ├── clippy.toml                 # Linting config
│   │
│   ├── src/
│   │   ├── lib.rs                  # Extension entry point, registers classes
│   │   ├── error.rs                # BlockotError enum
│   │   ├── test_utils.rs           # Test fixtures (pub mod, not cfg(test))
│   │   │
│   │   ├── geometry/               # FR1-5, FR26-29 (Geometry, Properties)
│   │   │   ├── mod.rs              # Public API, re-exports
│   │   │   ├── mesh.rs             # BlockotGeometry struct
│   │   │   ├── face.rs             # Face struct, n-gon support
│   │   │   ├── primitives.rs       # Box, Plane, Cylinder, Sphere generators
│   │   │   ├── preview.rs          # PreviewState for drag operations
│   │   │   ├── properties.rs       # FaceDirection, CollisionType enums
│   │   │   └── serialization.rs    # Pure to/from PackedArrays functions
│   │   │
│   │   ├── selection/              # FR9-14 (Selection)
│   │   │   ├── mod.rs              # Selection struct, public API
│   │   │   └── modes.rs            # SelectionMode enum (Vertex/Edge/Face)
│   │   │
│   │   ├── tools/                  # FR15-21 (Transform, Geometry Ops)
│   │   │   ├── mod.rs              # Command trait definition
│   │   │   └── commands/
│   │   │       ├── mod.rs          # Re-exports all commands
│   │   │       ├── move_vertices.rs      # FR15 + inline tests
│   │   │       ├── rotate_vertices.rs    # FR16 + inline tests
│   │   │       ├── scale_vertices.rs     # FR17 + inline tests
│   │   │       ├── extrude_faces.rs      # FR20 + inline tests
│   │   │       ├── cut_edge.rs           # FR21 + inline tests
│   │   │       ├── select_vertices.rs    # Selection commands
│   │   │       ├── deselect_all.rs
│   │   │       └── delete_faces.rs       # Future
│   │   │
│   │   └── editor/                 # FR6-8, FR22-25, FR30-33 (Godot integration)
│   │       ├── mod.rs              # Module exports
│   │       ├── plugin.rs           # EditorPlugin implementation
│   │       ├── blockot_node.rs     # BlockotNode class, calls serialization
│   │       ├── edit_mode.rs        # Edit mode state machine
│   │       ├── input_handler.rs    # Key bindings, drag handling
│   │       ├── snapping.rs         # Grid + proximity snap (FR22-25)
│   │       ├── gizmos.rs           # Selection highlight rendering
│   │       ├── inspector.rs        # Custom Inspector UI
│   │       ├── collision.rs        # Collision mesh generation
│   │       ├── history.rs          # Bridges Command to EditorUndoRedoManager
│   │       └── logging.rs          # Godot logger backend
│   │
│   └── tests/                      # Integration tests
│       ├── undo_redo_chains.rs     # Multi-command undo/redo
│       ├── selection_transforms.rs # Select + transform workflows
│       ├── serialization.rs        # Save/load roundtrip
│       └── preview_commit.rs       # Preview → commit/cancel
│
└── docs/                           # Optional documentation
    ├── architecture.md             # This document (generated)
    └── keybindings.md              # User-facing shortcut reference
```

### Architectural Boundaries

**Godot Boundary:**
```
┌─────────────────────────────────────────────────────────────┐
│                      editor/ module                         │
│  (Godot types allowed: Gd<>, PackedArrays, signals, etc.)  │
├─────────────────────────────────────────────────────────────┤
│              Pure Rust Boundary (no Godot types)            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  geometry/   │  │  selection/  │  │    tools/    │      │
│  │              │  │              │  │   commands/  │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

**Serialization Boundary:**
```
geometry/serialization.rs          editor/blockot_node.rs
┌─────────────────────────┐        ┌─────────────────────────┐
│ Pure Rust functions:    │        │ Godot integration:      │
│                         │◄──────►│                         │
│ to_packed_arrays()      │        │ #[export] fields        │
│ from_packed_arrays()    │        │ _ready() loads geometry │
│                         │        │ Calls serialization fns │
└─────────────────────────┘        └─────────────────────────┘
```

**Data Flow:**
```
User Input (Godot)
       │
       ▼
editor/input_handler.rs
       │
       ▼ (begin_preview / update_preview)
geometry/preview.rs ◄──── Rendering (get_vertex_for_render)
       │
       ▼ (commit_preview)
tools/commands/*.rs ◄──── Command created on commit
       │
       ▼ (execute)
geometry/mesh.rs ◄──── Source of truth modified
       │
       ▼
editor/history.rs ◄──── Bridges to EditorUndoRedoManager
       │
       ▼
geometry/ → editor/blockot_node.rs ◄──── Rebuild ArrayMesh
```

### Undo/Redo Integration

**Pattern:** Use Godot's `EditorUndoRedoManager`, not internal history

```rust
// editor/history.rs
use godot::prelude::*;
use godot::classes::EditorUndoRedoManager;
use crate::tools::Command;

pub fn execute_with_undo(
    undo_redo: Gd<EditorUndoRedoManager>,
    geometry: &mut BlockotGeometry,
    cmd: impl Command + 'static,
) {
    let cmd_do = cmd.clone();
    let cmd_undo = cmd.clone();

    // Execute immediately
    cmd.execute(geometry);

    // Register with Godot's undo system
    let mut ur = undo_redo.bind_mut();
    ur.create_action(cmd.name());
    ur.add_do_method(/* callable to re-execute */);
    ur.add_undo_method(/* callable to undo */);
    ur.commit_action(false);  // false = already executed
}
```

### File Responsibilities

| File | Responsibility | FR Coverage |
|------|----------------|-------------|
| `lib.rs` | Extension entry, class registration | - |
| `error.rs` | `BlockotError` enum | - |
| `test_utils.rs` | Test fixtures (`pub mod`, not `cfg(test)`) | - |
| `geometry/mesh.rs` | `BlockotGeometry` struct | FR1 |
| `geometry/primitives.rs` | Box, Plane, Cylinder, Sphere | FR2-5 |
| `geometry/preview.rs` | PreviewState, begin/update/commit | FR15-19 |
| `geometry/serialization.rs` | Pure `to/from_packed_arrays()` | FR31-32 |
| `selection/mod.rs` | Selection struct, vertex-canonical | FR9-14 |
| `tools/mod.rs` | Command trait | FR30 |
| `tools/commands/*.rs` | Individual commands | FR15-21 |
| `editor/plugin.rs` | EditorPlugin | FR33 |
| `editor/blockot_node.rs` | BlockotNode, calls serialization | FR31-32 |
| `editor/edit_mode.rs` | Edit mode state machine | FR6-8 |
| `editor/input_handler.rs` | G/R/S/E/C handling | FR15-19 |
| `editor/snapping.rs` | Grid + proximity snap | FR22-25 |
| `editor/history.rs` | Bridges Command → EditorUndoRedoManager | FR30 |
| `editor/collision.rs` | Collision generation | FR28-29 |

### Module Scaling Notes

**Current:** `editor/` is flat (10 files) — acceptable for MVP

**Future split if needed:**
```
editor/
├── mod.rs
├── plugin.rs
├── blockot_node.rs
├── input/
│   ├── mod.rs
│   ├── handler.rs
│   └── snapping.rs
├── rendering/
│   ├── mod.rs
│   └── gizmos.rs
└── ...
```

**Trigger for split:** Any file exceeds ~500 lines or has clear subcomponents.

## Architecture Validation Results

### Coherence Validation ✅

**Decision Compatibility:**
All architectural decisions work together cohesively:
- Rust/gdext + BlockotNode + Hybrid Geometry form a consistent stack
- Command Pattern + Preview State + EditorUndoRedoManager integration is coherent
- Vertex-Canonical selection aligns naturally with all transform commands
- Flat array serialization works with Godot's native .tscn format

**Pattern Consistency:**
Implementation patterns support architectural decisions:
- Naming conventions (snake_case Rust, past-tense signals) are consistent
- Module organization maintains Godot-type boundary at `editor/`
- Command trait with validation-at-construction is enforced uniformly
- Preview → Commit flow is consistent across all transform operations

**Structure Alignment:**
Project structure enables all architectural decisions:
- `geometry/`, `selection/`, `tools/` are pure Rust (testable without Godot)
- `editor/` module properly isolates Godot dependencies
- Test organization (inline + `tests/`) supports coverage requirements
- CI workflows cover lint, test, and cross-platform build

### Requirements Coverage Validation ✅

**Functional Requirements Coverage:**

| Category | FRs | Architectural Support |
|----------|-----|----------------------|
| Geometry Creation | FR1-5 | `geometry/primitives.rs`, `BlockotGeometry` |
| Edit Mode | FR6-8 | `editor/edit_mode.rs`, state machine |
| Selection | FR9-14 | `selection/mod.rs`, vertex-canonical model |
| Transform Tools | FR15-19 | `tools/commands/`, Preview State pattern |
| Geometry Ops | FR20-21 | `extrude_faces.rs`, `cut_edge.rs` |
| Snapping | FR22-25 | `editor/snapping.rs` |
| Properties | FR26-29 | `geometry/properties.rs`, `editor/collision.rs` |
| Editor Integration | FR30-33 | `editor/history.rs`, `blockot_node.rs` |

**Non-Functional Requirements Coverage:**

| NFR | Architectural Support |
|-----|----------------------|
| Performance (<100ms transform) | Fast vertex updates, topology changes only on action start/commit |
| Reliability (undo/redo correctness) | Command Pattern with exact inverse transforms |
| Integration (Godot conventions) | EditorUndoRedoManager, native serialization |

### Implementation Readiness Validation ✅

**Decision Completeness:**
- All critical technology choices documented with rationale
- Specific versions targeted (Godot 4.1+ minimum, 4.2+ recommended)
- Command trait fully specified with validation rules
- Preview State pattern documented with complete code examples

**Structure Completeness:**
- Complete directory structure with all files mapped to FRs
- Module boundaries clearly defined
- Integration points (serialization, history, logging) specified
- Scaling notes provided for future growth

**Pattern Completeness:**
- All transform operations follow Preview → Commit flow
- Error handling pattern (`Result<T, BlockotError>`) defined
- Test requirements per command documented
- Logging pattern with injected backend specified

### Gap Analysis Results

**No Critical Gaps:** All requirements have architectural support

**Minor Enhancement Opportunities (Post-MVP):**
- Resource extraction for reusable geometry (documented as future path)
- Auto-vertex-split feature (explicitly deferred)
- Additional collision type options beyond convex/concave

### Architecture Completeness Checklist

**✅ Requirements Analysis**
- [x] Project context thoroughly analyzed
- [x] Scale and complexity assessed (Low-Medium)
- [x] Technical constraints identified (Godot 4.0+, EditorPlugin)
- [x] Cross-cutting concerns mapped (undo/redo as foundational)

**✅ Architectural Decisions**
- [x] Critical decisions documented with rationale
- [x] Technology stack fully specified (Rust + gdext)
- [x] Integration patterns defined (EditorUndoRedoManager)
- [x] Performance considerations addressed (affected vertices, not total)

**✅ Implementation Patterns**
- [x] Naming conventions established
- [x] Structure patterns defined (Godot types at edges)
- [x] Communication patterns specified (signals, logging)
- [x] Process patterns documented (Preview → Commit)

**✅ Project Structure**
- [x] Complete directory structure defined
- [x] Component boundaries established
- [x] Integration points mapped
- [x] Requirements to structure mapping complete

### Architecture Readiness Assessment

**Overall Status:** READY FOR IMPLEMENTATION

**Confidence Level:** High — based on comprehensive validation results

**Key Strengths:**
- Foundational undo/redo integration ensures data integrity
- Command Pattern enables deterministic testing
- Pure Rust core modules maximize testability
- Preview State pattern prevents accidental state mutation during drags
- Hybrid geometry representation balances performance and correctness

**Areas for Future Enhancement:**
- Resource-based geometry storage for reusability
- Extended collision options
- Module subdivision as codebase grows

### Implementation Handoff

**AI Agent Guidelines:**
- Follow all architectural decisions exactly as documented
- Use implementation patterns consistently across all components
- Respect project structure and module boundaries
- Refer to this document for all architectural questions
- Validate commands at construction, never at execute time
- Use Preview State for all drag operations

**First Implementation Priority:**
Initialize Rust/gdext project structure with `cargo init` and configure `Cargo.toml` for godot-rust bindings.

## Architecture Completion Summary

### Workflow Completion

**Architecture Decision Workflow:** COMPLETED ✅
**Total Steps Completed:** 8
**Date Completed:** 2026-01-24
**Document Location:** `_bmad-output/planning-artifacts/architecture.md`

### Final Architecture Deliverables

**📋 Complete Architecture Document**
- All architectural decisions documented with specific versions
- Implementation patterns ensuring AI agent consistency
- Complete project structure with all files and directories
- Requirements to architecture mapping
- Validation confirming coherence and completeness

**🏗️ Implementation Ready Foundation**
- 5 core architectural decisions made (BlockotNode, Hybrid Geometry, Vertex-Canonical, Command Pattern, Flat Array Serialization)
- 9 implementation patterns defined (naming, modules, commands, preview, external mod, errors, tests, logging)
- 6 architectural components specified (geometry, selection, tools, editor, plus supporting modules)
- 33 functional requirements fully supported

**📚 AI Agent Implementation Guide**
- Technology stack: Rust + gdext targeting Godot 4.1+
- Consistency rules preventing implementation conflicts
- Project structure with clear module boundaries
- Integration patterns via EditorUndoRedoManager

### Implementation Handoff

**For AI Agents:**
This architecture document is your complete guide for implementing blockot. Follow all decisions, patterns, and structures exactly as documented.

**First Implementation Priority:**
```bash
cd rust && cargo init --lib
# Configure Cargo.toml for godot-rust/gdext
```

**Development Sequence:**
1. Initialize project using documented structure
2. Set up development environment (Godot project + Rust crate)
3. Implement core architectural foundations (`BlockotGeometry`, `Command` trait)
4. Build features following established patterns
5. Maintain consistency with documented rules

### Quality Assurance Checklist

**✅ Architecture Coherence**
- [x] All decisions work together without conflicts
- [x] Technology choices are compatible
- [x] Patterns support the architectural decisions
- [x] Structure aligns with all choices

**✅ Requirements Coverage**
- [x] All functional requirements are supported
- [x] All non-functional requirements are addressed
- [x] Cross-cutting concerns are handled
- [x] Integration points are defined

**✅ Implementation Readiness**
- [x] Decisions are specific and actionable
- [x] Patterns prevent agent conflicts
- [x] Structure is complete and unambiguous
- [x] Examples are provided for clarity

### Project Success Factors

**🎯 Clear Decision Framework**
Every technology choice was made collaboratively with clear rationale, ensuring all stakeholders understand the architectural direction.

**🔧 Consistency Guarantee**
Implementation patterns and rules ensure that multiple AI agents will produce compatible, consistent code that works together seamlessly.

**📋 Complete Coverage**
All project requirements are architecturally supported, with clear mapping from business needs to technical implementation.

**🏗️ Solid Foundation**
The chosen Rust + gdext stack with Command Pattern provides a production-ready, testable foundation following current best practices.

---

**Architecture Status:** READY FOR IMPLEMENTATION ✅

**Next Phase:** Begin implementation using the architectural decisions and patterns documented herein.

**Document Maintenance:** Update this architecture when major technical decisions are made during implementation.
