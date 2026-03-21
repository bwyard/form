# FORM — Roadmap

Math-first procedural graphics framework. SDFs, not meshes. A scene is a pure function of space.

Last updated: 2026-03-21

---

## Thesis

**Math thesis:** Creative outputs are mathematical objects, not processes.

Form proves this for the visual domain:

> `shape = f(description, position)` — a scene is a pure function of space.

No mesh files. No geometry tables. No state. Given the same description and the same point in space, the same value comes back every time. The SDF is the object.

This is one of four simultaneous proofs across the projects:

| Project | Domain | Claim |
|---|---|---|
| Prime | Foundation | Pure functions exist with no domain — computation without side effects |
| Score | Music | `output[n] = f(song, n, sr)` — composition is a pure function of time |
| **Form** | **Visual art** | **`shape = f(description, position)` — scene is a pure function of space** |
| Stage | Experience | `experience = f(world, t, pos)` — experience is a pure function of state and time |

All four together prove that mutation is not required — only a scan loop and explicit state threading.

### No STORE. No JUMP.

Score's assembly rule is: no STORE, no JUMP, only APPEND+ADVANCE. The sample loop advances time and evaluates. Nothing is cached or branched to.

Form follows the same rule in the spatial domain:

- **No STORE** — no mesh files, no geometry tables, no stored normals, no keyframes
- **No JUMP** — the ray marcher does not branch to precomputed data
- **MARCH+EVALUATE** — advance position along the ray, evaluate the SDF at each step

The corruption stack enforces this at every level of detail. Micro-surface texture is not a stored normal map — it is FBM evaluated at that position. Every complexity, at every scale, is evaluation not lookup.

| | Score | Form |
|---|---|---|
| Axis | Time (`n`) | Space (`p`) |
| Scan loop | Sample loop | Ray march |
| Rule | APPEND+ADVANCE | MARCH+EVALUATE |
| Pure function | `f(song, n, sr)` | `f(description, p)` |
| No STORE | No audio buffers in DSL layer | No mesh, no stored geometry |
| No JUMP | No conditional branches to cached samples | No lookup tables, no precomputed normals |

### One model, four axis types

At the assembly level APPEND+ADVANCE and MARCH+EVALUATE are the same four instructions:

```
LOAD    cursor          // read current position on the axis
EVAL    f(cursor)       // pure computation — no side effects
APPEND  result          // emit the output
ADVANCE cursor          // move forward
```

The axis type is the only variable:

| Project | Cursor type | Axis |
|---|---|---|
| Score | `n: u64` | time (sample index) |
| Form | `p: Vec3` | space (position along ray) |
| Stage | `(world, t, pos)` | state-time |

This means the thesis is not four separate proofs — it is one proof instantiated four times with different axis types. Prime is the shared foundation that all four compile onto. When Prime gains an explicit scan/march loop primitive, all four consumers will call the same instruction.

The assembly rule is: **no STORE, no JUMP — only LOAD, EVAL, APPEND, ADVANCE.**

---

## Corruption architecture

The corruption model is Form's core conceptual contribution. It is not a feature of Phase 3 — it is the way all Form scenes are structured from the start.

### What it is

Every scene has two layers:

1. **Ground state** — mathematical truth. A sphere. A box. A plane. Defined exactly by its SDF.
2. **Corruption stack** — a sequence of pure functions applied to the ground state, each one adding biological/physical irregularity.

The output is `corrupt(ground, params)` — still a pure function. The corruption stack is not mutation. It is composition.

### Corruption as proof strategy

The corruption model is not a technique inside Form — it is Form's *proof strategy*, the same role the scan loop plays in Score.

Score answers the sceptic's challenge "surely audio needs mutable buffers" with the sample scan.
Form answers "surely geometry needs stored data" with the corruption stack.

The implicit claim: **organic complexity is not evidence that you need stored data.** Biological shapes look complex. But complexity emerges from composing pure functions — not from storing data.

### Why this matters

A perfect sphere is Platonic. A human head is a sphere run through a corruption stack:

```
sphere
  → bilateral_symmetry_break(asymmetry: 0.03)
  → brow_ridge(prominence: 0.8, width: 0.6)
  → occipital_protrusion(angle: 15.0)
  → temporal_hollow(depth: 0.04)
  → jaw_shape(width: 0.9, angle: 12.0)
  → micro_surface_fbm(octaves: 6, amplitude: 0.005)
```

None of these steps store state. Each is a pure function `f(sdf, params) -> sdf`. The full stack is function composition.

This is the architectural claim Form makes: **biological complexity is mathematical composition, not stored data.**

### Corruption in animation

When Form adds time (Phase 3), time is a parameter — not a trigger. The corruption stack accepts `t` as an input and the same composition rules apply:

```
sphere
  → symmetry_break(t)
  → lorenz_gait(t, sigma, rho, beta)   // Lorenz system drives lateral/vertical/sagittal
  → micro_surface_fbm(t)
```

The running-person demo is the proof of concept: a human running, fully derived from trig and chaos math, no mesh, no keyframes, no stored poses.

---

## Milestones

| Milestone | Target | Status |
|---|---|---|
| form-sdf complete | 2026-03-18 | ✅ Done |
| form-noise + WASM | 2026-04-07 | Phase 2 |
| form-animate (gait/IK) | 2026-04-21 | Phase 3 |
| form-render (offline CPU) | 2026-05-05 | Phase 4 |
| TypeScript CLI + component API | 2026-05-19 | Phase 5 |
| SCORE ↔ FORM bridge | 2026-06-02 | Phase 6 |
| Bevy integration demo | 2026-06-23 | Phase 7 — key milestone |
| crates.io publish + docs site | 2026-07-07 | Phase 8 |

---

## PRIME dependency map

FORM consumes PRIME crates. Each phase lists its PRIME dependencies.
Pattern: Rust implementation → TS port → WASM drop-in (same API, no consumer changes).

| Form phase | PRIME dependency | Status |
|---|---|---|
| Phase 1 (form-sdf) | prime-sdf (Rust + TS) | ✅ Done — form-sdf re-exports prime-sdf |
| Phase 2 (form-noise) | prime-noise (Rust + TS port) | Waiting on prime-noise TS port |
| Phase 3 (form-animate) | prime-dynamics (Lorenz, RK4, splines) | Waiting on prime-dynamics TS port |
| Phase 4 (form-render) | prime-render pattern + prime-random (samplers) | Waiting on prime-render |
| Phase 5+ | prime-interp | TBD |

---

## Phase detail

### Phase 0 — Scaffold ✅ (2026-03-18)
- Cargo workspace root
- pnpm workspace root
- form-sdf crate skeleton
- Git repo initialized

---

### Phase 1 — form-sdf ✅ (2026-03-18)
**Crate: `form-sdf`. Dependency: `prime-sdf` only (thin re-export).**
**40/40 tests passing.**

2D SDF primitives:
- [x] circle
- [x] box_2d
- [x] rounded_box
- [x] capsule_2d
- [x] line_segment
- [x] triangle
- [x] ring

3D SDF primitives:
- [x] sphere
- [x] box_3d
- [x] capsule_3d
- [x] cylinder
- [x] torus
- [x] plane

CSG operations:
- [x] union
- [x] intersection
- [x] subtract
- [x] xor
- [x] smooth_union
- [x] smooth_intersection
- [x] smooth_subtract

Domain operations:
- [x] translate
- [x] rotate_2d
- [x] scale
- [x] repeat
- [x] mirror_x
- [x] mirror_y
- [x] elongate

---

### Phase 2 — form-noise (target: 2026-04-07)
**Crate: `form-noise`. PRIME dependency: `prime-noise` TS port.**

Build `form-noise` Rust crate in parallel with prime-noise. Integrate TS port when it ships.

Noise functions:
- [ ] Value noise (2D, 3D)
- [ ] Gradient noise (Perlin-style, 2D, 3D)
- [ ] Simplex noise (2D, 3D)
- [ ] FBM — fractal Brownian motion (octaves, lacunarity, gain)
- [ ] Domain warping (IQ-style — noise warps the input to another noise call)

WASM:
- [ ] Add `wasm32-unknown-unknown` target to `form-noise`
- [ ] `wasm-pack` build pipeline
- [ ] JS bindings (`form-noise-wasm` npm package)

Success criteria:
1. `cargo test -p form-noise` full coverage
2. Every public function has rustdoc with `# Math` section
3. FBM + domain warping visually correct (manual preview via form-render or PPM output)
4. WASM build green

---

### Phase 3 — form-animate (target: 2026-04-21)
**Crate: `form-animate`. PRIME dependency: `prime-dynamics` (Lorenz, RK4, spline math).**

Gait and procedural animation as pure mathematical functions.
No mesh, no keyframes — everything derived from trig, splines, and dynamics.

- [ ] Gait math: walk cycle from pure trig (lateral sway, vertical bob, sagittal rotation)
- [ ] Lorenz system driver: x=lateral, y=vertical, z=sagittal (chaos drives natural irregularity)
- [ ] Procedural IK: limb positioning from joint constraints + target
- [ ] Spring/damper: secondary motion (hair, cloth approximation)
- [ ] Bezier / Hermite splines: trajectory and easing helpers
- [ ] Time-parameterized output: `animate(t: f32) -> Pose` — pure function

Flagship target: **running-person demo**
- Ground state: sphere (mathematical truth)
- Corruption stack: bilateral symmetry break → brow ridge → occipital → temporal hollow → jaw → micro-surface FBM
- Gait driven by Lorenz system through full corruption stack
- No mesh files, no stored poses — all derived at evaluation time

---

### Phase 4 — form-render (target: 2026-05-05)
**Crate: `form-render`. PRIME dependencies: `prime-render` pattern + `prime-random` (samplers for probabilistic effects).**

Offline CPU ray marcher for previews, tests, and standalone renders.
Probabilistic rendering built on PRIME's sampler layer.

Core renderer:
- [ ] Sphere tracing loop (ray marching SDF fields)
- [ ] Normal estimation (finite difference gradient)
- [ ] Basic lighting (diffuse, specular, directional light)
- [ ] Shadow rays (hard shadows)
- [ ] PPM / PNG output

Probabilistic / diffusion (built on prime-random samplers):
- [ ] Soft shadows (Monte Carlo sampling along shadow ray)
- [ ] Ambient occlusion estimation (hemisphere sampling)
- [ ] Depth-of-field (aperture sampling, circle of confusion)
- [ ] Path tracing path (global illumination approximation — research phase)

GPU path (dev tool only):
- [ ] `wgpu` compute shader for ray marching
- [ ] Side-by-side CPU / GPU output comparison for validation

Success criteria:
1. Renders a sphere + box scene to PPM correctly
2. Soft shadows and AO visually plausible
3. Running-person demo renders offline at correct quality

---

### Phase 5 — TypeScript packages (target: 2026-05-19)
**Packages: `form-core`, `form-cli`. PRIME dependency: prime-interp (for spline/easing helpers in scene compiler).**

Scene files as plain TypeScript — same philosophy as SCORE songs.

`form-core`:
- [ ] Scene compiler (TS → SDF scene graph)
- [ ] Component API: `sphere()`, `union()`, `translate()` etc. mirror Rust API
- [ ] Scene type: pure description, no imperative mutations

`form-cli`:
- [ ] `form dev` — watch scene file, hot-reload render preview
- [ ] `form build` — compile scene to optimised SDF bytecode
- [ ] `form render` — offline render via form-render

---

### Phase 6 — form-score-bridge (target: 2026-06-02)
**Package: `form-score-bridge`. Peer dependency: `@score/core`.**

Map SCORE audio parameters → FORM visual parameters. Music drives geometry.

- [ ] BPM → animation speed
- [ ] Amplitude → scale / intensity
- [ ] Frequency → spatial frequency of noise
- [ ] Beat events → corruption stack triggers
- [ ] Live sync API: `bridge.tick(audioState) → sceneParams`

---

### Phase 7 — Engine integrations (target: 2026-06-23) — KEY MILESTONE
**Crate: `form-bevy`. Plugin API.**

- [ ] Bevy plugin: expose SDF scene graph as Bevy component
- [ ] Real-time GPU sphere tracing via Bevy render graph
- [ ] Demo: procedural running character in Bevy, real-time
- [ ] Unity C# bridge (research / spike)
- [ ] Unreal BP/C++ bridge (research / spike)

---

### Phase 8 — Release (target: 2026-07-07)
- [ ] Publish `form-sdf`, `form-noise`, `form-animate` to crates.io
- [ ] Docs site (mdBook)
- [ ] GitHub README with visual demos
- [ ] Blog post: "A running human in 50 lines of trig"
