# VersusCore

VersusCore is a data-driven 2D (Planned 2.5D) fighting-game engine implemented in Rust using Bevy.
The architecture follows an ECS core with a data & DSL layer on the left and an
output & debug layer on the right (see `docs/architecture/versuscore-ecs-diagram.png`).

## Crates

- `vscr-core` – ECS core: Entity Registry, System Scheduler, Components, Simulation Systems Pipeline.
- `vscr-dsl` – VSCR fighting-game DSL and parser.
- `vscr-data` – JSON/config loaders for characters, stages, and VSCR scripts.
- `vscr-debug` – Hitbox viewer, state overlay, frame-step debugging tools.
- `vscr-demo` – Bevy application that runs a VersusCore match.

See `/docs/design-notes.md` for the primitive list and mechanics mapping.
