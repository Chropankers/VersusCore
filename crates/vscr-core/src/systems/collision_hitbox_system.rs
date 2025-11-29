use bevy::prelude::*;

use crate::components::{
    collider::ColliderAabb,
    tags::TeamTag,
};
use crate::events::HitEvent;

/// Naive O(n²) AABB overlap detection that emits HitEvent.
/// Attacker = entity with is_hitbox == true; victim = other entity.
pub fn detect_hits(
    mut hit_writer: EventWriter<HitEvent>,
    query: Query<(Entity, &Transform, &ColliderAabb, Option<&TeamTag>)>,
) {
    let entities: Vec<_> = query.iter().collect();

    for i in 0..entities.len() {
        for j in (i + 1)..entities.len() {
            let (ent_a, tf_a, col_a, team_a) = entities[i];
            let (ent_b, tf_b, col_b, team_b) = entities[j];

            // Require different teams if both have teams
            if let (Some(ta), Some(tb)) = (team_a, team_b) {
                if ta.team == tb.team {
                    continue;
                }
            }

            // Compute world AABBs
            let (min_a, max_a) = world_aabb(tf_a, col_a);
            let (min_b, max_b) = world_aabb(tf_b, col_b);

            if aabb_overlaps(min_a, max_a, min_b, max_b) {
                match (col_a.is_hitbox, col_b.is_hitbox) {
                    (true, false) => {
                        hit_writer.send(HitEvent {
                            attacker: ent_a,
                            victim: ent_b,
                            damage: 10,
                            hitstun_frames: 20,
                        });
                    }
                    (false, true) => {
                        hit_writer.send(HitEvent {
                            attacker: ent_b,
                            victim: ent_a,
                            damage: 10,
                            hitstun_frames: 20,
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

fn world_aabb(transform: &Transform, collider: &ColliderAabb) -> (Vec2, Vec2) {
    let center = Vec2::new(
        transform.translation.x,
        transform.translation.y,
    ) + collider.offset;
    let he = collider.half_extents;
    let min = center - he;
    let max = center + he;
    (min, max)
}

fn aabb_overlaps(min_a: Vec2, max_a: Vec2, min_b: Vec2, max_b: Vec2) -> bool {
    min_a.x <= max_b.x
        && max_a.x >= min_b.x
        && min_a.y <= max_b.y
        && max_a.y >= min_b.y
}
