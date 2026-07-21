use std::collections::{HashMap, HashSet};
use crate::map::Position;

pub struct Health {
    pub current: i32,
    pub max: i32,
}

pub struct World {
    pub next_entity: u32,
    pub positions:   HashMap<u32, Position>,
    pub health:      HashMap<u32, Health>,
    pub glyphs:      HashMap<u32, char>,
    pub players:     HashSet<u32>,
    pub enemies:     HashSet<u32>,
}

impl World {
    pub fn new() -> Self {
        World {
            next_entity: 0,
            positions: HashMap::new(),
            health: HashMap::new(),
            glyphs: HashMap::new(),
            players: HashSet::new(),
            enemies: HashSet::new(),
        }
    }

    pub fn spawn_player(&mut self, position:Position) -> u32{
        let entity = self.next_entity;

        //increment for next time
        self.next_entity += 1;

        self.positions.insert(entity, position);
        self.glyphs.insert(entity, "@");
        self.health.insert(entity, 10);
        

    }

    pub fn spawn_enemy
}