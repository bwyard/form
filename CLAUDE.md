# FORM — Claude Code Project Guide
Last updated: 2026-03-18

Read claude-resources/CLAUDE.md first, then this file.

---

## Core Philosophy

**Math reads as visual art.** SDFs describe shapes, noise describes texture, splines describe motion. A scene is a pure function of space.

Describe the world with functions, not data structures. No mesh files, no hardcoded geometry tables — generate everything at evaluation time. Pure functional throughout: same inputs, same outputs, no side effects, no state.

This is what makes FORM a reference implementation rather than just another graphics library.

**Probabilistic / diffusion direction (roadmap):** Probabilistic ray marching, soft shadows via Monte Carlo sampling, ambient occlusion estimation, depth-of-field — all built on PRIME's sampler layer. Planned for form-render phase.

---

## What is FORM?

Math-first procedural graphics framework. Game geometry, characters, and environments defined as pure mathematical functions (SDFs — signed distance functions), not mesh files. Pairs with SCORE (audio).

**FORM is not a game engine.** It is a library that plugs into Bevy/Unreal/Unity and provides the SDF math layer.

---

## Monorepo structure

```
form/
├── Cargo.toml              # Cargo workspace root
├── package.json            # pnpm workspace root
├── crates/
│   ├── form-sdf/           # Phase 1 — SDF primitives, CSG, domain transforms
│   ├── form-noise/         # Phase 2 — noise functions
│   ├── form-animate/       # Phase 3 — gait/animation math
│   └── form-render/        # Phase 4 — offline CPU renderer
└── packages/
    ├── form-core/          # Phase 5 — TS component API
    ├── form-cli/           # Phase 5 — CLI
    └── form-score-bridge/  # Phase 6 — SCORE ↔ FORM bridge
```

---

## Current phase: Phase 1 — form-sdf

**Only build what is listed here. Nothing else.**

### What to implement
- 2D SDF primitives: circle, box_2d, rounded_box, capsule_2d, line_segment, triangle, ring
- 3D SDF primitives: sphere, box_3d, capsule_3d, cylinder, torus, plane
- CSG operations: union, intersection, subtract, xor, smooth_union, smooth_intersection, smooth_subtract
- Domain operations: translate, rotate_2d, scale, repeat, mirror_x, mirror_y, elongate

### What NOT to do in Phase 1
- No WASM targets
- No TS packages
- No naga or wgpu
- No noise functions
- No publishing to crates.io

---

## Code standards

### Rustdoc — MANDATORY on every public function

Every public function must have rustdoc with ALL of these sections:
- One-line summary
- `# Math` — the formula, written in plain text math notation
- `# Arguments` — each param
- `# Returns` — what the value means
- `# Example` — runnable doctest

See the `circle()` function in `crates/form-sdf/src/primitives/d2.rs` as the canonical example.

### Math reference
All SDF implementations must match Inigo Quilez's reference: iquilezles.org/articles/distfunctions/

### Testing
Every function gets at minimum:
- outside test
- inside test
- on-surface test (where applicable)
- edge case test

Use `const EPSILON: f32 = 1e-5` for float comparisons.

---

## Commands

```bash
# Build
cargo build -p form-sdf

# Test
cargo test -p form-sdf

# Watch tests
cargo watch -x "test -p form-sdf"

# Docs
cargo doc -p form-sdf --open
```

---

## Environment requirements

- Rust 1.75+ (install via rustup)
- `rustup target add wasm32-unknown-unknown` (for Phase 2+)
- `cargo install wasm-pack` (for Phase 2+)
- `cargo install cargo-watch` (for watch mode)

Install Rust if not present:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
```

---

## Sister project

SCORE (EDM audio framework) — same pnpm/Nx monorepo structure, same philosophy.
Songs in SCORE = plain JS files. Scenes in FORM = plain TS files.
