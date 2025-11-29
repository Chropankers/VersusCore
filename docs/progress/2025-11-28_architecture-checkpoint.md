# VersusCore – Architecture Checkpoint Report  
**Date:** Nov 28, 2025  
**Stage:** Post–Initial Architecture, Pre–Core Implementation**

---

## 1. Project State Overview

VersusCore has exited the conceptual phase and entered structured architectural planning. The GitHub repository is initialized with a clean multi-crate workspace that matches the early engine diagram and the long-term engine philosophy.

The goal at this phase is *not* to build gameplay yet, but to ensure the core crate (`vscr-core`) has a stable foundation, a defined module structure, and the beginnings of a deterministic Simulation Pipeline.

The project is now positioned for **Phase 1 Sprint** work.

---

## 2. Repository Layout (Stable)

The repo has been structured as a multi-crate workspace:

```
versuscore/
├─ Cargo.toml (workspace)
├─ crates/
│  ├─ vscr-core/    (engine logic)
│  ├─ vscr-data/    (JSON + config layer)
│  ├─ vscr-dsl/     (VSCR language, parser)
│  ├─ vscr-debug/   (hitbox viewer, overlays)
│  └─ vscr-demo/    (Bevy app)
└─ docs/
```

This layout is **stable and future-proof**. No expected major deviations.

---

## 3. Core Architectural Decisions (Locked In)

### 3.1 ECS as the Heart of the Engine
- All simulation logic lives in `vscr-core`.
- Systems are ordered through a strict pipeline.
- `vscr-demo` depends on `vscr-core`, not vice versa.

### 3.2 Simulation Systems Pipeline (Finalized Order)

1. **Input System**  
2. **State Machine System**  
3. **Movement System**  
4. **Collision & Hitbox System**  
5. **Hit Resolution System**  
6. **Time / Freeze System**  
7. **Resource & Meter System**  

The order is enforced via `.chain()` for deterministic progression.

### 3.3 Round Flow Architecture
- `RoundState` resource tracks: Intro → Ready → Fight → KO Freeze → Result.
- `RoundStateEvent` manages transitions.
- This flow is independent of gameplay logic.

### 3.4 Component Vocabulary (Initial Set Finalized)
Planned and partially stubbed components:

- **Identity:** `PlayerTag`, `CharacterTag`, `TeamTag`
- **Kinematics:** `Velocity`, `MovementConfig`
- **Collision:** `ColliderAabb`
- **Combat:** `Health`
- **Input:** `InputBuffer`, `Buttons`
- **State:** `StateMachine`, `CharacterState`
- **Time:** `TimeFreeze`, `GlobalTimeScale`
- **Resources:** `Meters`

These form the engine’s **atomic primitives**.

### 3.5 Event Vocabulary (Finalized for Phase 1)
- `HitEvent`
- `KoEvent`
- `RoundStateEvent`

These events form the communication layer between systems.

### 3.6 Output Bridge
- Engine emits simulation data.  
- `vscr-demo` handles rendering, animation, and UI.  
- `vscr-core` remains **headless simulation**.

---

## 4. Systems Implemented as Stubs

### 4.1 Collision / Hitbox Detection
- Naive O(n²) AABB overlap.
- Distinguishes hitboxes vs. hurtboxes.
- Emits `HitEvent`.

### 4.2 Hit Resolution
- Applies damage.
- Sets `Hitstun`.
- Applies `TimeFreeze`.
- Emits `KoEvent`.

### 4.3 Time Freeze
- Per-entity freeze countdown.
- Optional global freeze via `GlobalTimeScale`.

### 4.4 Resource & Meter System
- Attacker gains meter per hit.

These stubs are simple but **structurally correct**.

---

## 5. Work Not Yet Begun (But Defined)

### 5.1 Input System
- Requires a `PlayerInputState` in `vscr-demo`.
- Maps keyboard/controller → logical `Buttons`.

### 5.2 State Machine System
- Needs logic for transitions:  
  `Idle → Walk → Jump → Attack → Hitstun → KO`

### 5.3 Movement System
- Gravity, velocity, clamping.
- Must respect `TimeFreeze`.

### 5.4 Character Data Loading
- `CharacterDef` JSON shape not yet finalized.
- Tools dependent on JSON format.

### 5.5 VSCR DSL Runtime
- Interpreter stub exists.
- DSL work intentionally deferred to Phase 2.

---

## 6. Open Decisions (Future Work Required)

### 6.1 Hitbox vs Hurtbox Data Model
- Currently unified (`ColliderAabb`).
- Future: separate `HitboxSet` and `HurtboxSet`.

### 6.2 Transform Abstraction
- Custom `Transform2D` vs. Bevy `Transform`.

### 6.3 Resource/Meter Depth
- Guard break, burst, tension, etc. to be added later.

### 6.4 Animation Synchronization
- Requires integrating state + frame data.

### 6.5 DSL (VSCR) Integration
- Will layer on once the ECS base is stable.

---

## 7. Recommended Next Steps (Clear Action Plan)

### Step 1 — Finish Component Implementations
Implement or refine:

- `MovementConfig`  
- `StateMachine`  
- `InputBuffer`  
- `ColliderAabb`  

### Step 2 — Implement Basic Systems
Priority order:
1. **Input System**  
2. **State Machine System**  
3. **Movement System**

This will enable basic locomotion and state changes.

### Step 3 — Spawn Two Test Characters
In `vscr-demo`, spawn characters with:

- Tags  
- Transform  
- Health  
- MovementConfig  
- ColliderAabb  

Goal: **boxes that can walk and collide**.

### Step 4 — Implement Round Flow Transitions
Even simple Intro/Ready/Fight/KO placeholders are enough.

### Step 5 — Define `CharacterDef` JSON
Minimal schema:
- `walk_speed`
- `jump_speed`
- `max_health`
- collider sizes

Tools can come **after** format stabilizes.

---

## 8. General Project Health

### Strengths
- Architecture deeply considered and robust.  
- Crate structure supports long-term modding.  
- Systems Pipeline fully specified.  
- Event model clean and engine-ready.  
- Vision/Mission provide strong project direction.

### Risks
- Rust + Bevy learning curve may slow Weeks 2–3.  
- Character data format must stabilize early.  
- DSL must not be started prematurely.

**Mitigation:** Focus on a playable **vertical slice** before tools or DSL.

---

## 9. Summary

VersusCore’s architecture is **stable, well-defined, and ready for implementation**.

The foundation of a deterministic, data-driven fighting game engine is fully in place:

- Core crate structure  
- Simulation pipeline  
- Component layout  
- Event model  
- Round flow  
- Basic system skeletons  

**Next step:** transition from architecture → implementation, starting with input, movement, and state transitions.
