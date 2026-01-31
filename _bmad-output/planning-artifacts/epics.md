---
stepsCompleted: ['step-01-validate-prerequisites', 'step-02-design-epics', 'step-03-create-stories', 'step-04-final-validation']
workflowStatus: complete
completedAt: '2026-01-31'
inputDocuments:
  - _bmad-output/planning-artifacts/prd.md
  - _bmad-output/planning-artifacts/architecture.md
  - _bmad-output/planning-artifacts/ux-design-specification.md
---

# blockot - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for blockot, decomposing the requirements from the PRD, UX Design if it exists, and Architecture requirements into implementable stories.

## Requirements Inventory

### Functional Requirements

**Geometry Creation (FR1-FR5)**
- FR1: User can add a blockot geometry node to a Godot scene
- FR2: User can create a Box primitive (default 1m cube)
- FR3: User can create a Plane primitive
- FR4: User can create a Cylinder primitive
- FR5: User can create a Sphere primitive

**Edit Mode (FR6-FR8)**
- FR6: User can enter edit mode on blockot geometry (Tab key)
- FR7: User can exit edit mode to normal Godot editor (Tab key)
- FR8: User can only edit one blockot geometry at a time

**Selection (FR9-FR14)**
- FR9: User can select individual vertices
- FR10: User can select individual edges
- FR11: User can select individual faces
- FR12: User can switch between vertex, edge, and face selection modes
- FR13: User can multi-select geometry elements (Ctrl+click)
- FR14: User can deselect all selected elements

**Transform Tools (FR15-FR19)**
- FR15: User can move selected geometry (G key)
- FR16: User can rotate selected geometry (R key)
- FR17: User can scale selected geometry (S key)
- FR18: User can confirm a transform operation (click)
- FR19: User can cancel a transform operation (Escape or right-click)

**Geometry Operations (FR20-FR21)**
- FR20: User can extrude selected faces/edges to create new geometry (E key)
- FR21: User can cut/bisect faces or edges with a straight line (C key)

**Snapping System (FR22-FR25)**
- FR22: User can snap geometry to a configurable grid (default 1m)
- FR23: User can snap geometry to nearby vertices/edges/faces (proximity snap)
- FR24: User can temporarily disable all snapping (Shift modifier)
- FR25: User can configure the grid snap size

**Geometry Properties (FR26-FR29)**
- FR26: User can set face direction to Outward
- FR27: User can set face direction to Inward
- FR28: User can set face direction to Both (double-sided)
- FR29: User can enable/disable collision generation (default: enabled)

**Editor Integration (FR30-FR33)**
- FR30: User can undo any edit operation
- FR31: User can redo any undone operation
- FR32: Blockot geometry persists correctly when scene is saved
- FR33: Blockot geometry loads correctly when scene is opened

### NonFunctional Requirements

**Performance (NFR1-NFR4)**
- NFR1: Transform operations (G/R/S) respond without perceptible lag (<100ms feedback)
- NFR2: Entering/exiting edit mode completes within 200ms
- NFR3: Snapping calculations cause no visible stutter during drag operations
- NFR4: Geometry with up to 500 faces remains responsive (typical blockout scale)

**Reliability (NFR5-NFR8)**
- NFR5: Undo/redo correctly restores all geometry state without corruption
- NFR6: Scene save persists all blockot geometry data without loss
- NFR7: Scene load restores geometry exactly as saved
- NFR8: Plugin does not crash Godot editor under normal usage

**Integration (NFR9-NFR12)**
- NFR9: Plugin follows Godot addon conventions (`addons/blockot/plugin.cfg`)
- NFR10: Plugin integrates with Godot's native undo/redo system
- NFR11: BlockotNode properties appear correctly in Godot's Inspector panel
- NFR12: Plugin does not interfere with standard Godot editor operations when not in edit mode

### Additional Requirements

**From Architecture:**
- Rust with gdext (godot-rust) as implementation language
- Godot 4.1+ minimum, 4.2+ recommended
- Project structure: `rust/` for Rust code, `godot/addons/blockot/` for plugin
- BlockotNode as custom node extending MeshInstance3D
- Hybrid geometry representation (Rust source of truth + cached ArrayMesh)
- Vertex-Canonical selection model with rendering hints
- Command Pattern with inverse transforms for undo/redo
- Preview State pattern for drag operations
- EditorUndoRedoManager integration (Godot's native undo system)
- Collision support: convex (fast) and concave (accurate) options
- Flat array serialization (PackedVector3Array, PackedInt32Array)
- CI pipeline: cargo clippy, cargo test, cross-compile Win/Mac/Linux

**From UX Design:**
- Edit mode indicator via handle appearance (no viewport border/tint needed)
- Use Godot's default selection colors (inherit editor theme)
- Real-time geometry movement during transforms (no ghost geometry)
- Axis constraint visualization when active (X/Y/Z lock lines)
- Default constraint to face normal for face operations
- Snap indicators with momentary visual feedback on snap
- No persistent grid plane visualization during transforms

### FR Coverage Map

| FR | Epic | Description |
|----|------|-------------|
| FR1 | Epic 1 | Add BlockotNode to scene |
| FR2 | Epic 1 | Create Box primitive |
| FR3 | Epic 7 | Create Plane primitive |
| FR4 | Epic 7 | Create Cylinder primitive |
| FR5 | Epic 7 | Create Sphere primitive |
| FR6 | Epic 2 | Enter edit mode (Tab) |
| FR7 | Epic 2 | Exit edit mode (Tab) |
| FR8 | Epic 2 | Single-node editing only |
| FR9 | Epic 2 | Select vertices |
| FR10 | Epic 2 | Select edges |
| FR11 | Epic 2 | Select faces |
| FR12 | Epic 2 | Switch selection modes |
| FR13 | Epic 2 | Multi-select (Ctrl+click) |
| FR14 | Epic 2 | Deselect all |
| FR15 | Epic 3 | Move (G key) |
| FR16 | Epic 3 | Rotate (R key) |
| FR17 | Epic 3 | Scale (S key) |
| FR18 | Epic 3 | Confirm transform (click) |
| FR19 | Epic 3 | Cancel transform (Escape) |
| FR20 | Epic 5 | Extrude (E key) |
| FR21 | Epic 5 | Cut/bisect (C key) |
| FR22 | Epic 3 | Grid snap |
| FR23 | Epic 6 | Proximity snap |
| FR24 | Epic 6 | Shift to disable snap |
| FR25 | Epic 6 | Configure grid size |
| FR26 | Epic 7 | Face direction: Outward |
| FR27 | Epic 7 | Face direction: Inward |
| FR28 | Epic 7 | Face direction: Both |
| FR29 | Epic 7 | Collision toggle |
| FR30 | Epic 3 | Undo |
| FR31 | Epic 3 | Redo |
| FR32 | Epic 1 | Scene save |
| FR33 | Epic 1 | Scene load |

## Epic List

### Epic 1: Foundation — First Editable Geometry
User can add a BlockotNode with a default cube to a scene, and it persists correctly on save/load. Undo/redo integration proven as part of Story 1.2.

**FRs covered:** FR1, FR2, FR32, FR33
**Mitigations:** Pin gdext version in Cargo.toml. Undo pattern proven in Story 1.2.
**Stories:** 3 (1.1 Init Project, 1.2 BlockotNode+Cube+Undo, 1.3 Save/Load)

---

### Epic 2: Edit Mode & Selection
User can Tab into edit mode and select vertices, edges, or faces. The foundation for all editing operations.

**FRs covered:** FR6, FR7, FR8, FR9, FR10, FR11, FR12, FR13, FR14
**Notes:** Share manual test builds for early user feedback before CI is ready.

---

### Epic 3: Transform Tools + Undo + Grid Snap
User can move (G), rotate (R), and scale (S) selected geometry with full undo/redo support and grid snapping for precision. The core Blender-like editing loop.

**FRs covered:** FR15, FR16, FR17, FR18, FR19, FR22, FR30, FR31
**Notes:** Builds on proven undo pattern from Epic 1. Grid snap bundled for UX coherence.

---

### Epic 4: Distribution & CI Pipeline
Developer can build cross-platform releases and share them with testers. Automated CI runs clippy, tests, and produces Windows/macOS/Linux binaries.

**FRs covered:** *(Architecture: CI/CD, cross-platform builds)*
**Notes:** Gated epic — required before Epic 5. Enables consistent test builds for Brian.

---

### Epic 5: Geometry Operations
User can extrude faces to create new geometry and cut/bisect to subdivide shapes. Actual mesh modeling is now possible.

**FRs covered:** FR20, FR21

---

### Epic 6: Advanced Snapping
User can snap geometry to nearby vertices/edges/faces (proximity snap), temporarily disable snapping with Shift, and configure grid size.

**FRs covered:** FR23, FR24, FR25
**Architecture Note:** SnapProvider trait abstraction enables future cross-node snapping without core rewrite.

---

### Epic 7: Primitives & Properties
User can create all primitive types (Plane, Cylinder, Sphere) and configure face direction and collision settings. Completes the MVP feature set.

**FRs covered:** FR3, FR4, FR5, FR26, FR27, FR28, FR29
**Notes:** All 4 primitives ship together — no splitting. Atomic delivery.

---

## Risk Mitigations

| Risk | Mitigation | Epic |
|------|------------|------|
| gdext breaking changes | Pin versions in Cargo.toml, document upgrade path | 1 |
| Undo/redo complexity | Undo pattern proven in Story 1.2 | 1 |
| Late user feedback | Share manual builds before CI is ready | 2, 3 |
| Primitive scope creep | All primitives in single atomic epic | 7 |
| CI procrastination | CI is gated — required before Epic 5 | 4 |

---

## Epic 1: Foundation — First Editable Geometry

**Goal:** User can add a BlockotNode with a default cube to a scene, and it persists correctly on save/load. Undo/redo integration proven as part of Story 1.2.

**FRs covered:** FR1, FR2, FR32, FR33
**Mitigations:** Pin gdext version in Cargo.toml. Undo pattern proven in Story 1.2.
**Stories:** 3

### Story 1.1: Initialize Rust/gdext Project

**As a** developer,
**I want** a working Rust/gdext project structure with build pipeline,
**So that** I can compile the plugin and load it in Godot.

**Acceptance Criteria:**

**Given** the repository is cloned
**When** I run `cargo build` in the `rust/` directory
**Then** the project compiles without errors
**And** produces a shared library in the expected location

**Given** the compiled library exists
**When** I open the Godot project in `godot/`
**Then** the plugin appears in Project Settings → Plugins
**And** I can enable the plugin without errors

**Given** the plugin is enabled
**When** I check the Godot console
**Then** no errors or warnings from blockot appear

**Technical Notes:**
- Pin gdext version in Cargo.toml (mitigation for breaking changes)
- Project structure per Architecture: `rust/src/lib.rs`, `godot/addons/blockot/`
- Include `plugin.cfg` and `blockot.gdextension` files

---

### Story 1.2: Create BlockotNode with Default Cube and Undo Integration

**As a** Godot developer,
**I want** to add a BlockotNode to my scene that displays a default cube,
**So that** I have geometry to work with for level blockout.

**Acceptance Criteria:**

**Given** the blockot plugin is enabled
**When** I open the "Add Node" dialog and search for "BlockotNode"
**Then** BlockotNode appears in the node list

**Given** I am in a 3D scene
**When** I add a BlockotNode to the scene
**Then** a 1m cube appears at the node's origin
**And** the cube is visible in the 3D viewport

**Given** a BlockotNode exists in the scene
**When** I select it in the Scene tree
**Then** its properties appear in the Inspector panel
**And** I can see it extends MeshInstance3D

**Given** I execute a test command that modifies geometry (e.g., move single vertex)
**When** I trigger undo (Ctrl+Z)
**Then** the geometry returns to its previous state exactly

**Given** an undo has been performed
**When** I trigger redo (Ctrl+Shift+Z)
**Then** the geometry returns to the modified state exactly

**Technical Notes:**
- BlockotGeometry stores vertices/faces in pure Rust
- Default primitive: 1m unit cube centered at origin
- Cached ArrayMesh for rendering (hybrid representation per Architecture)
- Includes undo spike: minimal Command trait + EditorUndoRedoManager integration
- Proves the pattern before Epic 3 builds on it

**Test Requirements:**
- Unit test: command roundtrip (execute → undo → verify original state)
- Unit test: command creation with valid inputs succeeds

---

### Story 1.3: Implement Save/Load Serialization

**As a** Godot developer,
**I want** my BlockotNode geometry to persist when I save and reload the scene,
**So that** I don't lose my blockout work.

**Acceptance Criteria:**

**Given** a scene contains a BlockotNode with default cube
**When** I save the scene (.tscn file)
**Then** the scene file contains the geometry data
**And** the data is stored as PackedVector3Array/PackedInt32Array (git-diffable)

**Given** a saved scene with BlockotNode geometry
**When** I close and reopen the scene
**Then** the BlockotNode appears exactly as it was saved
**And** the cube geometry is identical (vertices, faces)

**Given** a saved scene with BlockotNode
**When** I inspect the .tscn file in a text editor
**Then** the geometry data is readable (not binary blob)

**Technical Notes:**
- Flat array serialization per Architecture (vertices, face_vertex_counts, face_indices)
- `#[export]` fields on BlockotNode for Godot serialization
- `geometry/serialization.rs` handles pure Rust ↔ PackedArray conversion

---

## Epic 2: Edit Mode & Selection

**Goal:** User can Tab into edit mode and select vertices, edges, or faces. The foundation for all editing operations.

**FRs covered:** FR6, FR7, FR8, FR9, FR10, FR11, FR12, FR13, FR14
**Notes:** Share manual test builds for early user feedback before CI is ready.

### Story 2.1: Enter and Exit Edit Mode

**As a** Godot developer,
**I want** to press Tab to enter and exit edit mode on a BlockotNode,
**So that** I can switch between editing geometry and normal scene editing.

**Acceptance Criteria:**

**Given** a BlockotNode is selected in the scene
**When** I press Tab
**Then** I enter edit mode on that BlockotNode
**And** visual handles appear on the geometry (vertices/edges/faces based on mode)

**Given** I am in edit mode on a BlockotNode
**When** I press Tab
**Then** I exit edit mode
**And** the visual handles disappear
**And** I return to normal Godot editor state

**Given** I am in edit mode on BlockotNode A
**When** I select a different BlockotNode B and press Tab
**Then** I exit edit mode on A and enter edit mode on B
**And** only one BlockotNode is in edit mode at a time (FR8)

---

### Story 2.2: Vertex Selection

**As a** Godot developer,
**I want** to select individual vertices in edit mode,
**So that** I can manipulate specific points of my geometry.

**Acceptance Criteria:**

**Given** I am in edit mode with vertex selection mode active
**When** I click on a vertex
**Then** that vertex becomes selected
**And** the vertex is visually highlighted

**Given** a vertex is selected
**When** I click on a different vertex (without Ctrl)
**Then** the previous vertex is deselected
**And** the new vertex is selected

**Given** I am in vertex selection mode
**When** I click on empty space (no vertex)
**Then** all vertices are deselected

---

### Story 2.3: Edge Selection

**As a** Godot developer,
**I want** to select individual edges in edit mode,
**So that** I can manipulate edges of my geometry.

**Acceptance Criteria:**

**Given** I am in edit mode with edge selection mode active
**When** I click on an edge
**Then** that edge becomes selected
**And** the edge is visually highlighted

**Given** an edge is selected
**When** I click on a different edge (without Ctrl)
**Then** the previous edge is deselected
**And** the new edge is selected

**Technical Notes:**
- Edge selection internally stores affected vertex indices (vertex-canonical model)
- Edge highlight rendered via selection rendering hints

---

### Story 2.4: Face Selection

**As a** Godot developer,
**I want** to select individual faces in edit mode,
**So that** I can manipulate faces of my geometry.

**Acceptance Criteria:**

**Given** I am in edit mode with face selection mode active
**When** I click on a face
**Then** that face becomes selected
**And** the face is visually highlighted

**Given** a face is selected
**When** I click on a different face (without Ctrl)
**Then** the previous face is deselected
**And** the new face is selected

**Technical Notes:**
- Face selection internally stores affected vertex indices (vertex-canonical model)
- Face highlight rendered via selection rendering hints

---

### Story 2.5: Selection Mode Switching

**As a** Godot developer,
**I want** to switch between vertex, edge, and face selection modes,
**So that** I can work with different geometry elements.

**Acceptance Criteria:**

**Given** I am in edit mode
**When** I switch to vertex selection mode (UI or hotkey)
**Then** the selection mode changes to vertex
**And** visual feedback indicates current mode

**Given** I am in edit mode
**When** I switch to edge selection mode
**Then** the selection mode changes to edge

**Given** I am in edit mode
**When** I switch to face selection mode
**Then** the selection mode changes to face

**Given** I have geometry selected in one mode
**When** I switch to a different selection mode
**Then** the selection is preserved where applicable (same vertices)

---

### Story 2.6: Multi-Select and Deselect All

**As a** Godot developer,
**I want** to select multiple elements with Ctrl+click and deselect all,
**So that** I can work with multiple geometry elements at once.

**Acceptance Criteria:**

**Given** I am in edit mode with one element selected
**When** I Ctrl+click on another element
**Then** that element is added to the selection
**And** both elements are now selected

**Given** I have multiple elements selected
**When** I Ctrl+click on an already-selected element
**Then** that element is removed from the selection
**And** other elements remain selected

**Given** I have elements selected
**When** I trigger "deselect all" (click empty space or hotkey)
**Then** all elements are deselected

---

## Epic 3: Transform Tools + Undo + Grid Snap

**Goal:** User can move (G), rotate (R), and scale (S) selected geometry with full undo/redo support and grid snapping for precision. The core Blender-like editing loop.

**FRs covered:** FR15, FR16, FR17, FR18, FR19, FR22, FR30, FR31
**Notes:** Builds on proven undo pattern from Epic 1. Grid snap bundled for UX coherence.

### Story 3.1: Move Tool

**As a** Godot developer,
**I want** to press G and drag to move selected geometry,
**So that** I can reposition vertices, edges, or faces.

**Acceptance Criteria:**

**Given** I have geometry selected in edit mode
**When** I press G
**Then** I enter move mode
**And** the geometry follows my mouse cursor in real-time (Preview State)

**Given** I am in move mode
**When** I click (left mouse button)
**Then** the move is confirmed
**And** the geometry stays at the new position

**Given** I am in move mode
**When** I press Escape or right-click
**Then** the move is cancelled
**And** the geometry returns to its original position

**Given** nothing is selected
**When** I press G
**Then** nothing happens (no error, just ignored)

**Technical Notes:**
- Uses Preview State pattern from Architecture
- Creates MoveVertices command on commit

**Test Requirements:**
- Unit test: MoveVertices command roundtrip (execute → undo → verify original)
- Unit test: MoveVertices with empty selection returns error
- Integration test: Preview State commit produces correct MoveVertices command

---

### Story 3.2: Rotate Tool

**As a** Godot developer,
**I want** to press R and drag to rotate selected geometry,
**So that** I can orient vertices, edges, or faces.

**Acceptance Criteria:**

**Given** I have geometry selected in edit mode
**When** I press R
**Then** I enter rotate mode
**And** the geometry rotates around selection center as I move mouse

**Given** I am in rotate mode
**When** I click to confirm
**Then** the rotation is applied
**And** the geometry stays at the new orientation

**Given** I am in rotate mode
**When** I press Escape or right-click
**Then** the rotation is cancelled
**And** the geometry returns to its original orientation

**Technical Notes:**
- Rotation around selection centroid
- Stores inverse transform for exact undo (not negated delta)

**Test Requirements:**
- Unit test: RotateVertices command roundtrip (execute → undo → verify original)
- Unit test: RotateVertices with empty selection returns error
- Unit test: inverse transform produces exact restoration (not approximation)

---

### Story 3.3: Scale Tool

**As a** Godot developer,
**I want** to press S and drag to scale selected geometry,
**So that** I can resize vertices, edges, or faces.

**Acceptance Criteria:**

**Given** I have geometry selected in edit mode
**When** I press S
**Then** I enter scale mode
**And** the geometry scales from selection center as I move mouse

**Given** I am in scale mode
**When** I click to confirm
**Then** the scale is applied

**Given** I am in scale mode
**When** I press Escape or right-click
**Then** the scale is cancelled
**And** the geometry returns to its original size

**Technical Notes:**
- Scale around selection centroid
- Uniform scale based on mouse distance from center

**Test Requirements:**
- Unit test: ScaleVertices command roundtrip (execute → undo → verify original)
- Unit test: ScaleVertices with empty selection returns error
- Unit test: inverse transform produces exact restoration

---

### Story 3.4: Grid Snapping

**As a** Godot developer,
**I want** geometry to snap to a grid during transforms,
**So that** I can create aligned, precise blockouts.

**Acceptance Criteria:**

**Given** I am moving geometry with grid snap enabled (default)
**When** I drag the geometry
**Then** positions snap to 1m grid increments
**And** visual feedback shows snapped position

**Given** I am in a transform operation
**When** I hold Shift
**Then** snapping is temporarily disabled
**And** I can position geometry freely

**Given** I release Shift during a transform
**When** I continue dragging
**Then** snapping re-enables
**And** geometry snaps to grid again

**Technical Notes:**
- Default grid size: 1m
- Grid snap applies to Move operations
- Snap indicator shows momentary feedback when snap occurs

---

### Story 3.5: Full Undo/Redo Integration

**As a** Godot developer,
**I want** to undo and redo all transform operations,
**So that** I can correct mistakes and experiment freely.

**Acceptance Criteria:**

**Given** I have performed a move operation
**When** I press Ctrl+Z (undo)
**Then** the geometry returns to its previous position exactly

**Given** I have undone a move operation
**When** I press Ctrl+Shift+Z (redo)
**Then** the geometry returns to the moved position exactly

**Given** I have performed multiple operations (move, rotate, scale)
**When** I undo multiple times
**Then** each operation is undone in reverse order

**Given** I have performed an operation
**When** I use Edit menu → Undo
**Then** the Godot native undo works correctly with blockot operations

**Technical Notes:**
- Full integration with EditorUndoRedoManager
- All transform commands implement Command trait with execute/undo
- Command history cleared on scene reload (per Architecture)

**Test Requirements:**
- Integration test: 5-operation undo chain restores original state exactly
- Integration test: undo/redo interleaving works correctly (undo 2, redo 1, undo 1)
- Integration test: scene reload clears command history

---

## Epic 4: Distribution & CI Pipeline

**Goal:** Developer can build cross-platform releases and share them with testers. Automated CI runs clippy, tests, and produces Windows/macOS/Linux binaries.

**FRs covered:** *(Architecture: CI/CD, cross-platform builds)*
**Notes:** Gated epic — required before Epic 5. Enables consistent test builds for Brian.

### Story 4.1: GitHub Actions CI Pipeline

**As a** developer,
**I want** automated CI that runs on every push,
**So that** I catch build and test failures early.

**Acceptance Criteria:**

**Given** I push code to the repository
**When** GitHub Actions runs
**Then** `cargo clippy` runs and reports any warnings
**And** `cargo test` runs and reports test results
**And** the pipeline fails if clippy or tests fail

**Given** a pull request is opened
**When** CI completes
**Then** the PR shows pass/fail status
**And** I can see detailed logs for any failures

**Technical Notes:**
- `.github/workflows/test.yml`
- Runs on: push to main, pull requests
- Uses stable Rust toolchain

---

### Story 4.2: Cross-Platform Build Pipeline

**As a** developer,
**I want** CI to produce binaries for Windows, macOS, and Linux,
**So that** testers on any platform can use the plugin.

**Acceptance Criteria:**

**Given** CI runs on main branch
**When** the build step completes
**Then** artifacts are produced for:
- `libblockot.linux.x86_64.so`
- `blockot.windows.x86_64.dll`
- `libblockot.macos.universal.dylib`

**Given** a release is tagged (e.g., v0.1.0)
**When** the release workflow runs
**Then** a GitHub Release is created
**And** platform binaries are attached as downloadable assets

**Technical Notes:**
- `.github/workflows/build.yml` for builds
- `.github/workflows/release.yml` for releases
- Cross-compilation using cross-rs or native runners

---

### Story 4.3: Installation Documentation

**As a** tester,
**I want** clear instructions for installing the plugin,
**So that** I can start testing without developer help.

**Acceptance Criteria:**

**Given** I download the release zip
**When** I follow the README instructions
**Then** I can install the plugin by copying `addons/blockot/` to my project

**Given** the plugin is installed
**When** I enable it in Project Settings → Plugins
**Then** the plugin loads without errors

**Given** I am a new user
**When** I read the README
**Then** I understand:
- Installation steps
- How to add a BlockotNode
- Basic keyboard shortcuts (Tab, G, R, S)

---

## Epic 5: Geometry Operations

**Goal:** User can extrude faces to create new geometry and cut/bisect to subdivide shapes. Actual mesh modeling is now possible.

**FRs covered:** FR20, FR21

### Story 5.1: Extrude Faces

**As a** Godot developer,
**I want** to press E and drag to extrude selected faces,
**So that** I can create new geometry from existing faces.

**Acceptance Criteria:**

**Given** I have one or more faces selected in edit mode
**When** I press E
**Then** I enter extrude mode
**And** new geometry is created by extruding the selected faces
**And** the new faces follow my mouse cursor

**Given** I am in extrude mode
**When** I click to confirm
**Then** the extrusion is committed
**And** the new geometry becomes part of the mesh

**Given** I am in extrude mode
**When** I press Escape or right-click
**Then** the extrusion is cancelled
**And** the mesh returns to its original state (no new geometry)

**Given** I extrude and confirm
**When** I press Ctrl+Z
**Then** the extrusion is undone
**And** the created vertices and faces are removed

**Technical Notes:**
- ExtrudeFaces command tracks created_vertices and created_faces for undo
- Default extrude direction: face normal
- Grid snap applies to extrude distance

**Test Requirements:**
- Unit test: ExtrudeFaces command roundtrip (execute → undo removes created geometry)
- Unit test: ExtrudeFaces with no faces selected returns error
- Unit test: created_vertices and created_faces tracking is accurate
- Integration test: extrude + undo restores exact original topology

---

### Story 5.2: Cut/Bisect Tool

**As a** Godot developer,
**I want** to press C and draw a line to cut faces,
**So that** I can subdivide geometry for more detail.

**Acceptance Criteria:**

**Given** I am in edit mode
**When** I press C
**Then** I enter cut mode
**And** a visual line follows my cursor

**Given** I am in cut mode
**When** I click and drag across faces
**Then** a cut line is previewed across the faces

**Given** a cut line is previewed
**When** I release to confirm
**Then** the faces are bisected along the cut line
**And** new edges and vertices are created

**Given** I am in cut mode
**When** I press Escape or right-click
**Then** cut mode is cancelled
**And** no changes are made

**Given** I have made a cut
**When** I press Ctrl+Z
**Then** the cut is undone
**And** the mesh returns to its pre-cut state

**Technical Notes:**
- Cut creates new vertices at intersection points
- Affected faces are split into multiple faces
- Edge-only cuts also supported

**Test Requirements:**
- Unit test: CutEdge command roundtrip (execute → undo restores original topology)
- Unit test: cut across single face creates correct vertex/edge count
- Unit test: cut across multiple faces handles shared edges correctly
- Integration test: cut + undo restores exact original topology

---

## Epic 6: Advanced Snapping

**Goal:** User can snap geometry to nearby vertices/edges/faces (proximity snap), temporarily disable snapping with Shift, and configure grid size.

**FRs covered:** FR23, FR24, FR25
**Architecture Note:** SnapProvider trait abstraction enables future cross-node snapping without core rewrite.

### Story 6.1: Proximity Snapping

**As a** Godot developer,
**I want** geometry to snap to nearby vertices and edges,
**So that** I can align geometry precisely without manual positioning.

**Acceptance Criteria:**

**Given** I am moving geometry with proximity snap enabled
**When** a vertex approaches another vertex within snap threshold
**Then** it snaps to that vertex position
**And** visual feedback indicates the snap occurred

**Given** I am moving geometry
**When** a vertex approaches an edge within snap threshold
**Then** it snaps to the nearest point on that edge

**Given** proximity snap is working
**When** I also have grid snap enabled
**Then** proximity snap takes priority when within threshold
**And** grid snap applies otherwise

**Technical Notes:**
- Implements SnapProvider trait (LocalSnapProvider for MVP)
- Snap threshold configurable (default: 0.1m)
- Designed for future SceneSnapProvider (cross-node snapping)

---

### Story 6.2: Snap Toggle with Shift

**As a** Godot developer,
**I want** to hold Shift to disable all snapping temporarily,
**So that** I can make fine adjustments when needed.

**Acceptance Criteria:**

**Given** I am in a transform operation with snapping enabled
**When** I hold Shift
**Then** both grid snap and proximity snap are disabled
**And** geometry moves freely with mouse

**Given** I am holding Shift during a transform
**When** I release Shift
**Then** snapping re-enables immediately
**And** geometry snaps if near a snap target

**Technical Notes:**
- Shift modifier checked in input_handler.rs
- Applies to Move, Rotate, Scale, Extrude operations

---

### Story 6.3: Configurable Grid Size

**As a** Godot developer,
**I want** to configure the grid snap size,
**So that** I can work at different scales.

**Acceptance Criteria:**

**Given** a BlockotNode is selected
**When** I look in the Inspector panel
**Then** I see a "Grid Size" property (default: 1.0m)

**Given** I change the grid size to 0.5m
**When** I perform a move operation
**Then** geometry snaps to 0.5m increments

**Given** I change the grid size to 2.0m
**When** I perform a move operation
**Then** geometry snaps to 2.0m increments

**Technical Notes:**
- Grid size stored per-node or as editor setting (TBD)
- Exposed via Inspector (#[export] property)

---

## Epic 7: Primitives & Properties

**Goal:** User can create all primitive types (Plane, Cylinder, Sphere) and configure face direction and collision settings. Completes the MVP feature set.

**FRs covered:** FR3, FR4, FR5, FR26, FR27, FR28, FR29
**Notes:** All 4 primitives ship together — no splitting. Atomic delivery.

### Story 7.1: Plane Primitive

**As a** Godot developer,
**I want** to create a Plane primitive,
**So that** I can build floors, walls, and flat surfaces.

**Acceptance Criteria:**

**Given** I add a new BlockotNode
**When** I select "Plane" as the primitive type
**Then** a 1m x 1m plane is created
**And** the plane is visible in the viewport

**Given** a Plane primitive exists
**When** I enter edit mode
**Then** I can select and manipulate its vertices/edges/face

**Technical Notes:**
- Plane: 4 vertices, 1 quad face
- Default orientation: XZ plane (horizontal floor)

---

### Story 7.2: Cylinder Primitive

**As a** Godot developer,
**I want** to create a Cylinder primitive,
**So that** I can build columns, pillars, and rounded shapes.

**Acceptance Criteria:**

**Given** I add a new BlockotNode
**When** I select "Cylinder" as the primitive type
**Then** a cylinder is created (1m diameter, 1m height)
**And** the cylinder is visible in the viewport

**Given** a Cylinder primitive exists
**When** I enter edit mode
**Then** I can select and manipulate its vertices/edges/faces

**Technical Notes:**
- Cylinder: configurable segment count (default: 8 or 12)
- Includes top and bottom cap faces

---

### Story 7.3: Sphere Primitive

**As a** Godot developer,
**I want** to create a Sphere primitive,
**So that** I can build domes and curved shapes.

**Acceptance Criteria:**

**Given** I add a new BlockotNode
**When** I select "Sphere" as the primitive type
**Then** a sphere is created (1m diameter)
**And** the sphere is visible in the viewport

**Given** a Sphere primitive exists
**When** I enter edit mode
**Then** I can select and manipulate its vertices/edges/faces

**Technical Notes:**
- Sphere: UV sphere with configurable segments/rings
- Default: low-poly for blockout purposes (8x6 or similar)

---

### Story 7.4: Face Direction Property

**As a** Godot developer,
**I want** to set face direction (Outward, Inward, Both),
**So that** I can control how faces render and collide.

**Acceptance Criteria:**

**Given** a BlockotNode is selected
**When** I look in the Inspector
**Then** I see a "Face Direction" property with options: Outward, Inward, Both

**Given** Face Direction is set to "Outward" (default)
**When** I view the geometry
**Then** only outward-facing sides are visible
**And** backfaces are culled

**Given** Face Direction is set to "Inward"
**When** I view the geometry
**Then** only inward-facing sides are visible (for interior rooms)

**Given** Face Direction is set to "Both"
**When** I view the geometry
**Then** both sides of faces are visible (double-sided)

**Technical Notes:**
- Affects ArrayMesh generation (normal direction)
- Stored in geometry/properties.rs as FaceDirection enum

---

### Story 7.5: Collision Toggle

**As a** Godot developer,
**I want** to enable or disable collision generation,
**So that** I can control whether geometry is walkable/collidable.

**Acceptance Criteria:**

**Given** a BlockotNode is selected
**When** I look in the Inspector
**Then** I see a "Generate Collision" toggle (default: enabled)
**And** I see a "Collision Type" dropdown: Convex, Concave

**Given** collision is enabled
**When** I exit edit mode or trigger collision rebuild
**Then** a collision shape is generated matching the geometry

**Given** collision type is "Convex"
**When** collision is generated
**Then** a convex hull collision shape is created (fast, approximate)

**Given** collision type is "Concave"
**When** collision is generated
**Then** a trimesh collision shape is created (accurate, slower)

**Given** collision is disabled
**When** I run the game
**Then** no collision shape exists for this BlockotNode

**Technical Notes:**
- Collision rebuilt on edit mode exit (deferred rebuild per Architecture)
- Uses Godot's create_convex_collision / create_trimesh_collision

