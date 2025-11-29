use bevy::prelude::*;

use crate::components::resources::Meters;
use crate::events::HitEvent;

/// Very simple: attacker gains meter on hit.
pub fn update_meters(
    mut meters_query: Query<&mut Meters>,
    mut hit_reader: EventReader<HitEvent>,
) {
    for hit in hit_reader.read() {
        if let Ok(mut meters) = meters_query.get_mut(hit.attacker) {
            meters.super_meter += 5.0;
            // clamp or normalize later as needed
        }
    }
}
