use bracket_lib::prelude::*;
// render attack animations and actions

pub struct AttackAction {
    pub attacker_id: u32,
    pub target_id: u32,
    pub damage: i32,
}

pub fn perform_attack(world: &mut World, map: &Map, attacker_id: u32, target_id: u32, damage: i32) {
    // Check if the target is still alive
    if let Some(health) = world.health.get_mut(&target_id) {
        health.current -= damage;
        if health.current <= 0 {
            // Target dies — remove all its components
            world.positions.remove(&target_id);
            world.glyphs.remove(&target_id);
            world.health.remove(&target_id);
            world.players.remove(&target_id);
            world.enemies.remove(&target_id);
        }
    }
}