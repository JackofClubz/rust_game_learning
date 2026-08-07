//attack animations and actions

mod attack;
use attack::attack_entity;
use crate::world::World;
use crate::map::Map;    


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

impl attack::AttackAction {
    pub fn new(attacker_id: u32, target_id: u32, damage: i32) -> Self {
        AttackAction {
            attacker_id,
            target_id,
            damage,
        }
    }

    pub fn execute(&self, world: &mut World, map: &Map) {
        perform_attack(world, map, self.attacker_id, self.target_id, self.damage);
    }

    pub fn render(&self, ctx: &mut bracket_lib::prelude::BTerm, world: &World) {
        attack_entity(ctx, world, self.attacker_id, self.target_id);
    }
}