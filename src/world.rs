use std::collections::{HashMap, HashSet};

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

    pub fn create_player(&mut self, position: Position, health: Health, glyph: char) -> u32 {
        let entity = self.next_entity;
        self.next_entity += 1;

        self.positions.insert(entity, position);
        self.health.insert(entity, health);
        self.glyphs.insert(entity, glyph);
        self.players.insert(entity);

        entity
    }

    pub fn create_enemy(&mut self, position: Position, health: Health, glyph: char) -> u32 {
        let entity = self.next_entity;
        self.next_entity += 1;

        self.positions.insert(entity, position);
        self.health.insert(entity, health);
        self.glyphs.insert(entity, glyph);
        self.enemies.insert(entity);

        entity
    }
}