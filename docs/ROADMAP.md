# FORM — Roadmap

Math-first procedural graphics framework. SDFs, not meshes.

---

## Milestones

| Milestone | Target | Status |
|---|---|---|
| form-sdf on crates.io | 2026-04-07 | Phase 1 |
| WASM + browser renderer | 2026-04-21 | Phase 2 |
| Procedural animation (gait) | 2026-05-05 | Phase 3 |
| Offline CPU render pipeline | 2026-05-19 | Phase 4 |
| TypeScript CLI + component API | 2026-06-02 | Phase 5 |
| SCORE ↔ FORM bridge | 2026-06-16 | Phase 6 |
| Bevy integration demo | 2026-07-07 | Phase 7 — key milestone |
| crates.io publish + docs site | 2026-07-21 | Phase 8 |

---

## Phase detail

### Phase 0 — Scaffold ✅ (2026-03-18)
- Cargo workspace root
- pnpm workspace root
- form-sdf crate skeleton
- Git repo initialized

### Phase 1 — form-sdf (target: 2026-04-07)
**Only crate: `form-sdf`. Dependency: `glam` only.**

2D SDF primitives:
- [ ] circle
- [ ] box_2d
- [ ] rounded_box
- [ ] capsule_2d
- [ ] line_segment
- [ ] triangle
- [ ] ring

3D SDF primitives:
- [ ] sphere
- [ ] box_3d
- [ ] capsule_3d
- [ ] cylinder
- [ ] torus
- [ ] plane

CSG operations:
- [ ] union
- [ ] intersection
- [ ] subtract
- [ ] xor
- [ ] smooth_union
- [ ] smooth_intersection
- [ ] smooth_subtract

Domain operations:
- [ ] translate
- [ ] rotate_2d
- [ ] scale
- [ ] repeat
- [ ] mirror_x
- [ ] mirror_y
- [ ] elongate

Success criteria:
1. `cargo test -p form-sdf` passes with full coverage
2. Every public function has rustdoc with `# Math` section
3. Tests verify against IQ's reference formulas

### Phase 2 — form-noise (target: 2026-04-21)
- Value noise, gradient noise (Perlin-style)
- Simplex noise
- FBM (fractal Brownian motion)
- Domain warping
- WASM target added (`wasm32-unknown-unknown`)
- `wasm-pack` build pipeline

### Phase 3 — form-animate (target: 2026-05-05)
- Gait math (walk cycles from pure trig)
- Procedural IK (inverse kinematics)
- Spring/damper systems
- Bezier / Hermite spline helpers

### Phase 4 — form-render (target: 2026-05-19)
- Offline CPU ray marcher (for previews/tests)
- Sphere tracing loop
- Basic lighting (normal estimation, diffuse, shadow)
- PPM/PNG output
- `wgpu` added for GPU path (dev tool only)

### Phase 5 — TypeScript packages (target: 2026-06-02)
- `form-core`: TS component API, scene compiler
- `form-cli`: `form dev`, `form build`, `form render` commands
- Scene files as plain TS (same philosophy as SCORE songs)

### Phase 6 — form-score-bridge (target: 2026-06-16)
- Map SCORE audio params → FORM visual params
- BPM → animation speed
- Amplitude → scale/intensity
- Frequency → spatial frequency of noise

### Phase 7 — Engine integrations (target: 2026-07-07) — KEY MILESTONE
- Bevy plugin (`form-bevy`)
- Demo: procedural character in Bevy running in real-time
- Unity C# bridge (research phase)
- Unreal BP/C++ bridge (research phase)

### Phase 8 — Release (target: 2026-07-21)
- Publish `form-sdf`, `form-noise`, `form-animate` to crates.io
- Docs site (mdBook or similar)
- GitHub README with visual demos
- Blog post: "A running human in 50 lines of trig"
