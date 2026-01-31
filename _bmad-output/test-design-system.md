# System-Level Test Design

**Project:** blockot
**Date:** 2026-01-31
**Phase:** 3 (Solutioning - Pre-Implementation Readiness)
**Author:** TEA (Test Architect)

---

## Executive Summary

This document provides a system-level testability review of the blockot architecture before implementation begins. It identifies architecturally significant requirements (ASRs), assesses testability, recommends test level distribution, and flags concerns for the solutioning gate check.

**Testability Assessment:** PASS with recommendations

The architecture is well-designed for testability, with pure Rust core modules, Command Pattern for deterministic operations, and clear module boundaries. No blockers identified.

---

## Testability Assessment

### Controllability ✅ PASS

**Assessment:** Excellent

| Aspect | Status | Evidence |
|--------|--------|----------|
| State control | ✅ | BlockotGeometry stores vertices/faces as pure Rust data structures |
| Test data seeding | ✅ | `test_utils.rs` provides shared geometry builders/factories |
| External dependencies | ✅ | Core modules (`geometry/`, `selection/`, `tools/`) have no Godot dependencies - mockable |
| Error condition triggering | ✅ | `BlockotError` enum enables explicit error states |

**Details:**
- Pure Rust core (`geometry/mesh.rs`, `selection/mod.rs`, `tools/commands/`) can be tested in isolation
- Command Pattern allows deterministic state manipulation via `execute()` and `undo()`
- Preview State pattern separates visual preview from committed state
- No external services or databases - all state is in-memory Rust structures

### Observability ✅ PASS

**Assessment:** Good

| Aspect | Status | Evidence |
|--------|--------|----------|
| State inspection | ✅ | `BlockotGeometry.vertices` and `faces` are accessible for verification |
| Deterministic results | ✅ | Commands are infallible once created; validation at construction |
| NFR validation | ⚠️ | Performance metrics require manual profiling (no built-in telemetry) |
| Logging | ✅ | `log` crate interface with Godot backend injection |

**Details:**
- Test assertions can directly compare `BlockotGeometry` state before/after commands
- All transform operations produce exact inverse transforms for precise undo verification
- Selection state is vertex-canonical - canonical representation enables straightforward assertions
- Logging via `log::debug!`, `log::warn!` routes to Godot console for debugging

**Minor Gap:** No built-in performance counters. Recommend manual benchmarking for NFR1-4.

### Reliability ✅ PASS

**Assessment:** Good

| Aspect | Status | Evidence |
|--------|--------|----------|
| Test isolation | ✅ | Commands operate on owned `&mut BlockotGeometry` - no shared state |
| Reproducibility | ✅ | Commands are deterministic; inverse transforms stored (not computed) |
| Loose coupling | ✅ | Godot types isolated to `editor/` module boundary |
| Cleanup discipline | ✅ | Preview auto-cancelled on scene exit/pre-save |

**Details:**
- Each test can create fresh `BlockotGeometry` via `test_utils.rs` fixtures
- No global state - command history cleared on scene reload (by design)
- Module boundaries prevent Godot type leakage into testable core
- Tests can run in parallel without state pollution

---

## Architecturally Significant Requirements (ASRs)

The following requirements drive architecture decisions and require targeted test coverage:

### ASR-1: Undo/Redo Correctness (NFR5)

**Requirement:** Undo/redo correctly restores all geometry state without corruption

| Attribute | Value |
|-----------|-------|
| Category | DATA |
| Probability | 2 (Possible) |
| Impact | 3 (Critical) |
| **Risk Score** | **6** |
| Mitigation | Command Pattern with exact inverse transforms |

**Testability Impact:**
- Every Command implementation requires roundtrip test: `execute → undo → assert original state`
- Inverse transforms precomputed (not negated deltas) prevents floating-point drift
- Multi-command chains need integration tests (5+ operations, interleaved undo/redo)

**Test Strategy:**
- Unit: Each command's roundtrip
- Integration: Undo chains, redo interleaving
- E2E (Automated): Undo restores geometry hash in headless Godot test

### ASR-2: Transform Responsiveness (NFR1)

**Requirement:** Transform operations (G/R/S) respond without perceptible lag (<100ms feedback)

| Attribute | Value |
|-----------|-------|
| Category | PERF |
| Probability | 2 (Possible) |
| Impact | 2 (Degraded) |
| **Risk Score** | **4** |
| Mitigation | Preview State operates on affected vertices only; cache rebuild deferred |

**Testability Impact:**
- Performance bounded by affected geometry, not total mesh size
- Requires benchmarking with 500-face geometry (NFR4 threshold)
- No automated performance tests in MVP - manual profiling

**Test Strategy:**
- Unit: Verify operation affects only selected vertices
- Benchmark: Manual profiling with `cargo bench` or Godot profiler
- Threshold: <100ms for 500-face mesh transform

### ASR-3: Save/Load Fidelity (NFR6, NFR7)

**Requirement:** Scene save persists all geometry data; load restores exactly

| Attribute | Value |
|-----------|-------|
| Category | DATA |
| Probability | 2 (Possible) |
| Impact | 3 (Critical) |
| **Risk Score** | **6** |
| Mitigation | Flat array serialization (PackedVector3Array, PackedInt32Array) |

**Testability Impact:**
- Serialization functions in `geometry/serialization.rs` are pure Rust - testable
- Roundtrip test: Rust data → PackedArrays → Rust data
- Git-diffable .tscn format enables visual inspection

**Test Strategy:**
- Unit: `to_packed_arrays()` → `from_packed_arrays()` roundtrip
- Integration: Save .tscn, reload, compare geometry state
- E2E (Automated): Save/load roundtrip preserves geometry hash in headless test

### ASR-4: Mode Switch Latency (NFR2)

**Requirement:** Entering/exiting edit mode completes within 200ms

| Attribute | Value |
|-----------|-------|
| Category | PERF |
| Probability | 1 (Unlikely) |
| Impact | 1 (Minor) |
| **Risk Score** | **1** |
| Mitigation | Simple state machine; no heavy operations on mode switch |

**Testability Impact:**
- Low risk - mode switch is lightweight state change
- No special test infrastructure needed

**Test Strategy:**
- Unit: Mode state transitions
- E2E (Automated): Mode state toggle verification in headless test

### ASR-5: Snapping Accuracy (FR22-25)

**Requirement:** Grid snap to 1m default; proximity snap to nearby geometry

| Attribute | Value |
|-----------|-------|
| Category | TECH |
| Probability | 2 (Possible) |
| Impact | 1 (Minor) |
| **Risk Score** | **2** |
| Mitigation | SnapProvider trait abstraction |

**Testability Impact:**
- Snap logic in `editor/snapping.rs` can be unit tested
- LocalSnapProvider mockable for proximity snap testing

**Test Strategy:**
- Unit: Grid snap calculations
- Unit: Proximity snap detection
- Integration: Shift modifier disables snap

---

## Test Levels Strategy

Based on the architecture (Rust EditorPlugin, pure Rust core, Godot integration layer):

### Recommended Distribution

| Level | Percentage | Rationale |
|-------|------------|-----------|
| **Unit** | 60% | Pure Rust core (`geometry/`, `selection/`, `tools/`) is highly testable |
| **Integration** | 25% | Command → EditorUndoRedoManager, serialization roundtrips |
| **E2E (Automated)** | 15% | Godot headless mode + GDScript test harness |
| **Manual** | 0% | Exploratory only, not counted in distribution |

> **Note:** E2E tests are fully automated via Godot's `--headless` mode, not manual validation.

### Unit Test Focus

**Primary targets (pure Rust, no Godot):**
- `geometry/mesh.rs` - BlockotGeometry operations
- `geometry/primitives.rs` - Box, Plane, Cylinder, Sphere generation
- `geometry/serialization.rs` - PackedArray conversion
- `selection/mod.rs` - Selection state, vertex-canonical model
- `tools/commands/*.rs` - All Command implementations

**Test pattern per command:**
```rust
#[test]
fn test_command_roundtrip() {
    let mut geo = BlockotGeometry::unit_cube();
    let original = geo.clone();

    let cmd = MoveVertices::new(indices, transform)?;
    cmd.execute(&mut geo);
    cmd.undo(&mut geo);

    assert_eq!(geo, original);  // Exact restoration
}
```

### Integration Test Focus

**Location:** `rust/tests/`

**Primary targets:**
- `undo_redo_chains.rs` - Multi-command undo/redo sequences
- `selection_transforms.rs` - Select + transform workflows
- `serialization.rs` - Full save/load roundtrip
- `preview_commit.rs` - Preview → commit/cancel flows

### E2E Test Focus (Automated)

**Automation approach:** Godot headless mode + GDScript test harness

**Location:** `godot/test_scenes/`

**Test harness architecture:**
```
godot/test_scenes/
├── e2e_test_runner.gd       # Autoloaded harness, iterates tests, exits with code
├── tests/
│   ├── test_plugin_load.gd   # Plugin enables without error
│   ├── test_cube_creation.gd # Default cube geometry correct
│   ├── test_edit_mode.gd     # Tab enters/exits edit mode
│   ├── test_transform.gd     # G/R/S initiates transform
│   ├── test_undo_redo.gd     # Ctrl+Z restores state
│   └── test_save_load.gd     # Scene roundtrip preserves geometry
└── fixtures/
    └── test_scene.tscn       # Pre-configured test scene
```

**P0 E2E Feature Checks (automated):**
- [ ] Plugin enables without error in Godot
- [ ] BlockotNode appears in Add Node dialog
- [ ] Default cube has correct geometry (8 vertices, 6 faces)
- [ ] Tab enters/exits edit mode (state check via exposed method)
- [ ] G key initiates move (preview state becomes active)
- [ ] Undo restores previous geometry state
- [ ] Save/load roundtrip preserves geometry hash

**Example GDScript test:**
```gdscript
# godot/test_scenes/tests/test_cube_creation.gd
extends Node

func test_cube_creation() -> bool:
    var node = BlockotNode.new()
    add_child(node)

    var passed = true
    passed = passed and (node.get_vertex_count() == 8)
    passed = passed and (node.get_face_count() == 6)
    passed = passed and (node.get_geometry_hash() != 0)

    node.queue_free()
    return passed
```

**CI execution:**
```bash
godot --headless --path godot/ -s test_scenes/e2e_test_runner.gd
```

### Cross-Version Compatibility Testing

**Requirement:** Validate plugin works across all supported Godot versions (4.1 to latest)

**CI Matrix Strategy:**
```yaml
# .github/workflows/e2e.yml
name: E2E Tests

on: [push, pull_request]

jobs:
  e2e-test:
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        godot: ['4.1.4', '4.2.2', '4.3.0', '4.4-stable']

    steps:
      - uses: actions/checkout@v4

      - name: Build Rust library
        run: cd rust && cargo build --release

      - uses: chickensoft-games/setup-godot@v2
        with:
          version: ${{ matrix.godot }}
          include-templates: false

      - name: Run E2E tests
        run: |
          godot --headless --path godot/ -s test_scenes/e2e_test_runner.gd
          exit $?
```

**Version compatibility notes:**
- gdext version pinned in `Cargo.toml` per architecture decision
- Different Godot versions may require different gdext builds
- Test logic remains identical across versions
- CI matrix catches compatibility regressions automatically

**Supported version range:**
| Version | Status | Notes |
|---------|--------|-------|
| 4.1.x | Minimum | gdext binary compatibility floor |
| 4.2.x | Recommended | Improved GDExtension APIs |
| 4.3.x | Current | Full feature support |
| 4.4+ | Future | Test on stable releases |

---

## NFR Testing Approach

### Security (SEC) - Not Applicable

- blockot is an editor plugin, not a networked service
- No authentication, authorization, or data exposure concerns
- No OWASP validation required

### Performance (PERF) - Manual Benchmarking

**NFR1-4 Validation Approach:**

| NFR | Threshold | Validation Method |
|-----|-----------|-------------------|
| NFR1: Transform latency | <100ms | `cargo bench` + manual timing |
| NFR2: Mode switch | <200ms | Manual timing |
| NFR3: Snap calculation | No visible stutter | Manual observation |
| NFR4: 500-face geometry | Responsive | Benchmark with test geometry |

**Recommended Tool:** Rust `criterion` crate for micro-benchmarks

```rust
// rust/benches/transform_bench.rs
#[bench]
fn bench_move_500_faces(c: &mut Criterion) {
    let geo = BlockotGeometry::complex_geometry(500);
    let cmd = MoveVertices::new(all_indices, small_offset);

    c.bench_function("move_500_faces", |b| {
        b.iter(|| cmd.execute(&mut geo.clone()))
    });
}
```

**Performance Gate Criteria:**
- P95 transform latency < 50ms (gives 2x headroom on 100ms threshold)
- No regressions > 20% between releases

### Reliability (REL) - Command Determinism

**Validation Approach:**

| Aspect | Test Strategy |
|--------|---------------|
| Undo/redo correctness | Roundtrip tests for every command |
| Save/load fidelity | Serialization roundtrip tests |
| Plugin stability | Manual Godot testing (no crash on normal usage) |

**No circuit breakers, retries, or health checks needed** - this is a single-process editor plugin.

### Maintainability (MAINT) - CI Pipeline

**Validation via CI:**

| Check | Tool | Threshold |
|-------|------|-----------|
| Code coverage | `cargo tarpaulin` or `grcov` | ≥80% for `tools/commands/` |
| Linting | `cargo clippy` | No warnings |
| Formatting | `cargo fmt --check` | Consistent |
| Test pass rate | `cargo test` | 100% |

---

## Test Environment Requirements

### Local Development

```bash
# Required
- Rust stable (latest)
- Godot 4.1+ (for manual testing)

# Test commands
cd rust && cargo test        # Unit + integration tests
cd rust && cargo clippy      # Lint
cd rust && cargo bench       # Benchmarks (optional)
```

### CI Environment

**GitHub Actions runners:**
- `ubuntu-latest` for Linux tests + clippy + E2E tests
- Cross-compilation targets for Windows/macOS (build-only, no runtime tests)
- Godot version matrix for E2E compatibility testing

**CI Pipeline Structure:**
```yaml
jobs:
  # Fast feedback (runs on every push)
  rust-checks:
    - cargo clippy
    - cargo test
    - cargo fmt --check

  # E2E validation (runs on every push)
  e2e-tests:
    matrix: [godot 4.1, 4.2, 4.3, latest]
    - Build Rust library
    - Run godot --headless E2E tests
    - Report pass/fail per version

  # Cross-platform builds (runs on main/release)
  cross-compile:
    - Windows x86_64
    - macOS universal
    - Linux x86_64
```

**Infrastructure requirements:**
- `chickensoft-games/setup-godot` action for Godot installation
- No databases, external services, or containers needed
- All tests run in ephemeral CI runners

---

## Testability Concerns

### No Blockers Identified

The architecture is well-suited for testing:
- Pure Rust core modules enable fast, isolated unit tests
- Command Pattern provides deterministic state transitions
- Module boundaries prevent Godot coupling in testable code
- Explicit error handling via `BlockotError` enum

### Minor Recommendations

1. **Performance telemetry**: Add optional timing metrics for transform operations during development (can remove for release)

2. **Test coverage enforcement**: Configure CI to fail if `tools/commands/` drops below 80% coverage

3. **Benchmark baseline**: Establish performance baseline early; track regressions

4. **Geometry hash method**: Add `BlockotNode.get_geometry_hash() -> u64` for fast E2E assertions:
   ```rust
   // rust/src/editor/blockot_node.rs
   #[func]
   fn get_geometry_hash(&self) -> u64 {
       self.geometry.hash()
   }
   ```
   This enables quick integrity checks in GDScript tests without deep geometry comparison.

5. **E2E test harness**: Set up `godot/test_scenes/e2e_test_runner.gd` early in Epic 4 to enable automated feature validation

---

## Recommendations for Sprint 0

### Immediate Actions (Before Epic 1)

1. **Set up `cargo test` in CI** (Story 4.1)
   - clippy + test on every push
   - Fail on warnings

2. **Create `test_utils.rs` with fixtures**
   - `unit_cube()` - 8 vertices, 6 faces
   - `simple_plane()` - 4 vertices, 1 face
   - `complex_geometry(face_count)` - for benchmarking

3. **Establish command test pattern**
   - Every command in `tools/commands/` gets roundtrip test
   - Every command gets validation rejection test

4. **Set up E2E test harness** (Story 4.2 or new story)
   - Create `godot/test_scenes/e2e_test_runner.gd`
   - Add basic plugin load test
   - Configure CI with Godot version matrix

5. **Add `get_geometry_hash()` method** (Story 1.2)
   - Enables fast E2E assertions
   - Returns hash of current geometry state

### Framework Recommendations

| Tool | Purpose | When to Set Up |
|------|---------|----------------|
| `cargo test` | Unit + integration tests | Story 1.1 |
| `cargo clippy` | Linting | Story 1.1 |
| `cargo tarpaulin` | Coverage reporting | Story 4.1 |
| `criterion` | Benchmarking | Story 4.2 (optional) |
| `godot --headless` | E2E tests | Story 4.2 |
| `chickensoft-games/setup-godot` | CI Godot installation | Story 4.1 |

### Test Naming Convention

```
test_{component}_{scenario}
test_{command}_roundtrip
test_{command}_validation_rejects_invalid_input
```

---

## Summary

| Category | Status | Notes |
|----------|--------|-------|
| **Controllability** | ✅ PASS | Pure Rust core, Command Pattern |
| **Observability** | ✅ PASS | Accessible state, logging, geometry hash |
| **Reliability** | ✅ PASS | Isolated tests, deterministic commands |
| **Test Levels** | 60/25/15 | Unit + Integration + Automated E2E |
| **E2E Automation** | ✅ | Godot headless + GDScript harness |
| **Cross-Version** | ✅ | CI matrix: Godot 4.1, 4.2, 4.3, latest |
| **NFR Testing** | Benchmarks | Performance via `criterion` crate |
| **Blockers** | None | Architecture is testable |

**Gate Recommendation:** PASS - Proceed with implementation

---

## Appendix: ASR Risk Summary

| ID | Category | Risk | Score | Status |
|----|----------|------|-------|--------|
| ASR-1 | DATA | Undo/redo corruption | 6 | Mitigated by Command Pattern |
| ASR-2 | PERF | Transform latency | 4 | Mitigated by affected-vertex-only operations |
| ASR-3 | DATA | Save/load fidelity | 6 | Mitigated by flat array serialization |
| ASR-4 | PERF | Mode switch latency | 1 | Low risk |
| ASR-5 | TECH | Snapping accuracy | 2 | Low risk |

---

**Generated by:** BMad TEA Agent - Test Architect Module
**Workflow:** `_bmad/bmm/testarch/test-design` (System-Level Mode)
**Version:** 4.0 (BMad v6)
