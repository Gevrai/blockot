---
stepsCompleted: ['lightweight-pass']
completedAt: '2026-01-24'
approach: lightweight
inputDocuments:
  - _bmad-output/planning-artifacts/prd.md
---

# UX Design Specification - blockot

**Author:** You
**Date:** 2026-01-24
**Approach:** Lightweight pass (EditorPlugin - inherits Godot editor theme)

---

## Overview

blockot is a Godot EditorPlugin. It inherits the editor's visual theme and focuses on keyboard-driven interaction. This spec covers the custom visual feedback elements needed beyond Godot's defaults.

## Edit Mode Visual State

**Decision:** Handle appearance is sufficient indicator.

When user enters edit mode (Tab), the appearance of vertex/edge/face handles on the geometry communicates the mode change. No additional viewport border, tint, or overlay needed.

**Note:** Revisit after initial implementation if users find it unclear.

## Selection Highlighting

**Decision:** Use Godot's default selection colors.

- Inherit Godot editor's existing selection color scheme (typically orange/white)
- Maintains visual consistency with the rest of the editor
- No custom palette needed

## Transform Feedback

**Decision:** Real-time geometry movement with axis constraint visualization.

| Element | Behavior |
|---------|----------|
| **Geometry** | Moves/rotates/scales in real-time as user drags |
| **Axis lines** | Display constraint axis when active (e.g., X/Y/Z lock) |
| **Default constraint** | Face normal for face operations |

No ghost geometry or numeric readout in MVP. Geometry moves live.

## Snap Indicators

**Decision:** Visual highlight when snap occurs.

- When geometry snaps to grid or proximity target, provide momentary visual feedback
- Snap point/line highlights briefly on snap
- No persistent grid plane visualization during transforms

---

## Keyboard Shortcuts (Reference from PRD)

| Key | Action |
|-----|--------|
| Tab | Enter/exit edit mode |
| G | Move (grab) |
| R | Rotate |
| S | Scale |
| E | Extrude |
| C | Cut |
| Shift | Disable snapping (hold) |
| Escape / Right-click | Cancel operation |
| Click | Confirm operation |

## Out of Scope (EditorPlugin Constraints)

- Color palette / typography (inherited from Godot)
- Responsive design (fixed editor context)
- Component library (uses Godot's built-in controls)
- Emotional design / branding (developer tool)
