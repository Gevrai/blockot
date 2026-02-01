# Story 1.1: Initialize Rust/gdext Project

Status: ready-for-dev

## Story

As a **developer**,
I want **a working Rust/gdext project structure with build pipeline**,
so that **I can compile the plugin and load it in Godot**.

## Acceptance Criteria

1. **Given** the repository is cloned **When** I run `cargo build` in the `rust/` directory **Then** the project compiles without errors **And** produces a shared library in the expected location

2. **Given** the compiled library exists **When** I open the Godot project in `godot/` **Then** the plugin appears in Project Settings → Plugins **And** I can enable the plugin without errors

3. **Given** the plugin is enabled **When** I check the Godot console **Then** no errors or warnings from blockot appear

## Tasks / Subtasks

- [ ] **Task 1: Create project directory structure** (AC: 1)
  - [ ] Create `rust/` directory with `Cargo.toml`
  - [ ] Create `rust/src/lib.rs` entry point
  - [ ] Create `godot/` directory with `project.godot`
  - [ ] Create `godot/addons/blockot/` directory structure
  - [ ] Create `godot/addons/blockot/bin/` for compiled libraries (gitignored)
  - [ ] Create `godot/test_scenes/` directory

- [ ] **Task 2: Configure Cargo.toml for gdext** (AC: 1)
  - [ ] Add gdext dependency with pinned version
  - [ ] Configure crate type as `cdylib`
  - [ ] Set package metadata (name, version, edition)
  - [ ] Add lto and optimization settings for release builds

- [ ] **Task 3: Implement minimal lib.rs** (AC: 1, 2, 3)
  - [ ] Add gdext imports and extension init macro
  - [ ] Register extension entry point with `#[gdextension]`
  - [ ] Implement `ExtensionLibrary` trait
  - [ ] Add debug logging to confirm initialization

- [ ] **Task 4: Create plugin.cfg** (AC: 2)
  - [ ] Set plugin name, description, author
  - [ ] Configure script path (empty for GDExtension-only)
  - [ ] Set version number

- [ ] **Task 5: Create blockot.gdextension** (AC: 2)
  - [ ] Configure entry symbol
  - [ ] Set compatibility minimum (Godot 4.1)
  - [ ] Configure library paths for all platforms:
    - `linux.x86_64` → `bin/libblockot.linux.x86_64.so`
    - `windows.x86_64` → `bin/blockot.windows.x86_64.dll`
    - `macos.universal` → `bin/libblockot.macos.universal.dylib`

- [ ] **Task 6: Create Godot project.godot** (AC: 2)
  - [ ] Configure project settings
  - [ ] Set project name
  - [ ] Reference plugin in addons

- [ ] **Task 7: Build and verify** (AC: 1, 2, 3)
  - [ ] Run `cargo build` and verify compilation
  - [ ] Copy library to correct `godot/addons/blockot/bin/` location
  - [ ] Open Godot project and enable plugin
  - [ ] Verify no console errors

- [ ] **Task 8: Update .gitignore** (AC: 1)
  - [ ] Add `rust/target/` to gitignore
  - [ ] Add `godot/addons/blockot/bin/` to gitignore
  - [ ] Add `.godot/` cache directory

## Dev Notes

### Architecture Compliance

This story establishes the foundational project structure per the Architecture document. All decisions MUST align with:

- **Technology Stack:** Rust with gdext (godot-rust)
- **Godot Version:** 4.1+ minimum, 4.2+ recommended
- **Project Structure:** Must match exactly as defined in architecture.md

[Source: architecture.md#Project-Structure]

### Project Structure (MUST MATCH EXACTLY)

```
blockot/
├── godot/                    # Godot project for testing
│   ├── project.godot
│   ├── addons/
│   │   └── blockot/
│   │       ├── plugin.cfg
│   │       ├── blockot.gdextension
│   │       └── bin/          # Compiled libraries (git-ignored)
│   └── test_scenes/
├── rust/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs            # Entry point, registers extension
└── .gitignore
```

[Source: architecture.md#Complete-Directory-Structure]

### Cargo.toml Configuration

```toml
[package]
name = "blockot"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
godot = { git = "https://github.com/godot-rust/gdext", branch = "master" }
# NOTE: Pin to specific commit/tag when available for stability

[profile.release]
lto = true
opt-level = 3
```

**CRITICAL:** Pin gdext version to mitigate breaking changes (Risk mitigation from epics.md)

[Source: architecture.md#Build-Distribution]

### Minimal lib.rs Implementation

```rust
use godot::prelude::*;

struct BlockotExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BlockotExtension {}
```

This is the bare minimum to verify the GDExtension loads. No classes registered yet.

[Source: architecture.md#Key-Rust-gdext-Patterns]

### plugin.cfg Format

```ini
[plugin]

name="blockot"
description="Blender-like geometry editing for Godot"
author="You"
version="0.1.0"
script=""
```

**Note:** `script=""` because this is a pure GDExtension plugin, not GDScript.

[Source: architecture.md#NFR9]

### blockot.gdextension Format

```ini
[configuration]

entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1

[libraries]

linux.x86_64 = "res://addons/blockot/bin/libblockot.linux.x86_64.so"
windows.x86_64 = "res://addons/blockot/bin/blockot.windows.x86_64.dll"
macos.universal = "res://addons/blockot/bin/libblockot.macos.universal.dylib"
```

[Source: architecture.md#Release-Artifacts]

### Build Commands

```bash
# From project root
cd rust && cargo build

# Output location (Linux example):
# rust/target/debug/libblockot.so

# Copy to Godot addon:
cp rust/target/debug/libblockot.so godot/addons/blockot/bin/libblockot.linux.x86_64.so
```

[Source: architecture.md#Development]

### Technical Requirements

| Requirement | Value | Source |
|-------------|-------|--------|
| Rust toolchain | Latest stable | project-context.md |
| Godot minimum | 4.1 | architecture.md |
| Godot recommended | 4.2+ | architecture.md |
| Crate type | cdylib | architecture.md |
| Entry symbol | gdext_rust_init | gdext convention |

### Naming Conventions (Apply from Story 1.1)

- **Rust modules/files:** `snake_case`
- **Structs/Enums:** `PascalCase`
- **Functions:** `snake_case`

[Source: architecture.md#Naming-Patterns]

### Test Requirements

This story has NO unit tests because it's infrastructure setup only. Verification is manual:

1. `cargo build` succeeds
2. Plugin loads in Godot
3. No console errors

**Future stories WILL require tests per the project-context.md rules.**

### Critical Don'ts for This Story

- **DO NOT** add any BlockotNode class yet (that's Story 1.2)
- **DO NOT** add geometry modules yet
- **DO NOT** add any GDScript files
- **DO NOT** over-engineer the structure — keep it minimal

### gdext Version Research (Current as of 2026-02-01)

**Latest gdext status:**
- gdext is the successor to godot-rust/gdnative
- Uses Godot 4's GDExtension API
- Check https://github.com/godot-rust/gdext for latest commit/tag

**Recommendation:** Use git dependency initially, then pin to a tag/commit once verified working:
```toml
# During development:
godot = { git = "https://github.com/godot-rust/gdext", branch = "master" }

# After verification (update hash):
godot = { git = "https://github.com/godot-rust/gdext", rev = "abc1234" }
```

### Project Context Reference

**MUST READ:** `/home/gejora/Documents/godot/blockot/_bmad-output/project-context.md`

This file contains critical rules that apply to ALL stories:
- Godot types at edges only
- Command pattern rules (N/A for this story)
- Testing rules
- Code quality standards

### Success Verification Checklist

- [ ] `cargo build` compiles without errors
- [ ] Library file exists at expected location
- [ ] Godot recognizes plugin in Project Settings → Plugins
- [ ] Plugin can be enabled without errors
- [ ] Godot console shows no errors/warnings from blockot
- [ ] Project structure matches architecture.md exactly

## Dev Agent Record

### Agent Model Used

_To be filled by dev agent_

### Debug Log References

_To be filled by dev agent_

### Completion Notes List

_To be filled by dev agent_

### File List

_To be filled by dev agent - list all files created/modified_
