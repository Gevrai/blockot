---
stepsCompleted: ['step-01-init', 'step-02-discovery', 'step-03-success', 'step-04-journeys', 'step-05-domain', 'step-06-innovation', 'step-07-project-type', 'step-08-scoping', 'step-09-functional', 'step-10-nonfunctional', 'step-11-polish', 'step-12-complete']
completedAt: '2026-01-20'
classification:
  projectType: developer_tool
  domain: general
  complexity: low
  projectContext: greenfield
inputDocuments:
  - _bmad-output/planning-artifacts/product-brief-blockot-2026-01-20.md
  - _bmad-output/analysis/brainstorming-session-2026-01-19.md
documentCounts:
  briefCount: 1
  researchCount: 0
  brainstormingCount: 1
  projectDocsCount: 0
workflowType: 'prd'
---

# Product Requirements Document - blockot

**Author:** You
**Date:** 2026-01-20

## Executive Summary

**blockot** is a Godot EditorPlugin for rapid level blockout and greyboxing. It brings Blender-familiar editing (G/R/S shortcuts, extrude, cut) directly into the Godot editor, eliminating the friction of round-tripping between tools.

**Core Differentiator:** Fast, keyboard-driven workflow that stays out of your way. Build a room, hit F5, playtest immediately.

**Target Users:** Godot developers who want native in-editor geometry editing, and Blender users who want their muscle memory to transfer.

**Project Type:** Personal/open-source tool. Success metric: "I use it and don't hate it."

## Success Criteria

### User Success

| Signal | Indicator |
|--------|-----------|
| **Flow improvement** | Blockout feels faster and more fun than Blender round-tripping |
| **Instinctive reach** | Reaching for blockot instead of Blender becomes automatic |
| **Collaborator onboarding** | Brian can use it after a quick walkthrough |
| **Gamejam viability** | Can use blockot during a jam without reverting to Blender |

**Primary success metric:** "Can I use it myself without cursing?"

### Business Success

Personal/open-source project. No external adoption metrics required.

**Success metric:** "I use it and don't hate it."

### Technical Success

| Aspect | Requirement |
|--------|-------------|
| **Responsiveness** | No perceptible lag during transform operations |
| **Performance** | No major editor slowdown with typical blockout geometry |
| **Reliability** | Undo/redo works correctly; no data loss |

**Priority:** Fast and fun workflow takes precedence over raw performance optimization.

### Anti-Metrics (Failure Signals)

| Signal | Meaning |
|--------|---------|
| Reverted to Blender mid-jam | Core workflow is broken or too slow |
| Brian gave up after walkthrough | Interaction model isn't intuitive enough |
| Avoided using blockot for "real" work | Trust in the tool hasn't developed |

## Product Scope

### MVP (Phase 1)

**Core Deliverables:**
- Editable geometry (BlockotNode OR BlockotMesh resource — TBD in Architecture)
- 5 tools: Move (G), Rotate (R), Scale (S), Extrude (E), Cut (C)
- 4 primitives: Box, Plane, Cylinder, Sphere
- 3 selection modes: Vertex, Edge, Face
- Snapping: Grid (1m default) + Proximity + Shift to disable
- Face direction toggle (Outward/Inward/Both)
- Collision toggle (default on)
- Tab to enter/exit edit mode
- Full undo/redo integration
- README documentation

**MVP Success Gate:** Can build a room with columns and corridors, then immediately playtest.

**Technical Decision (TBD in Architecture):**

| Option | Approach | Trade-offs |
|--------|----------|------------|
| **BlockotNode** | Custom node extending MeshInstance3D | Cleaner UX; more Godot coupling |
| **BlockotMesh** | Resource in standard MeshInstance3D | More flexible; may complicate edit mode |

### Growth Features (Phase 2)

- Shape cuts (door/window presets)
- Loop cuts
- Bevel, Inset, Bridge tools
- Merge/Split BlockotNodes
- In-editor tooltips
- Numerical input during transforms

### Vision (Phase 3)

- Additional primitives (stairs, wedge, arch)
- Scripting API for procedural generation
- Tutorials and broader documentation
- v1.0 stability if traction develops
- Potential Godot Asset Library listing

### Risk Mitigation

| Risk Type | Risk | Mitigation |
|-----------|------|------------|
| **Technical** | EditorPlugin API complexity | Start simple; iterate |
| **Technical** | Undo/redo integration | Prototype early (critical path) |
| **Resource** | Solo developer, limited time | Tight MVP scope; no feature creep |
| **Resource** | Motivation decay | Ship fast; use in a real jam |

## User Journeys

### Journey 1: Gina's First Use — "Finally, I Can Stay in Godot"

**Opening Scene:**
Gina is prototyping a dungeon crawler. She's built the player controller, the combat system feels good, but now she needs to build a level. She sighs, knowing the next hour will be spent in Blender making boxes, exporting, reimporting, realizing the scale is wrong, going back to Blender...

**Rising Action:**
She remembers installing blockot. She adds a BlockotNode, presses Tab, and enters edit mode. She presses G—the face moves. Just like Blender. She presses E—it extrudes. She drags out a corridor, adds a column, shapes a doorway. Grid snapping keeps everything aligned.

**Climax:**
She presses F5. The game runs. She's standing in her corridor. It took 90 seconds from "I need a level" to "I'm playtesting it."

**Resolution:**
She tweaks the corridor width (Tab, select face, G, drag, Tab out, F5). The iteration loop is instant. Blockot disappears into the background—it's just how she builds now.

### Journey 2: Brian's Onboarding — "Oh, It Just Works Like Blender"

**Opening Scene:**
Brian joins Gina for a weekend jam. He's the Blender guy—can model anything, but never touched Godot's editor beyond placing nodes. Gina says "just use blockot for the blockout, I'll handle the code."

**Rising Action:**
Gina gives him a 2-minute walkthrough: "Add a BlockotNode, Tab to edit, G/R/S work like Blender, E to extrude, Shift disables snap." Brian nods—this is his language. He Tabs in, and his hands already know what to do.

**Climax:**
Brian doesn't ask any more questions. An hour later, he's blocked out three interconnected rooms with interesting verticality. He never opened Blender.

**Resolution:**
At the end of the jam, Brian says "I actually liked working in Godot this time." The muscle memory transfer worked.

### Journey 3: Gina's Gamejam Crunch — "It Didn't Break Under Pressure"

**Opening Scene:**
Hour 18 of a 48-hour jam. Gina's exhausted. The game needs three more rooms and the deadline is in 6 hours. In past jams, this is where Blender round-trips eat 20 minutes per iteration.

**Rising Action:**
She drops in a BlockotNode and starts building. Tab, extrude, snap, Tab out, test. The room's too small—Tab back in, scale the walls, Tab out, test again. No export dialogs. No "did I save the .blend?" anxiety.

**Climax:**
She finishes all three rooms with an hour to spare. She adds a secret alcove she thought of at hour 20 but assumed she'd have to cut.

**Resolution:**
The jam ends. The game ships. She never reverted to Blender. Blockot earned its place—not because it's powerful, but because it didn't slow her down when speed mattered most.

### Journey Requirements Summary

| Journey | Capabilities Revealed |
|---------|----------------------|
| **Gina's First Use** | Edit mode (Tab), Blender shortcuts (G/R/S/E), grid snapping, seamless play-test loop |
| **Brian's Onboarding** | Teachable in 2 minutes, muscle memory transfer, no Godot expertise required |
| **Gina's Gamejam** | Reliability under pressure, fast iteration, no export/import friction |

## Developer Tool Requirements

### Platform

| Aspect | Requirement |
|--------|-------------|
| **Engine Version** | Godot 4.0+ (may target later if needed) |
| **Target Users** | New projects; no legacy concerns |
| **Platform** | Cross-platform (wherever Godot 4 runs) |

### Installation

| Method | Priority | Notes |
|--------|----------|-------|
| **Addon folder** | MVP | Copy `addons/blockot` to project |
| **Godot Asset Library** | MVP | Easy discovery and installation |
| **Git submodule** | Post-MVP | Version-controlled workflows |

### API Surface

| Scope | Priority | Description |
|-------|----------|-------------|
| **Editor UI** | MVP | Full editing interface (Tab, G/R/S/E/C, snapping) |
| **Inspector properties** | MVP | Face direction, collision toggle |
| **Scripting API** | Post-MVP | Procedural generation, runtime mesh access |

### Documentation

| Phase | Approach |
|-------|----------|
| **MVP** | README (installation, usage, shortcuts) |
| **Post-MVP** | In-editor tooltips |
| **Future** | Tutorials if traction develops |

## Functional Requirements

### Geometry Creation

- **FR1:** User can add a blockot geometry node to a Godot scene
- **FR2:** User can create a Box primitive (default 1m cube)
- **FR3:** User can create a Plane primitive
- **FR4:** User can create a Cylinder primitive
- **FR5:** User can create a Sphere primitive

### Edit Mode

- **FR6:** User can enter edit mode on blockot geometry (Tab key)
- **FR7:** User can exit edit mode to normal Godot editor (Tab key)
- **FR8:** User can only edit one blockot geometry at a time

### Selection

- **FR9:** User can select individual vertices
- **FR10:** User can select individual edges
- **FR11:** User can select individual faces
- **FR12:** User can switch between vertex, edge, and face selection modes
- **FR13:** User can multi-select geometry elements (Ctrl+click)
- **FR14:** User can deselect all selected elements

### Transform Tools

- **FR15:** User can move selected geometry (G key)
- **FR16:** User can rotate selected geometry (R key)
- **FR17:** User can scale selected geometry (S key)
- **FR18:** User can confirm a transform operation (click)
- **FR19:** User can cancel a transform operation (Escape or right-click)

### Geometry Operations

- **FR20:** User can extrude selected faces/edges to create new geometry (E key)
- **FR21:** User can cut/bisect faces or edges with a straight line (C key)

### Snapping System

- **FR22:** User can snap geometry to a configurable grid (default 1m)
- **FR23:** User can snap geometry to nearby vertices/edges/faces (proximity snap)
- **FR24:** User can temporarily disable all snapping (Shift modifier)
- **FR25:** User can configure the grid snap size

### Geometry Properties

- **FR26:** User can set face direction to Outward
- **FR27:** User can set face direction to Inward
- **FR28:** User can set face direction to Both (double-sided)
- **FR29:** User can enable/disable collision generation (default: enabled)

### Editor Integration

- **FR30:** User can undo any edit operation
- **FR31:** User can redo any undone operation
- **FR32:** Blockot geometry persists correctly when scene is saved
- **FR33:** Blockot geometry loads correctly when scene is opened

## Non-Functional Requirements

### Performance

- **NFR1:** Transform operations (G/R/S) respond without perceptible lag (<100ms feedback)
- **NFR2:** Entering/exiting edit mode completes within 200ms
- **NFR3:** Snapping calculations cause no visible stutter during drag operations
- **NFR4:** Geometry with up to 500 faces remains responsive (typical blockout scale)

### Reliability

- **NFR5:** Undo/redo correctly restores all geometry state without corruption
- **NFR6:** Scene save persists all blockot geometry data without loss
- **NFR7:** Scene load restores geometry exactly as saved
- **NFR8:** Plugin does not crash Godot editor under normal usage

### Integration

- **NFR9:** Plugin follows Godot addon conventions (`addons/blockot/plugin.cfg`)
- **NFR10:** Plugin integrates with Godot's native undo/redo system
- **NFR11:** BlockotNode properties appear correctly in Godot's Inspector panel
- **NFR12:** Plugin does not interfere with standard Godot editor operations when not in edit mode
