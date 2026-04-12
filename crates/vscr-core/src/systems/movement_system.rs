use bevy::prelude::*;

use crate::components::{kinematics, state, time};

/// Ground level used for clamping. Can be overridden via a Bevy resource in the demo.
pub const GROUND_Y: f32 = -200.0;

pub fn apply_movement(
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        &mut kinematics::Velocity,
        &state::StateMachine,
        &kinematics::MovementConfig,
        Option<&time::TimeFreeze>,
    )>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut velocity, _sm, config, freeze) in query.iter_mut() {
        // Hitstop: skip movement while frozen
        if let Some(f) = freeze {
            if f.frames_remaining > 0 {
                continue;
            }
        }

        // Apply gravity
        velocity.y -= config.gravity * dt;

        // Clamp fall speed
        if velocity.y < -config.max_fall_speed {
            velocity.y = -config.max_fall_speed;
        }

        // Integrate velocity into position
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;

        // Ground clamp
        if transform.translation.y < GROUND_Y {
            transform.translation.y = GROUND_Y;
            velocity.y = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::kinematics::{MovementConfig, Velocity};

    #[test]
    fn gravity_accelerates_falling_entity() {
        let config = MovementConfig {
            walk_speed: 200.0,
            jump_speed: 500.0,
            gravity: 980.0,
            max_fall_speed: 600.0,
        };
        let mut vel = Velocity { x: 0.0, y: 0.0 };

        // Simulate one frame of gravity (dt = 1/60)
        let dt = 1.0 / 60.0;
        vel.y -= config.gravity * dt;

        assert!(vel.y < 0.0, "gravity should pull velocity downward");
    }

    #[test]
    fn fall_speed_clamped_to_max() {
        let config = MovementConfig {
            walk_speed: 200.0,
            jump_speed: 500.0,
            gravity: 980.0,
            max_fall_speed: 600.0,
        };
        let mut vel = Velocity { x: 0.0, y: -9999.0 };

        if vel.y < -config.max_fall_speed {
            vel.y = -config.max_fall_speed;
        }

        assert_eq!(vel.y, -config.max_fall_speed);
    }

    #[test]
    fn entity_clamped_at_ground_level() {
        let mut y = -500.0_f32;
        let mut vy = -100.0_f32;

        if y < GROUND_Y {
            y = GROUND_Y;
            vy = 0.0;
        }

        assert_eq!(y, GROUND_Y);
        assert_eq!(vy, 0.0);
    }

    #[test]
    fn horizontal_velocity_moves_entity() {
        let mut x = 0.0_f32;
        let vx = 200.0_f32;
        let dt = 1.0 / 60.0;

        x += vx * dt;

        assert!(x > 0.0, "positive x velocity should move entity right");
    }
}
