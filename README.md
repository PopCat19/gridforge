# Gridforge

Desktop audio application built with Tauri + SolidJS. Modular synthesis engine with plugin architecture.

## Status Summary

| Module | Status | Description |
|--------|--------|-------------|
| M0 | Done | Tauri + SolidJS app shell |
| M1 | Done | Audio engine with real-time mixing |
| M2 | Done | ChipSynth plugin (8-bit synthesis) |
| M3 | Done | Sequencer with pattern-based playback |

## Setup

```bash
# Clone repository
git clone <repo-url>
cd gridforge

# Enter development shell (Nix)
nix develop
# OR
nix-shell

# Install frontend dependencies
bun install

# Run development server
bun tauri dev
```

## Architecture Boundaries

### Frontend Never Synthesizes Audio

The frontend (SolidJS/TypeScript) never synthesizes audio. It only:
- Renders UI state
- Sends commands to backend via Tauri IPC
- Receives display data (waveform, meters) from backend

All audio processing happens in the Rust backend.

### Tauri IPC Communication Pattern

Frontend invokes backend commands via `invoke()`:

```typescript
import { invoke } from "@tauri-apps/api/core";

// Send note event
await invoke("note_on", { note: 60, velocity: 100 });

// Load plugin
await invoke("load_plugin", { plugin_type: "chip_synth" });
```

Backend exposes commands in `src-tauri/src/commands.rs`.

### SoundSource Trait and Plugin System

Audio plugins implement the [`SoundSource`](src-tauri/src/plugin/traits.rs:149) trait:

```rust
pub trait SoundSource: Send + Sync {
    fn process(&mut self, buffer: &mut [f32], sample_rate: u32);
    fn note_on(&mut self, note: Note, velocity: Velocity);
    fn note_off(&mut self, note: Note);
    fn set_parameter(&mut self, id: &ParamId, value: ParamValue);
    fn get_parameters(&self) -> Vec<Parameter>;
}
```

Plugin implementations live in `src-tauri/src/plugin/`. Current implementation:
- [`chip_synth.rs`](src-tauri/src/plugin/chip_synth.rs) - 8-bit style synthesizer

## Commit Conventions

Format: `<type>(<scope>): <verb> <summary>`

Types: `feat` `fix` `refactor` `docs` `style` `test` `chore` `perf`

Examples:
```
feat(audio): add chip synth oscillator types
fix(sequencer): handle note-off events correctly
docs(readme): clarify installation steps
```

## Validation Commands

```bash
# Enter dev shell
nix develop
# OR
nix-shell

# Install dependencies
bun install

# Run development server
bun tauri dev

# Check Rust formatting
cargo fmt --check

# Check frontend code
bun run check
```

## Leptos Revisit Criteria

The current architecture uses SolidJS for the frontend. If future requirements demand server-side rendering or more sophisticated state management, Leptos could be revisited. The current decision stands because:

- SolidJS provides fine-grained reactivity suitable for real-time audio UI
- Tauri IPC pattern is well-established and performant
- No SSR requirements currently exist

This decision can be revisited when:
- SSR becomes a hard requirement
- WebAssembly audio processing needed on frontend
- Complex state management exceeds SolidJS capabilities
