# FORM — Claude Code Project Guide
Last updated: 2026-03-21

Read claude-resources/CLAUDE.md first, then this file.

---

## Core Philosophy

**Math reads as visual art.** SDFs describe shapes, noise describes texture, splines describe motion. A scene is a pure function of space.

Describe the world with functions, not data structures. No mesh files, no hardcoded geometry tables — generate everything at evaluation time. Pure functional throughout: same inputs, same outputs, no side effects, no state.

**FORM is not a game engine.** It is a library that plugs into Bevy/Unreal/Unity and provides the SDF math layer.

---

## 2D before 3D

Every proof is done in 2D first, then extended to 3D. This is the core structural decision.

- **2D rasterizer** — for each pixel, evaluate `sdf(point) -> f32`. Inside = white, outside = dark. No ray marching, no camera, no lighting. Instant feedback, nothing to go wrong.
- **3D renderer** — sphere tracing, camera, lighting. Used only after the 2D proof is solid.

This means every corruption warp is proven in 2D before being extended to 3D. If a warp is wrong in 2D, you can see exactly what it does without any renderer complexity in the way.

---

## Monorepo structure

```
form/
├── Cargo.toml
├── package.json
├── docs/
│   └── ROADMAP.md
├── crates/
│   ├── form-sdf/       # Phase 0 — SDF primitives, CSG, domain transforms (DONE)
│   ├── form-render/    # Phase 1 — 3D offline CPU ray marcher (DONE)
│   ├── form-raster/    # Phase 2 — 2D flat rasterizer (next)
│   ├── form-noise/     # Future — noise (blocked on prime-noise)
│   └── form-animate/   # Future — gait (scaffolded)
└── packages/
    ├── form-core/      # Future — TS component API
    ├── form-cli/       # Future — CLI
    └── form-score-bridge/ # Future — SCORE bridge
```

---

## Current phase: Phase 2 — form-raster (2D rasterizer)

**Build the 2D flat rasterizer before anything else.**

### What form-raster does

```rust
// For each pixel, map to world space, evaluate SDF, color by sign
fn rasterize(sdf: impl Fn(Vec2) -> f32, width: u32, height: u32, scale: f32) -> Vec<u8>
```

- Inside (sdf < 0): white `[255, 255, 255]`
- Edge (sdf.abs() < edge_width): gray `[180, 180, 180]`
- Outside (sdf > 0): dark blue `[20, 20, 40]`
- Optional: distance gradient tint on outside

No camera. No march. No lighting. Pure SDF → pixel color.

### After form-raster

Phase 3: 2D primitive gallery — one example per 2D SDF primitive, T1+T2+T3 each.
Phase 4: 2D corruption stack — circle warped one layer at a time, 10 steps.
Phase 5: 3D corruption stack — extend each proven 2D warp to 3D sphere.
Phase 6: 2D gait — animated circle, pure trig, no corruption.
Phase 7: 3D gait — extend to capsule body.

---

## Testing philosophy — three layers, every phase

**T1 — Math tests** (`cargo test`, no rendering)
SDF values at known points, warp direction/magnitude tests, CSG correctness.

**T2 — Render tests** (`cargo test`, small pixel buffer assertions)
2D: inside pixels are white, outside pixels are dark, edge visible.
3D: center hits, corners miss to sky, symmetry.

**T3 — Visual confirmation** (user opens BMP)
I write "Expected:" before every render. User confirms or reports diff.
Do not proceed to next step until T3 passes.

---

## Testing requirements

Every SDF function:
- `sdf(inside_point) < 0`
- `sdf(outside_point) > 0`
- `sdf(surface_point).abs() < EPSILON`

Every domain warp function:
- direction test: warp moves point in expected direction
- magnitude test: displacement within expected bounds
- identity test: outside warp region, point unchanged

Use `const EPSILON: f32 = 1e-5`.

---

## Key architectural decisions

- **Domain warps only**: corruption warps the input point `p`, never the SDF output value
- **2D before 3D**: every warp proven in 2D first, then extended to 3D
- **Demo A** (corruption) and **Demo B** (gait) are separate proofs — not combined until Phase 8
- **THESIS.md** is local only — gitignored, never pushed
- **smooth_union formula**: form-sdf uses quadratic smin (IQ). Example files use cubic smin (IQ). Both valid, do not mix within one scene.

---

## Commands

```bash
cargo test                          # all crates
cargo test -p form-raster           # 2D rasterizer tests
cargo test -p form-render           # 3D renderer tests
cargo test -p form-sdf              # SDF primitive tests
cargo run --example ex_circle -p form-raster   # 2D examples
cargo run --example ex_sphere -p form-render   # 3D examples
cargo clippy --workspace -- -D warnings        # must be clean
```

---

## Math reference

All SDF implementations must match Inigo Quilez: iquilezles.org/articles/distfunctions/
