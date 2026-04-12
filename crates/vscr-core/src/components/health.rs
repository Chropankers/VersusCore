use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_reduces_current_health() {
        let mut h = Health { current: 100, max: 100 };
        h.current -= 30;
        assert_eq!(h.current, 70);
    }

    #[test]
    fn fatal_damage_drops_health_to_zero() {
        let mut h = Health { current: 10, max: 100 };
        h.current -= 10;
        assert!(h.current <= 0);
    }

    #[test]
    fn overkill_damage_goes_negative() {
        let mut h = Health { current: 5, max: 100 };
        h.current -= 20;
        assert!(h.current < 0);
    }
}
