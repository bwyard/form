# FORM — Thesis

**The formal claim Form makes:**

> A scene is a pure function of space.
> `shape = f(description, position)`

The object does not exist as mutable state. It exists as a mathematical fact — timeless, immutable, evaluable from any point in space. The SDF is the object.

---

## The formal statement

```
f(point: Vec3) -> f32
```

The return value is the signed distance from `point` to the nearest surface. Positive outside, negative inside, zero on the surface. This single function is sufficient to describe any shape.

With animation, time is a parameter — not a trigger:

```
f(point: Vec3, t: f32) -> f32
```

The complete formal statement for a human figure:

```
human(point: Vec3, t: f32) -> f32
```

No mesh. No keyframes. No stored poses. Same inputs, same output, every time.

---

## The proof strategy: Controlled Corruption

A sphere is mathematically exact. A human is not. The question Form answers is: *can biological complexity be expressed as a pure function?*

The corruption model says yes. Every scene has two layers:

**1. Ground state** — mathematical truth.

```
sphere(point, radius) -> f32
```

**2. Corruption stack** — a sequence of pure functions, each adding physical or biological irregularity.

```
sphere(point, 1.0)
  |> bilateral_symmetry_break(asymmetry: 0.03)
  |> brow_ridge(prominence: 0.8, width: 0.6)
  |> occipital_protrusion(angle: 15.0)
  |> temporal_hollow(depth: 0.04)
  |> jaw_shape(width: 0.9, angle: 12.0)
  |> micro_surface_fbm(octaves: 6, amplitude: 0.005)
```

Each step is `f(sdf, params) -> sdf` — a pure function that takes a signed distance field and returns a signed distance field. The full stack is function composition. No step stores state. No step mutates geometry.

The result is still a pure function. The corruption is in the *description*, not in any stored data.

**Corruption is Form's proof strategy** — not a technique for making things look organic. It answers the sceptic's challenge: *"surely complex geometry needs stored data."* The running-person demo is the concrete refutation. A human running, fully derived at evaluation time, from nothing but composition of pure functions over `(point, t)`.

---

## The assembly rule: No STORE. No JUMP.

Form follows the same assembly rule as Score, in the spatial domain.

```
LOAD    cursor          // read current position
EVAL    f(cursor)       // pure computation — no side effects
APPEND  result          // emit the output
ADVANCE cursor          // move forward
```

| | Score | Form |
|---|---|---|
| Axis | Time (`n: u64`) | Space (`p: Vec3`) |
| Scan loop | Sample loop | Ray march |
| Rule | APPEND+ADVANCE | MARCH+EVALUATE |
| Pure function | `f(song, n, sr)` | `f(description, p)` |
| No STORE | No audio buffers in DSL layer | No mesh, no stored geometry |
| No JUMP | No branches to cached samples | No lookup tables, no precomputed normals |

**No STORE** — no mesh files, no geometry tables, no stored normals, no keyframes. The corruption stack does not store intermediate geometry — it evaluates it.

**No JUMP** — the ray marcher does not branch to precomputed data. It marches forward through space, evaluating the SDF at each step. Every level of detail — including micro-surface FBM — is evaluation, not lookup.

---

## Relation to the ecosystem

Form is one of four simultaneous proofs of the same formal claim:

> Mutation is not required for real-world computation. A scan loop and explicit state threading are sufficient.

| Project | Domain | Formal claim |
|---|---|---|
| Prime | Foundation | Pure functions exist with no domain — computation without side effects |
| Score | Music | `output[n] = f(song, n, sr)` — composition is a pure function of time |
| **Form** | **Visual art** | **`shape = f(description, position)` — scene is a pure function of space** |
| Stage | Experience | `experience = f(world, t, pos)` — experience is a pure function of state and time |

These are not four separate projects that happen to share a philosophy. They share one computational model — LOAD, EVAL, APPEND, ADVANCE — instantiated with four different axis types. Prime is the shared foundation all four compile onto.

**Convergent evidence:** four independent domains implementing the same architecture is stronger evidence than one. If the claim were domain-specific, it would not generalise. It generalises.

---

## What Form does not prove

Form does not prove that pure functions are *practical* for production rendering. GPU pipelines, rasterisation, and mesh-based workflows exist for good reasons — performance at scale. Form proves the *mathematical* claim: that scenes can be *described* as pure functions and *evaluated* without mutable state.

The render layer (Phase 4) is a CPU ray marcher — a proof-of-concept evaluator, not a production renderer. The claim is about the description, not the pipeline.

---

## The flagship proof-of-concept

**Running-person demo** — a human figure running, fully derived at evaluation time.

```
human(point: Vec3, t: f32) -> f32
```

Implementation:
- Ground state: sphere
- Corruption stack: bilateral symmetry break → brow ridge → occipital → temporal hollow → jaw → micro-surface FBM
- Gait: Lorenz system drives lateral sway, vertical bob, sagittal rotation as a function of `t`
- No mesh files. No stored poses. No keyframes. No mutable state.

If this function compiles, runs, and produces a recognisable human figure in motion — the thesis is proved for the visual domain.
