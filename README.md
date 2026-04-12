# VersusCore

VersusCore is a data-driven 2D (planned 2.5D) fighting-game engine built in Rust using the Bevy ECS framework. The goal is a moddable, creator-first platform — a spiritual successor to MUGEN where characters and stages are defined entirely through data files, not code.

## Requirements

- [Rust](https://rustup.rs/) (stable, 1.75+)
- Cargo (included with Rust)

## Running the Demo

```bash
cargo run -p vscr-demo
```

### Demo Controls

| Player 1 | Action |
|----------|--------|
| A / D | Move left / right |
| W | Jump |
| J | Attack |
| K | Throw projectile |
| F1 | Toggle debug / tuning mode |

## Running Tests

```bash
cargo test --workspace
```

Expected output: **17 tests, 0 failures**

- `vscr-core` — 15 unit tests covering movement, collision, hit resolution, and state machine
- `vscr-demo-test` — 2 integration tests validating scene config and asset integrity

## Crates

| Crate | Role |
|-------|------|
| `vscr-core` | Headless ECS simulation: components, systems pipeline, events, round flow |
| `vscr-demo` | Bevy application — rendering, input, animation, projectiles |
| `vscr-data` | TOML/JSON loaders for characters and stages |
| `vscr-debug` | Hitbox viewer, state overlay, frame-step tools (Phase 2) |
| `vscr-dsl` | VSCR scripting language parser and interpreter (Phase 2) |
| `vscr-demo-test` | Integration tests for data-driven scene loading |

## Architecture

The engine separates simulation from rendering. `vscr-core` runs a deterministic per-frame pipeline:

```
Input → State Machine → Movement → Collision → Hit Resolution → Time Freeze → Meters
```

Characters and stages are loaded from `assets/demo/scene.toml`. No gameplay logic is hardcoded.

## Documentation

- `docs/vision.md` — Project mission and design philosophy
- `docs/progress/2025-11-28_architecture-checkpoint.md` — Architecture design record
