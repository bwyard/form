# FORM — Claude Code Project Guide
Last updated: 2026-03-21

Read claude-resources/CLAUDE.md first, then this file.

---

## Core Philosophy

**Math reads as visual art.** SDFs describe shapes, noise describes texture, splines describe motion. A scene is a pure function of space.

Describe the world with functions, not data structures. No mesh files, no hardcoded geometry tables — generate everything at evaluation time. Pure functional throughout: same inputs, same outputs, no side effects, no state.

This is what makes FORM a reference implementation rather than just another graphics library.

---

## What is FORM?

Math-first procedural graphics framework. Geometry defined as pure mathematical functions (SDFs), not mesh files. Pairs with SCORE (audio) and Stage (game feel).

**FORM is not a game engine.** It is a library that plugs into Bevy/Unreal/Unity and provides the SDF math layer.

---

## Monorepo structure

```
form/
├── Cargo.toml              # Cargo workspace root
├── package.json            # pnpm workspace root
├── docs/
│   └── ROADMAP.md          # Full roadmap with phase detail and testing philosophy
├── crates/
│   ├── form-sdf/           # Phase 0 — SDF primitives, CSG, domain transforms (DONE)
│   ├── form-render/        # Phase 1 — offline CPU ray marcher (DONE, gap tests pending)
│   ├── form-noise/         # Phase 5 — noise functions (scaffolded, blocked on prime)
│   └── form-animate/       # Phase 4 gait — animation math (scaffolded)
└── packages/
    ├── form-core/          # Phase 6 — TS component API (scaffolded)
    ├── form-cli/           # Phase 6 — CLI (scaffolded)
    └── form-score-bridge/  # Phase 7 — SCORE ↔ FORM bridge (future)
```

---

## Current phase: Phase 1 gap work → Phase 2

See `docs/ROADMAP.md` for full phase detail and testing philosophy.

### Phase 1 — gap tests to add (do this first)

Missing T2 render tests in `form-render`:
- Sky pixel is blue-dominant: corner pixels have B > R
- Hit pixel is not sky: center pixel of sphere render is brighter than sky background
- Symmetric SDF produces symmetric pixel output: left half ≈ right half
- BMP file dimensions correct for given width/height

### Phase 2 — Primitive gallery (next after gap tests)

One example + test block per SDF primitive. Each has T1 (math), T2 (render), T3 (visual confirm).
Primitives: sphere, box, capsule, torus, cylinder, union, subtract, smooth_union, smooth_subtract.

### Phase 3 — Demo A: Corruption stack (10 steps)

Head built one corruption layer at a time. No time parameter. See ROADMAP.md for step-by-step spec.

### Phase 4 — Demo B: Gait animation (5 steps)

Simple body animated by pure trig. Comes after Phase 3. See ROADMAP.md.

---

## Testing philosophy

Three layers — all must pass before moving to the next phase or step:

**T1 — Math tests** (`cargo test`, no rendering)
SDF values at known points, warp directions, CSG correctness.

**T2 — Render tests** (`cargo test`, 11×11 pixel assertions)
Center hits, corners miss to sky, symmetry, brightness bounds.

**T3 — Visual confirmation** (user opens BMP)
I write "Expected:" before every render. User confirms or reports diff.
We do not proceed until T3 passes.

---

## Code standards

### Rustdoc — MANDATORY on every public function

Every public function must have rustdoc with ALL of these sections:
- One-line summary
- `# Math` — the formula, written in plain text math notation
- `# Arguments` — each param
- `# Returns` — what the value means
- `# Example` — runnable doctest

### Math reference
All SDF implementations must match Inigo Quilez's reference: iquilezles.org/articles/distfunctions/

### Testing requirements
Every SDF function:
- inside test: `sdf(inside_point) < 0`
- outside test: `sdf(outside_point) > 0`
- surface test: `sdf(surface_point).abs() < EPSILON`

Every domain warp function:
- direction test: warp moves point in expected direction
- magnitude test: displacement within expected bounds
- identity test: outside warp region, point unchanged

Use `const EPSILON: f32 = 1e-5` for float comparisons.

---

## Commands

```bash
# Test everything
cargo test

# Test a specific crate
cargo test -p form-render
cargo test -p form-sdf
cargo test -p form-animate

# Run a specific example
cargo run --example sphere -p form-render
cargo run --example human -p form-render -- 0.0

# Watch tests
cargo watch -x test

# Docs
cargo doc -p form-render --open
```

---

## Environment requirements

- Rust 1.75+ (install via rustup)
- `rustup target add wasm32-unknown-unknown` (for Phase 5 form-noise WASM)
- `cargo install wasm-pack` (for Phase 5)
- `cargo install cargo-watch` (for watch mode)

---

## Key decisions recorded

- **Domain warps only**: corruption functions warp the input point `p`, never modify SDF output values. This preserves SDF validity.
- **smooth_union formula**: form-sdf uses the quadratic smin (IQ). Example files use the cubic smin (IQ). Both are valid. Do not mix within a single scene.
- **Demo A and Demo B are separate**: static corruption proof has no time parameter. Animation proof has minimal geometry. They are composed only at Phase 8 (Bevy).
- **form-render is a tool, not a deliverable**: it enables visual verification of every other phase. Phase numbering reflects build order, not importance.
- **THESIS.md is local only**: gitignored, never pushed to remote. Not ready to publish.
