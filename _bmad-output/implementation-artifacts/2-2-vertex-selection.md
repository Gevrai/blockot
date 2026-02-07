# Story 2.2: Vertex Selection

Status: done

## Story

As a **Godot developer**,
I want **to select individual vertices in edit mode**,
so that **I can manipulate specific points of my geometry**.

## Acceptance Criteria

1. **Given** I am in edit mode with vertex selection mode active **When** I click on a vertex **Then** that vertex becomes selected **And** the vertex is visually highlighted

2. **Given** a vertex is selected **When** I click on a different vertex (without Ctrl) **Then** the previous vertex is deselected **And** the new vertex is selected

3. **Given** I am in vertex selection mode **When** I click on empty space (no vertex) **Then** all vertices are deselected

## Tasks / Subtasks

- [x] **Task 1: Add `select_vertex` and `deselect_all` methods to Selection** (AC: 1, 2, 3)
  - [x] Add `select_vertex(index: usize)` that clears current selection and selects a single vertex
  - [x] Add `toggle_vertex(index: usize)` that adds/removes a vertex (for future multi-select)
  - [x] Verify `clear()` already handles deselect all (it does)
  - [x] Add unit tests for all new methods

- [x] **Task 2: Implement screen-space vertex hit testing in pure Rust** (AC: 1, 3)
  - [x] Create `selection/hit_test.rs` with `find_closest_vertex()` function
  - [x] Input: list of 2D screen positions (projected vertices) + mouse position (Vector2) + pixel threshold (f32)
  - [x] Output: `Option<usize>` — index of the closest vertex within threshold, or None
  - [x] Algorithm: iterate all projected positions, find minimum distance to mouse, return if under threshold
  - [x] Default pixel threshold: 15.0 pixels
  - [x] Add unit tests for: vertex found, no vertex in range, closest wins when multiple in range

- [x] **Task 3: Handle mouse click in `forward_3d_gui_input()`** (AC: 1, 2, 3)
  - [x] In `plugin.rs`, detect `InputEventMouseButton` left click (pressed, not released)
  - [x] Only process clicks when edit mode is active and selection mode is Vertex
  - [x] Extract mouse position from `InputEventMouseButton::get_position()`
  - [x] Get Camera3D from the `viewport_camera` parameter
  - [x] Consume the click event (return `AfterGuiInput::STOP.ord()`) when in edit mode

- [x] **Task 4: Implement vertex projection and picking in plugin** (AC: 1, 2, 3)
  - [x] Access the BlockotNode's geometry via `Gd::<Object>::try_from_instance_id()` and `try_cast::<BlockotNode>()`
  - [x] Project each vertex to screen space using `camera.unproject_position(vertex_world_pos)`
  - [x] Filter out vertices behind the camera using `camera.is_position_behind()`
  - [x] Account for the BlockotNode's transform: `node.get_global_transform() * vertex_local_pos`
  - [x] Call `find_closest_vertex()` from hit_test.rs with projected positions and mouse pos
  - [x] If vertex found: call `select_vertex(index)` on the node's Selection
  - [x] If no vertex found (empty space click): call `clear()` on the Selection

- [x] **Task 5: Store Selection in BlockotNode and sync with plugin** (AC: 1, 2, 3)
  - [x] Add `selection: Selection` field to `BlockotNode`
  - [x] Add `pub fn selection(&self) -> &Selection` and `pub fn selection_mut(&mut self) -> &mut Selection` accessors
  - [x] Clear selection when entering edit mode (fresh start)
  - [x] Clear selection when exiting edit mode

- [x] **Task 6: Update vertex handle rendering to show selection state** (AC: 1, 2)
  - [x] Modify `show_vertex_handles()` in `blockot_node.rs` to accept a `&Selection` parameter
  - [x] Selected vertices: draw handles in **white** (Godot editor selection color) with larger size
  - [x] Unselected vertices: keep current orange handles at normal size
  - [x] Add `refresh_vertex_handles()` method that rebuilds handles (called after selection changes)
  - [x] Plugin calls `refresh_vertex_handles()` on the BlockotNode after every selection change

- [x] **Task 7: Build, test, and verify** (AC: 1, 2, 3)
  - [x] `cargo clippy` — zero warnings
  - [x] `cargo test` — all existing tests pass, new tests pass
  - [ ] Manual test: enter edit mode, click vertex — vertex highlights white
  - [ ] Manual test: click different vertex — old deselects, new selects
  - [ ] Manual test: click empty space — all vertices deselect

## Dev Notes

### Architecture Compliance

This story implements vertex selection from Architecture Decision 3 (Vertex-Canonical Selection):

- **Vertex-Canonical Model:** Selection stores `vertex_indices: HashSet<usize>` as canonical form. In Vertex mode, each click selects a single vertex index. [Source: architecture.md#Decision-3-Vertex-Canonical-Selection]
- **Pure Rust Core:** Hit testing logic (`find_closest_vertex`) lives in `selection/hit_test.rs` — pure Rust, no Godot types. Uses `Vector2` from godot prelude (math type, acceptable in core). [Source: architecture.md#Architectural-Boundaries]
- **Editor Integration:** Mouse event handling and Camera3D projection happen in `editor/plugin.rs` (Godot types allowed). [Source: architecture.md#editor/input_handler.rs]

### Screen-Space Vertex Picking Algorithm

**Approach:** Project 3D vertices to 2D screen space, find closest to mouse click.

```rust
// In editor/plugin.rs (Godot types allowed here)
fn pick_vertex(
    camera: &Gd<Camera3D>,
    node_transform: Transform3D,
    geometry: &BlockotGeometry,
    mouse_pos: Vector2,
) -> Option<usize> {
    let projected: Vec<Option<Vector2>> = geometry.vertices.iter()
        .map(|v| {
            let world_pos = node_transform * *v;
            if camera.is_position_behind(world_pos) {
                None
            } else {
                Some(camera.unproject_position(world_pos))
            }
        })
        .collect();

    find_closest_vertex(&projected, mouse_pos, 15.0)
}
```

**Why screen-space over ray-cast:**
- Simpler implementation for MVP
- Sufficient performance for meshes up to 500 vertices (NFR4)
- Naturally handles perspective (closer vertices appear larger on screen)
- 15-pixel threshold provides good usability

[Source: architecture.md — NFR4: up to 500 faces remains responsive]

### Hit Test Pure Rust Function

```rust
// selection/hit_test.rs (pure Rust)
pub fn find_closest_vertex(
    screen_positions: &[Option<Vector2>],
    mouse_pos: Vector2,
    threshold: f32,
) -> Option<usize> {
    let mut best_index = None;
    let mut best_dist_sq = threshold * threshold;

    for (i, pos) in screen_positions.iter().enumerate() {
        if let Some(p) = pos {
            let dist_sq = (p.x - mouse_pos.x).powi(2) + (p.y - mouse_pos.y).powi(2);
            if dist_sq < best_dist_sq {
                best_dist_sq = dist_sq;
                best_index = Some(i);
            }
        }
    }

    best_index
}
```

### Mouse Click Detection in forward_3d_gui_input

The `forward_3d_gui_input()` method receives mouse events when the 3D viewport is active. This is the right place for click detection (unlike Tab which needed `process()` polling due to focus issues).

```rust
fn forward_3d_gui_input(
    &mut self,
    viewport_camera: Option<Gd<Camera3D>>,
    event: Option<Gd<InputEvent>>,
) -> i32 {
    // Only process when in edit mode
    if !self.edit_state.is_active() {
        return AfterGuiInput::PASS.ord();
    }

    let Some(event) = event else {
        return AfterGuiInput::PASS.ord();
    };
    let Some(camera) = viewport_camera else {
        return AfterGuiInput::PASS.ord();
    };

    // Detect left mouse button click
    if let Ok(mb) = event.try_cast::<InputEventMouseButton>() {
        if mb.is_pressed() && mb.get_button_index() == MouseButton::LEFT {
            let mouse_pos = mb.get_position();
            self.handle_vertex_click(&camera, mouse_pos);
            return AfterGuiInput::STOP.ord();
        }
    }

    AfterGuiInput::PASS.ord()
}
```

**Key gdext notes:**
- `try_cast::<T>()` consumes the `Gd<T>` — this is fine since we don't need the original after casting
- `InputEventMouseButton` methods: `is_pressed()`, `get_button_index()`, `get_position()`
- `MouseButton::LEFT` from `godot::global::MouseButton`
- Return `AfterGuiInput::STOP.ord()` to consume the click (prevent Godot from selecting nodes underneath)

[Source: MEMORY.md — gdext API Gotchas — try_cast consumes Gd<T>]

### Node Transform Consideration

Vertices in `BlockotGeometry` are in local space. To project them to screen space correctly, must apply the node's global transform:

```rust
let global_transform = node.base().get_global_transform();
let world_pos = global_transform * local_vertex;
let screen_pos = camera.unproject_position(world_pos);
```

This is important — without this, vertices would project incorrectly if the BlockotNode is translated/rotated in the scene.

### Visual Feedback

**Selection colors (from UX spec):**
- Inherit Godot editor's selection color scheme (typically orange/white)
- **Unselected vertices:** Orange crosses (existing from Story 2.1)
- **Selected vertices:** White crosses, slightly larger (1.5x handle_size)

The handle rendering in `show_vertex_handles()` needs to accept selection state. After each click, call `refresh_vertex_handles()` to rebuild the ImmediateMesh with correct colors.

[Source: ux-design-specification.md#Selection-Highlighting]

### Project Structure After This Story

```
rust/src/
├── lib.rs
├── error.rs
├── test_utils.rs
├── geometry/
│   ├── mod.rs
│   ├── mesh.rs
│   ├── face.rs
│   ├── primitives.rs
│   └── serialization.rs
├── selection/
│   ├── mod.rs                # MODIFIED: add select_vertex(), toggle_vertex()
│   ├── modes.rs
│   └── hit_test.rs           # NEW: find_closest_vertex()
├── tools/
│   ├── mod.rs
│   └── commands/
│       ├── mod.rs
│       └── move_vertices.rs
└── editor/
    ├── mod.rs
    ├── blockot_node.rs       # MODIFIED: add Selection field, refresh_vertex_handles()
    ├── history.rs
    ├── plugin.rs             # MODIFIED: handle mouse clicks, vertex picking
    └── edit_mode.rs
```

### Previous Story Intelligence (Story 2.1)

**Key Learnings:**
- Tab detection via `forward_3d_gui_input()` didn't work due to Godot editor focus issues — but **mouse clicks DO work** in `forward_3d_gui_input()` because the 3D viewport keeps focus during mouse interaction
- Handle rendering uses `ImmediateMesh` with `PrimitiveType::LINES` — same pattern extends for color-coded selection
- `handle_mesh_instance` stored as `Option<Gd<MeshInstance3D>>` in BlockotNode — same pattern for refreshing
- Plugin accesses nodes via `Gd::<Object>::try_from_instance_id()` + `try_cast::<BlockotNode>()` — reuse same pattern for selection updates
- `is_class("BlockotNode")` works for GDExtension classes — used in `handles()` and node detection
- All 39 tests pass with zero regressions as of Story 2.1

**Files to Modify:**
- `rust/src/selection/mod.rs` — Add select_vertex(), toggle_vertex() methods
- `rust/src/editor/plugin.rs` — Add mouse click handling and vertex picking
- `rust/src/editor/blockot_node.rs` — Add Selection field, refresh_vertex_handles()

**Established Patterns:**
- gdext v0.4.5 pinned in Cargo.toml
- `#[class(base=..., tool)]` for editor execution
- `#[func]` for GDScript-callable methods, `#[signal]` for signals
- `InstanceId::to_i64()` for node tracking — never store `Gd<T>` across frames
- Handle material: unshaded, vertex color, depth test disabled

**Debug Log from Story 2.1:**
- `#[export]` auto-generates getter — don't define manual one with same name
- `Array::get()` returns `Option<T>` in gdext v0.4.5
- `try_cast::<T>()` consumes the `Gd<T>` — clone first if needed later

[Source: 2-1-enter-and-exit-edit-mode.md#Dev-Agent-Record]

### Git Intelligence

**Recent Commits:**
- `4e26e85` enter and exit edit mode — Story 2.1 (most recent)
- `56efb55` implement save load — Story 1.3
- `2d2fe52` feat: create blockot node with default cube — Story 1.2

**Patterns Established:**
- Selection module exists with vertex-canonical model
- EditModeState tracks active node via InstanceId
- BlockotPlugin handles input and notifies nodes via instance ID lookup
- ImmediateMesh handles with unshaded material for always-on-top rendering

### Critical Don'ts for This Story

- **DO NOT** implement multi-select (Ctrl+click) — that's Story 2.6
- **DO NOT** implement edge or face selection — that's Stories 2.3 and 2.4
- **DO NOT** add selection mode switching UI — that's Story 2.5
- **DO NOT** store `Gd<T>` across frames — use InstanceId, retrieve fresh references
- **DO NOT** create a separate gizmos.rs — keep handle rendering in blockot_node.rs (established pattern from 2.1)
- **DO NOT** use `input()` for mouse clicks — use `forward_3d_gui_input()` which is reliable for mouse events
- **DO NOT** add undo/redo for selection changes — selection state is ephemeral (not in undo history per architecture)

### Critical Do's for This Story

- **DO** use screen-space projection for vertex picking (simpler, sufficient for MVP)
- **DO** filter out vertices behind the camera with `is_position_behind()`
- **DO** apply BlockotNode's global transform before projecting vertices
- **DO** consume the click event when in edit mode (return `AfterGuiInput::STOP.ord()`)
- **DO** rebuild vertex handles after every selection change (color update)
- **DO** clear selection when entering/exiting edit mode
- **DO** write unit tests for `find_closest_vertex()` and Selection methods
- **DO** keep hit testing logic in pure Rust (`selection/hit_test.rs`)

### Key gdext Types to Use

- `InputEventMouseButton` — Mouse button click events
- `MouseButton::LEFT` — From `godot::global::MouseButton`
- `Camera3D::unproject_position(Vector3) -> Vector2` — 3D world to 2D screen
- `Camera3D::is_position_behind(Vector3) -> bool` — Behind camera check
- `AfterGuiInput::STOP` / `AfterGuiInput::PASS` — Event consumption
- `Transform3D` — Node's global transform for local-to-world conversion

### Testing Strategy

**Unit Tests (Pure Rust):**
- `selection/mod.rs`: select_vertex(), toggle_vertex(), clear after select
- `selection/hit_test.rs`: find_closest_vertex() — found, not found, closest wins, behind camera (None)
- Expected: ~8-10 new unit tests

**Manual Verification:**
- Enter edit mode, click on a vertex — white highlight appears
- Click on a different vertex — previous deselects (orange), new selects (white)
- Click on empty space — all vertices return to orange
- Rotate camera, verify selection still works from different angles
- Test with BlockotNode that has non-identity transform (translated/rotated)

[Source: project-context.md#Testing-Rules]

### References

- [Architecture: architecture.md#Decision-3-Vertex-Canonical-Selection]
- [Architecture: architecture.md#Architectural-Boundaries]
- [Architecture: architecture.md#editor/input_handler.rs]
- [PRD: prd.md — FR9 (vertex selection)]
- [Epics: epics.md — Epic 2, Story 2.2]
- [UX Design: ux-design-specification.md#Selection-Highlighting]
- [Project Context: project-context.md]
- [Previous Story: 2-1-enter-and-exit-edit-mode.md]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.6

### Debug Log References

- `show_vertex_handles()` reads `self.selection.vertex_indices` directly rather than taking a `&Selection` parameter — simpler since selection is owned by BlockotNode
- `refresh_vertex_handles()` simply calls `show_vertex_handles()` which tears down and rebuilds the ImmediateMesh; acceptable for MVP vertex counts

### Completion Notes List

- Task 1: Added `select_vertex()` (clears + selects one) and `toggle_vertex()` (add/remove) to Selection. 6 new unit tests.
- Task 2: Created `selection/hit_test.rs` with `find_closest_vertex()` — pure Rust, uses squared distance for efficiency. 7 new unit tests covering found/not-found/closest-wins/behind-camera/empty/boundary cases.
- Task 3: Implemented left-click detection in `forward_3d_gui_input()` — only processes when edit mode active and selection mode is Vertex. Consumes click with `AfterGuiInput::STOP`.
- Task 4: Implemented `handle_vertex_click()` — projects vertices to screen space via Camera3D, applies global transform, calls `find_closest_vertex()`, updates selection or clears on empty-space click.
- Task 5: Added `selection: Selection` field to BlockotNode with `selection()`/`selection_mut()` accessors. Selection cleared on enter and exit edit mode.
- Task 6: Modified `show_vertex_handles()` to color selected vertices white at 1.5x size, unselected orange at normal size. Added `refresh_vertex_handles()` called after every selection change.
- Task 7: `cargo clippy` — zero warnings. `cargo test` — 52 tests pass (13 new, 39 existing). Manual tests require Godot editor.

### Change Log

- 2026-02-07: Implemented vertex selection (Story 2.2) — 13 new unit tests, 3 files modified, 1 file created
- 2026-02-07: Code review fixes — added missing files to File List, extracted hardcoded 15.0 threshold to constant `VERTEX_SELECTION_THRESHOLD_PX`

### File List

- `rust/src/selection/mod.rs` — MODIFIED: added `select_vertex()`, `toggle_vertex()`, re-exported `hit_test` module
- `rust/src/selection/hit_test.rs` — NEW: `find_closest_vertex()` pure Rust screen-space hit testing
- `rust/src/editor/plugin.rs` — MODIFIED: mouse click handling in `forward_3d_gui_input()`, vertex projection in `handle_vertex_click()`
- `rust/src/editor/blockot_node.rs` — MODIFIED: added `Selection` field, accessors, selection-aware handle rendering, `refresh_vertex_handles()`
- `_bmad-output/implementation-artifacts/sprint-status.yaml` — MODIFIED: story status tracking
- `godot/test_scenes/simple_cube.tscn` — MODIFIED: test scene updates
