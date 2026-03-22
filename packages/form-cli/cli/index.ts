#!/usr/bin/env node
/**
 * form CLI — `form dev | build | render`
 *
 * Scene files are plain TypeScript.
 * The CLI watches, compiles, and renders them.
 * No STORE. No JUMP. The scene is a pure function of space.
 */

const [,, command, ...args] = process.argv

const commands = ['dev', 'build', 'render'] as const
type Command = typeof commands[number]

const isCommand = (s: string): s is Command => (commands as readonly string[]).includes(s)

const run = (cmd: Command, _args: readonly string[]): void => {
  switch (cmd) {
    case 'dev':
      // TODO: watch scene file, hot-reload render preview (Phase 5 implementation)
      console.log('form dev — not yet implemented')
      break
    case 'build':
      // TODO: compile scene description to optimised SDF bytecode (Phase 5 implementation)
      console.log('form build — not yet implemented')
      break
    case 'render':
      // TODO: offline render via form-render (Phase 4 dependency)
      console.log('form render — not yet implemented')
      break
  }
}

if (command === undefined || !isCommand(command)) {
  console.log('Usage: form <dev|build|render> [options]')
  process.exit(1)
}

run(command, args)
