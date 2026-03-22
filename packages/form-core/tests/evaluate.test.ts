import { describe, it, expect } from 'vitest'
import {
  sphere, box3d, capsule3d, cylinder, torus, plane,
  union, intersection, subtract, smoothUnion,
  translate, scale, repeat,
  evaluate,
} from '../src/index.js'

const EPSILON = 1e-4

describe('sphere', () => {
  it('returns -radius at centre', () => {
    expect(evaluate(sphere(1), [0, 0, 0])).toBeCloseTo(-1, 4)
  })
  it('returns 0 on surface', () => {
    expect(evaluate(sphere(1), [1, 0, 0])).toBeCloseTo(0, 4)
  })
  it('returns positive outside', () => {
    expect(evaluate(sphere(1), [2, 0, 0])).toBeCloseTo(1, 4)
  })
  it('is deterministic', () => {
    expect(evaluate(sphere(0.5), [0.3, 0.4, 0])).toBe(evaluate(sphere(0.5), [0.3, 0.4, 0]))
  })
})

describe('box3d', () => {
  it('returns negative inside', () => {
    expect(evaluate(box3d([1, 1, 1]), [0, 0, 0])).toBeLessThan(0)
  })
  it('returns 0 on face centre', () => {
    expect(evaluate(box3d([1, 1, 1]), [1, 0, 0])).toBeCloseTo(0, 4)
  })
  it('returns positive outside', () => {
    expect(evaluate(box3d([1, 1, 1]), [2, 0, 0])).toBeCloseTo(1, 4)
  })
})

describe('union', () => {
  it('returns min of two distances', () => {
    const s1 = sphere(1)
    const s2 = translate(sphere(1), [3, 0, 0])
    const u = union(s1, s2)
    const d1 = evaluate(s1, [1.5, 0, 0])
    const d2 = evaluate(s2, [1.5, 0, 0])
    expect(evaluate(u, [1.5, 0, 0])).toBeCloseTo(Math.min(d1, d2), 4)
  })
  it('point inside either shape is inside union', () => {
    expect(evaluate(union(sphere(1), translate(sphere(1), [3, 0, 0])), [0, 0, 0])).toBeLessThan(0)
    expect(evaluate(union(sphere(1), translate(sphere(1), [3, 0, 0])), [3, 0, 0])).toBeLessThan(0)
  })
})

describe('intersection', () => {
  it('point outside both is outside intersection', () => {
    const i = intersection(sphere(1), translate(sphere(1), [3, 0, 0]))
    expect(evaluate(i, [0, 0, 0])).toBeGreaterThan(0)
  })
})

describe('subtract', () => {
  it('point inside a but inside b is outside result', () => {
    const s = subtract(sphere(2), sphere(1))
    expect(evaluate(s, [0, 0, 0])).toBeGreaterThan(0)
  })
  it('point inside a but outside b is inside result', () => {
    const s = subtract(sphere(2), translate(sphere(1), [5, 0, 0]))
    expect(evaluate(s, [0, 0, 0])).toBeLessThan(0)
  })
})

describe('smoothUnion', () => {
  it('blends between two spheres', () => {
    const hard = evaluate(union(sphere(1), translate(sphere(1), [2, 0, 0])), [1, 0, 0])
    const soft = evaluate(smoothUnion(sphere(1), translate(sphere(1), [2, 0, 0]), 0.5), [1, 0, 0])
    expect(soft).toBeLessThan(hard)
  })
})

describe('translate', () => {
  it('moves the shape', () => {
    const s = translate(sphere(1), [5, 0, 0])
    expect(evaluate(s, [5, 0, 0])).toBeCloseTo(-1, 4)
    expect(evaluate(s, [6, 0, 0])).toBeCloseTo(0, 4)
  })
})

describe('scale', () => {
  it('scales the shape uniformly', () => {
    const s = scale(sphere(1), 2)
    expect(evaluate(s, [0, 0, 0])).toBeCloseTo(-2, 4)
    expect(evaluate(s, [2, 0, 0])).toBeCloseTo(0, 4)
  })
})

describe('plane', () => {
  it('returns positive above the plane', () => {
    const p = plane([0, 1, 0], 0)
    expect(evaluate(p, [0, 1, 0])).toBeCloseTo(1, 4)
  })
  it('returns negative below the plane', () => {
    const p = plane([0, 1, 0], 0)
    expect(evaluate(p, [0, -1, 0])).toBeCloseTo(-1, 4)
  })
})
