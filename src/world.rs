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
        self.glyphs.insert(entity, '@');
        self.health.insert(entity, Health{current: 10, max:10});
        self.players.insert(entity);
        entity
    }

    pub fn spawn_enemy(&mut self, position:Position) -> u32{
        let entity = self.next_entity;

        //increment for next time
        self.next_entity += 1;

        self.positions.insert(entity, position);
        self.glyphs.insert(entity, 'E');
        self.health.insert(entity, Health{current: 10, max:10});
        self.enemies.insert(entity);
        entity
    }

    pub fn player_id(&self) -> u32{
        *self.players.iter().next().unwrap()
    }

    pub fn player_position(&self) -> Position{
        let id = self.player_id();
        *self.positions.get(&id).unwrap()
    }

    pub fn enemy_ids(&self) -> Vec<u32>{
        self.enemies.iter().cloned().collect()
    }

    pub fn enemy_positions(&self) -> Vec<Position>{
        let ids = self.enemy_ids();
        ids.iter().map(|id| *self.positions.get(id).unwrap()).collect()
    }

    pub fn enemy_at(&self, x: i32, y: i32) -> Option<u32> {
        for &enemy_id in self.enemies.iter() {
            if let Some(pos) = self.positions.get(&enemy_id) {
                if pos.x == x && pos.y == y {
                    return Some(enemy_id);
                }
            }
        }
        None  
    }

    
}