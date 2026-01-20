---
stepsCompleted: [1, 2, 3, 4]
inputDocuments: []
session_topic: 'Editing workflow optimization for blockot MVP - Godot level blockout plugin'
session_goals: 'Define MVP features, optimize editing workflow, establish technical approach'
selected_approach: 'ai-recommended'
techniques_used: ['cross-pollination', 'role-playing', 'first-principles-thinking']
ideas_generated: 38
context_file: 'project-context-template.md'
session_completed: true
---

# Brainstorming Session: blockot MVP

**Facilitator:** You
**Date:** 2026-01-19

## Session Overview

**Topic:** Editing workflow optimization for blockot MVP - a Godot level blockout plugin inspired by ProBuilder and BSP brushes

**Goals:**
- Define MVP feature set (beginner-focused, simple to learn)
- Optimize the editing workflow (fast, doesn't get in the way)
- Establish technical approach
- Draw inspiration from existing tools (ProBuilder, BSP, Blender)

### Context Guidance

This is a software/product development brainstorm focusing on:
- User workflow and pain points in level blockout
- Feature ideas prioritized for MVP simplicity
- Technical approaches for Godot plugin architecture
- UX patterns that minimize friction

### Key Constraints (Pre-Decided)

| Constraint | Decision |
|------------|----------|
| Grid snap default | 1m (configurable) |
| Fine-tuning | Modifier key to escape grid |
| Keymapping | Blender-style default |
| Materials | Not in MVP - pure 3D modeling |
| Target user | Beginners, simple to learn |

### Core Mantra

> *"Fast workflow that doesn't get in the way"*

---

## Technique Selection

**Approach:** AI-Recommended Techniques
**Analysis Context:** Workflow optimization for beginner-friendly Godot blockout plugin MVP

**Recommended Technique Sequence:**

1. **Cross-Pollination** *(creative)* - Mine ProBuilder, BSP, Blender for proven workflow patterns
2. **Role Playing** *(collaborative)* - Embody beginner personas to filter and prioritize features
3. **First Principles Thinking** *(creative)* - Distill to irreducible MVP core

**AI Rationale:** This sequence follows an Inspiration → Empathy → Distillation path, starting with what works in existing tools, filtering through user needs, and crystallizing into an actionable MVP feature set.

---

## Phase 1: Cross-Pollination Results

**Source Tools Mined:** Blender, ProBuilder, Hammer/BSP editors

**Key Patterns Extracted:**

| ID | Pattern | Insight |
|----|---------|---------|
| Workflow #1 | Left-Hand Sovereignty | All shortcuts from left hand, right stays on mouse |
| Workflow #2 | Zero-Interrupt Building | No dialogs, no context switches, thought → geometry |
| Workflow #3 | Purposeful Minimalism | Few tools, each earns its place |
| Workflow #4 | Contextual Geometry Snapping | Snap to existing geometry, not just world grid |
| Workflow #5 | Reference Matching | Sample dimensions from other geometry |
| Workflow #7 | QWE Transform Trinity | Blender-style G/R/S (or Q/W/E) |
| Workflow #9 | Transient Operation Mode | Key activates, mouse executes, click confirms |
| Workflow #11-14 | Doorway Creation Flow | Slice → Extrude inward → Snap through |
| Workflow #15 | Move vs Extrude | Move repositions, Extrude creates geometry |
| Workflow #16 | Full Transform Support | G/R/S all work, skewing is expected |

**Technical Decisions:**

| ID | Decision | Rationale |
|----|----------|-----------|
| Technical #3 | BlockotNode as Universal Primitive | One node type, many behaviors |
| Technical #4 | Face Direction Mode (Out/In/Both) | Room = flipped faces, not special type |
| Technical #5 | Always-On Collision (Default) | Place block = it collides |
| Technical #6 | No Wall Thickness (MVP) | Forces clear inside/outside thinking |
| Technical #9 | Explicit Edit Mode (Tab) | Clean boundary with Godot editor |
| Technical #10 | Object Mode = Godot Native | Blockot adds, never replaces |
| Technical #12 | Undo/Redo (Critical) | Non-negotiable safety net |
| Technical #14-15 | Extends MeshInstance3D | Seamless Godot integration |
| Technical #16-17 | No Export Needed, Collision Toggle | It IS the final geometry |

---

## Phase 2: Role Playing Results

**Personas Tested:**

### Fresh Finn (True Beginner)
- Learned core loop from one extrude action
- Contextual hints made shortcuts discoverable
- Doorway workflow felt achievable

### Blender Brian (Muscle Memory)
- Tab for edit mode works as expected
- G/R/S behave correctly
- Context-aware transforms feel natural

### Unity Uma (ProBuilder User)
- Proximity snapping makes workflow workable
- Missing features (bevel, inset, etc.) acceptable for MVP
- Shift-to-escape-snap is intuitive

### Godot Gina (Engine Native)
- BlockotNode = MeshInstance3D with superpowers
- Zero learning curve at object level
- Merge/split enables flexible workflows

**UX Patterns Validated:**

| ID | Pattern | Purpose |
|----|---------|---------|
| UX #1 | Sensible Defaults | Face pre-selected on edit mode entry |
| UX #3 | Contextual Hint Bar | Shows controls during operations |
| UX #4 | Mouse Capture Mode | Signals active operation |
| UX #5 | Normal-Relative Movement | Extrude follows face direction |
| UX #6 | Persistent Grid Indicator | Always visible, clickable |
| UX #8 | Instant Cheatsheet | Single key shows all shortcuts |
| UX #11-13 | Snap System | Proximity + highlight + Shift to escape |

---

## Phase 3: First Principles MVP Definition

**The irreducible blockot for "fast workflow that doesn't get in the way":**

### Core Architecture
| Element | Specification |
|---------|---------------|
| Node type | BlockotNode (extends MeshInstance3D) |
| Primitives | Box (default 1m), Plane, Cylinder, Sphere |
| Selection modes | Vertex, Edge, Face |
| Edit mode | Tab to enter/exit |

### MVP Tools (5 total)
| Tool | Key | Function |
|------|-----|----------|
| Move | G | Reposition selected geometry |
| Rotate | R | Rotate selected geometry |
| Scale | S | Scale selected geometry |
| Extrude | E | Create new geometry outward/inward |
| Cut | C | Bisect face/edge with straight line |

### Snapping System
| Type | Behavior |
|------|----------|
| Grid | 1m default, configurable, always visible in top bar |
| Proximity | Snap to nearby vertex/edge/face while dragging, highlight shows target |
| Shift modifier | Disables all snapping for free movement |

### BlockotNode Properties
| Property | Default | Options |
|----------|---------|---------|
| Face direction | Outward | Outward / Inward / Both |
| Collision | Enabled | Toggle on/off |

### Godot Integration
| Aspect | Behavior |
|--------|----------|
| Inheritance | Extends MeshInstance3D - all Godot features work |
| Object mode | Normal Godot editor (transforms, parenting, inspector) |
| Edit mode | Blockot tools active, Tab to toggle |
| Undo/redo | Full support, integrates with Godot's undo system |
| Shortcuts | Left-hand accessible, Blender-style (configurable later) |

### Deferred to Post-MVP
- Loop cuts, shape cuts (preset door/window shapes)
- Bevel, Inset, Bridge tools
- Edge loop selection
- Merge/Split BlockotNodes
- Export to OBJ/GLTF (Godot handles this natively)
- Contextual hint overlays
- Cheatsheet overlay
- Numerical input during transforms
- Additional primitives (stairs, wedge, arch)

---

## Session Summary and Next Steps

### Thematic Organization

**Theme 1: Core Interaction Model**
*Focus: How users interact with geometry*
- Transient operation mode (key → drag → confirm)
- Left-hand shortcuts, right hand on mouse
- Tab to enter/exit edit mode
- Vertex/Edge/Face selection modes
- Blender-style G/R/S transforms

**Theme 2: Snapping System**
*Focus: Precision without friction*
- Grid snap (1m default, configurable)
- Proximity snap (vertex/edge/face while dragging)
- Snap target highlighting
- Shift = all snapping disabled

**Theme 3: Godot Integration**
*Focus: Seamless engine fit*
- BlockotNode extends MeshInstance3D
- All Godot features work (parenting, inspector, collision layers)
- Object mode = normal Godot
- Edit mode = Blockot tools
- No export step needed - it IS the geometry

**Theme 4: Geometry Operations**
*Focus: What you can do to shapes*
- Move/Rotate/Scale (transforms)
- Extrude (create new geometry)
- Cut (bisect faces/edges)
- Face direction (outward/inward/both)

**Theme 5: Beginner Experience**
*Focus: Easy to learn, fast to master*
- Sensible defaults (face pre-selected, grid visible)
- Collision on by default
- Few tools, each with clear purpose
- Consistent interaction patterns

---

### Prioritized MVP Scope

**Must Have (MVP):**
1. BlockotNode (extends MeshInstance3D)
2. Four primitives: Box, Plane, Cylinder, Sphere
3. Five tools: Move (G), Rotate (R), Scale (S), Extrude (E), Cut (C)
4. Three selection modes: Vertex, Edge, Face
5. Grid snap + proximity snap + Shift override
6. Face direction toggle (outward/inward/both)
7. Collision toggle (default on)
8. Tab to enter/exit edit mode
9. Undo/redo support

**Nice to Have (Post-MVP):**
- Shape cuts (door/window presets)
- Loop cuts, bevel, inset, bridge
- Merge/split BlockotNodes
- Contextual hints, cheatsheet overlay
- Numerical input during transforms
- Additional primitives (stairs, wedge)

---

### Action Plan

**Immediate Next Steps:**

1. **Product Brief** - Take this brainstorm output into the Product Brief workflow to formalize the vision
2. **Research** - Investigate Godot EditorPlugin architecture for implementing BlockotNode and edit mode
3. **PRD** - Create detailed requirements from this MVP definition

**Technical Investigation Needed:**
- How to extend MeshInstance3D with custom editor behavior
- ArrayMesh generation for primitives
- Godot undo/redo integration
- Inspector panel customization for BlockotNode properties

---

### Session Achievements

- **38 ideas** captured across workflow, UX, and technical domains
- **4 personas** validated (beginner, Blender user, ProBuilder user, Godot native)
- **Irreducible MVP** defined through First Principles
- **Clear boundary** between MVP and post-MVP features
- **Core mantra** established: "Fast workflow that doesn't get in the way"

---

### Key Insights

1. **Blockot should be invisible** - It's just a MeshInstance3D until you Tab into edit mode
2. **Five tools are enough** - Move, Rotate, Scale, Extrude, Cut cover all MVP needs
3. **Snapping is essential** - Grid + proximity with Shift escape handles all precision needs
4. **Face direction replaces "room" concept** - No special room type, just flip the faces
5. **Left-hand sovereignty** - All shortcuts accessible without moving right hand from mouse

---

*Session completed: 2026-01-19*
*Facilitator: Mary (Business Analyst)*
*Techniques: Cross-Pollination, Role Playing, First Principles Thinking*

