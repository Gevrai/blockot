---
stepsCompleted: [1, 2, 3, 4, 5]
inputDocuments:
  - _bmad-output/analysis/brainstorming-session-2026-01-19.md
date: 2026-01-20
author: You
project_name: blockot
---

# Product Brief: blockot

## Executive Summary

**Efficient blockout in Godot: from idea to playtest in seconds.**

blockot is a low-overhead level blockout plugin for Godot that enables rapid greyboxing workflows without leaving the engine. Built for level designers who need to iterate fast—whether prototyping a full game or racing through a gamejam—blockot prioritizes flow-state preservation over feature completeness.

The core philosophy: **"Fast workflow that doesn't get in the way."**

Unlike Blender round-tripping or Godot's built-in CSG nodes, blockot provides native in-editor geometry editing where keys ARE tools and selections ARE context. No modes to remember. No toolbars to click. Shape it, play it, tweak it, ship it.

---

## Core Vision

### Problem Statement

Level designers and game developers using Godot lack a native, low-overhead tool for greyboxing and blockout workflows. This foundational step in level design—rapidly shaping spaces to test gameplay ideas—is currently solved through workarounds that break creative flow.

### Problem Impact

- **Blender round-tripping** pulls designers out of Godot, fragmenting attention between applications even when .blend imports are seamless
- **CSG nodes** are finicky to reshape, have odd boolean interactions, and carry significant performance overhead
- **Existing addons** (e.g., Cyclops) attempt to solve this but become bloated and struggle to keep pace with Godot engine updates
- **Result**: Prototyping feels slow and frustrating when it should be fast and fun. Iteration suffers. Ideas die before they're tested.

### Why Existing Solutions Fall Short

| Solution | Gap |
|----------|-----|
| Blender export | Context switching breaks design flow state |
| CSG nodes | No real geometry editing, performance heavy, boolean quirks |
| Existing addons | Feature bloat, poor engine version compatibility |
| Primitive meshes | No editing capability—just static placement |

The Godot ecosystem lacks a tool that is simultaneously **native**, **focused**, and **low-overhead**.

### Proposed Solution

blockot introduces a single node type—**BlockotNode**—that extends MeshInstance3D and provides intuitive geometry editing directly in the Godot editor. Users enter edit mode with Tab, shape geometry using keyboard-driven tools (G/R/S/E/C), and immediately playtest their levels.

Key characteristics:
- **Native integration**: BlockotNode IS a MeshInstance3D with editing superpowers
- **Low-overhead interaction**: Keys are tools, selections are context—no modes to remember
- **Blender-familiar shortcuts**: G/R/S muscle memory transfers directly, with openness to better patterns
- **Instant iteration**: No export, no conversion—from idea to playtest in seconds

### Key Differentiators

1. **Low-overhead, not low-power**: Minimal friction for all skill levels—beginners learn fast, experts stay in flow
2. **Native Godot citizen**: Extends MeshInstance3D—all Godot features work automatically
3. **Flow-state preservation**: Zero context switching, keyboard-centric, no toolbar hunting
4. **Blender-first, pragmatism-always**: Familiar shortcuts for the Godot+Blender crowd, open to better patterns
5. **Velocity over ceremony**: Pre-1.0 prioritizes shipping and learning over stability guarantees

---

## Target Users

### Primary Users

**Godot Gina** — Engine Native
- **Profile**: Experienced Godot developer who knows the engine deeply but lacks native blockout tools
- **Context**: Building games solo or in small teams, often under gamejam time pressure
- **Current pain**: Relies on Blender round-tripping or clunky CSG nodes; wants to stay in Godot
- **Success criteria**: "I shaped a room, hit play, and tested it without leaving the editor"
- **Represents**: The creator — you

**Blender Brian** — Muscle Memory Migrant
- **Profile**: Developer with strong Blender background, G/R/S shortcuts are instinct
- **Context**: Uses Blender for assets but wants blockout inside Godot for faster iteration
- **Current pain**: Context switching between Blender and Godot breaks flow
- **Success criteria**: "Tab to edit, G to move — it just works like I expect"
- **Represents**: Your collaborator/friend

### Secondary Users (Post-MVP Consideration)

**Fresh Finn** — True Beginner
- Needs more guidance and discoverability
- Deprioritized for MVP; low-overhead design naturally helps but won't optimize for hand-holding yet

**Unity Uma** — ProBuilder Migrant
- Expects ProBuilder-like patterns
- Deprioritized for MVP; Blender-first approach takes precedence

**Collaborators/Teammates**
- May tweak geometry in shared projects
- Benefit from git-friendly scene files (.tscn) — no special export/import needed

### User Journey

| Stage | Experience |
|-------|------------|
| **Discovery** | Word of mouth among Godot+Blender community |
| **Onboarding** | README in project; install addon, read basics, start using |
| **First use** | Add BlockotNode, Tab in, shape geometry — feels immediately familiar |
| **Aha moment** | The edit-play loop: shape a space, hit F5, playtest in seconds |
| **Long-term** | Blockot becomes default starting point for every level prototype |

**Documentation Strategy:**
- Pre-1.0: READMEs and inline documentation sufficient
- Near 1.0: Simple tutorial for broader adoption

---

## Success Metrics

### User Success

**Primary success criteria**: "Can I use it myself without cursing?"

| Signal | Indicator |
|--------|-----------|
| **Flow improvement** | Blockout feels faster and more fun than Blender round-tripping |
| **Instinctive reach** | Reaching for blockot instead of Blender becomes automatic |
| **Collaborator onboarding** | Brian can use it after a quick walkthrough (self-teaching not required) |
| **Gamejam survival** | Complete a jam without abandoning blockot mid-project |

### Project Milestones

**MVP Complete**
- Can build a room with columns and corridors
- Edit-play loop works without friction
- 5 tools functional: Move (G), Rotate (R), Scale (S), Extrude (E), Cut (C)
- Grid snap + proximity snap operational

**First Real Test**
- Next gamejam (approximately 1 month out)
- Success = used blockot for level blockout without reverting to Blender

**v1.0 / Stability**
- Not a current priority
- May be revisited if project gains traction and isn't abandoned
- No formal stability guarantees pre-1.0

### Anti-Metrics (Failure Signals)

| Failure Signal | What It Means |
|----------------|---------------|
| Reverted to Blender mid-jam | Core workflow is broken or too slow |
| Brian gave up after walkthrough | Interaction model isn't intuitive enough |
| Avoided using blockot for "real" work | Trust in the tool hasn't developed |

### Public Release Strategy

- Shared publicly from the start
- Clear "not stable software" warning
- No support guarantees; feedback welcome but not obligated

---

## MVP Scope

### Core Features

**BlockotNode / Mesh Editing**
- Single node type or mesh resource for editable geometry (technical approach TBD in architecture)
- Extends or integrates with Godot's existing mesh system
- Tab to enter/exit edit mode
- Full undo/redo integration with Godot's system

**Tools (5)**

| Tool | Key | Function |
|------|-----|----------|
| Move | G | Reposition selected geometry |
| Rotate | R | Rotate selected geometry |
| Scale | S | Scale selected geometry |
| Extrude | E | Create new geometry outward/inward |
| Cut | C | Bisect face/edge with straight line |

**Primitives (4)**
- Box (default 1m cube)
- Plane
- Cylinder
- Sphere

**Selection System**
- Three modes: Vertex, Edge, Face
- Multi-select with Ctrl+click
- Selection modes share same foundation (face = selected vertices of face)

**Snapping System**

| Type | Behavior |
|------|----------|
| Grid | 1m default, configurable |
| Proximity | Snap to nearby vertex/edge/face while dragging |
| Shift | Disables all snapping for free movement |

**BlockotNode Properties**
- Face direction: Outward / Inward / Both
- Collision: Toggle (default on)

### Out of Scope for MVP

**Deferred Tools**
- Loop cuts
- Shape cuts (door/window presets) — noted for post-MVP
- Bevel, Inset, Bridge
- Numerical input during transforms

**Deferred Features**
- Edge loop selection
- Merge/Split nodes
- Additional primitives (stairs, wedge, arch)
- Contextual hint overlays
- Cheatsheet overlay

**Explicitly Not Planned**
- Materials/texturing (pure geometry focus)
- Export to external formats (Godot handles natively)

### MVP Success Criteria

- Can build a room with columns and corridors
- Edit-play loop works without friction
- All 5 tools functional and keyboard-driven
- Multi-select works intuitively
- Grid + proximity snapping operational
- Survives gamejam usage without reverting to Blender

### Future Vision

**Post-MVP Enhancements**
- Shape cuts for faster doorway/window creation
- Loop cuts for detail work
- Bevel/Inset for geometry refinement
- Additional primitives (stairs, wedges, arches)

**If Traction Develops**
- Broader documentation and tutorials
- Community feedback integration
- Stability improvements toward v1.0
- Potential asset library listing

### Technical Investigation (For Architecture Phase)

- **MeshInstance3D extension vs Mesh Resource approach**: Evaluate which integrates better with Godot's model and provides cleaner API
