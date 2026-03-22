import init, { render_frame } from './wasm/form_wasm.js'

await init()

const canvas  = document.getElementById('canvas')
const ctx     = canvas.getContext('2d')
const btn     = document.getElementById('btn')
const timeEl  = document.getElementById('time')

const W = canvas.width
const H = canvas.height

let t       = 0
let running = true
let last    = null

btn.addEventListener('click', () => {
  running = !running
  btn.textContent = running ? 'pause' : 'play'
  if (running) requestAnimationFrame(frame)
})

const frame = (now) => {
  if (!running) return

  const dt = last ? Math.min((now - last) / 1000, 0.05) : 0
  last = now
  t += dt

  const rgba = render_frame(t, W, H)
  ctx.putImageData(new ImageData(rgba, W, H), 0, 0)
  timeEl.textContent = `t = ${t.toFixed(3)}`

  requestAnimationFrame(frame)
}

requestAnimationFrame(frame)
