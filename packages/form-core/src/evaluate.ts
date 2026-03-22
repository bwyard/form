/**
 * TypeScript SDF evaluator — pure recursive MARCH+EVALUATE.
 *
 * Evaluates an SdfNode tree at a point in 3D space.
 * This is the formal statement made executable:
 *
 *   evaluate(node, point) -> signed distance
 *
 * No STORE. No JUMP. Same node + same point = same result, always.
 *
 * When form-sdf WASM ships, this evaluator will be replaced by
 * a WASM-backed version under the same API. Scene descriptions do not change.
 */

import type { SdfNode, Vec3 } from './types.js'

// ---------------------------------------------------------------------------
// Vector helpers (pure, const-only)
// ---------------------------------------------------------------------------

const dot = ([ax, ay, az]: Vec3, [bx, by, bz]: Vec3): number =>
  ax * bx + ay * by + az * bz

const length = ([x, y, z]: Vec3): number =>
  Math.sqrt(x * x + y * y + z * z)

const sub = ([ax, ay, az]: Vec3, [bx, by, bz]: Vec3): Vec3 =>
  [ax - bx, ay - by, az - bz]

const abs3 = ([x, y, z]: Vec3): Vec3 =>
  [Math.abs(x), Math.abs(y), Math.abs(z)]

const max3 = ([ax, ay, az]: Vec3, [bx, by, bz]: Vec3): Vec3 =>
  [Math.max(ax, bx), Math.max(ay, by), Math.max(az, bz)]

const clamp = (v: number, lo: number, hi: number): number =>
  Math.max(lo, Math.min(hi, v))

// ---------------------------------------------------------------------------
// Primitive evaluators
// ---------------------------------------------------------------------------

const evalSphere = (p: Vec3, radius: number): number =>
  length(p) - radius

const evalBox3d = (p: Vec3, half: Vec3): number => {
  const q = sub(abs3(p), half) as Vec3
  return length(max3(q, [0, 0, 0]) as Vec3) + Math.min(Math.max(q[0], q[1], q[2]), 0)
}

const evalCapsule3d = ([x, y, z]: Vec3, radius: number, height: number): number => {
  const cy = clamp(y, 0, height)
  return length([x, y - cy, z] as Vec3) - radius
}

const evalCylinder = ([x, y, z]: Vec3, radius: number, height: number): number => {
  const dx = Math.sqrt(x * x + z * z) - radius
  const dy = Math.abs(y) - height * 0.5
  return Math.min(Math.max(dx, dy), 0) + length([Math.max(dx, 0), Math.max(dy, 0), 0] as Vec3)
}

const evalTorus = ([x, y, z]: Vec3, major: number, minor: number): number => {
  const q: Vec3 = [Math.sqrt(x * x + z * z) - major, y, 0]
  return length(q) - minor
}

const evalPlane = (p: Vec3, normal: Vec3, offset: number): number =>
  dot(p, normal) + offset

// ---------------------------------------------------------------------------
// Smooth blend helper (IQ polynomial smooth min)
// ---------------------------------------------------------------------------

const smin = (a: number, b: number, k: number): number => {
  const h = clamp(0.5 + 0.5 * (b - a) / k, 0, 1)
  return a + h * ((b - a) - k * h * (1 - h))
}

// ---------------------------------------------------------------------------
// Public evaluator
// ---------------------------------------------------------------------------

/**
 * Evaluate a signed distance field description at a point in 3D space.
 *
 * Returns the signed distance from `point` to the nearest surface described
 * by `node`. Positive = outside, negative = inside, zero = on surface.
 *
 * @param node  - the SDF description tree
 * @param point - the query point in 3D space
 * @returns signed distance in the same units as the scene
 *
 * @example
 * ```ts
 * import { sphere, translate } from './sdf.js'
 * import { evaluate } from './evaluate.js'
 *
 * const scene = translate(sphere(1.0), [0, 2, 0])
 * const d = evaluate(scene, [0, 2, 0]) // → -1.0 (inside the sphere)
 * ```
 */
export const evaluate = (node: SdfNode, point: Vec3): number => {
  switch (node.kind) {
    case 'sphere':         return evalSphere(point, node.radius)
    case 'box3d':          return evalBox3d(point, node.half)
    case 'capsule3d':      return evalCapsule3d(point, node.radius, node.height)
    case 'cylinder':       return evalCylinder(point, node.radius, node.height)
    case 'torus':          return evalTorus(point, node.major, node.minor)
    case 'plane':          return evalPlane(point, node.normal, node.offset)
    case 'union':          return Math.min(evaluate(node.a, point), evaluate(node.b, point))
    case 'intersection':   return Math.max(evaluate(node.a, point), evaluate(node.b, point))
    case 'subtract':       return Math.max(evaluate(node.a, point), -evaluate(node.b, point))
    case 'smooth_union':        return smin(evaluate(node.a, point), evaluate(node.b, point), node.k)
    case 'smooth_intersection': return -smin(-evaluate(node.a, point), -evaluate(node.b, point), node.k)
    case 'smooth_subtract':     return -smin(-evaluate(node.a, point), evaluate(node.b, point), node.k)
    case 'translate': {
      const [ox, oy, oz] = node.offset
      const [px, py, pz] = point
      return evaluate(node.node, [px - ox, py - oy, pz - oz])
    }
    case 'scale':
      return evaluate(node.node, point.map(v => v / node.factor) as unknown as Vec3) * node.factor
    case 'repeat': {
      const [px, py, pz] = point
      const [rx, ry, rz] = node.period
      const p2: Vec3 = [
        px - rx * Math.round(px / rx),
        py - ry * Math.round(py / ry),
        pz - rz * Math.round(pz / rz),
      ]
      return evaluate(node.node, p2)
    }
  }
}
