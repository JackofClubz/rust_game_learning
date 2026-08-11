// game engine

pub mod map;
pub mod player;
pub mod enemy;

pub struct GameEngine {
    pub map: map::Map,
    pub player: player::Player,
    pub enemies: Vec<enemy::Enemy>,
}

impl GameEngine {
    pub fn new() -> Self {
        GameEngine {
            map: map::Map::new(),
            player: player::Player::new(),
            enemies: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        // Update game logic here
        self.player.update();
        for enemy in &mut self.enemies {
            enemy.update();
        }
    }

    pub fn render(&self) {
        // Render game state here
        self.map.render();
        self.player.render();
        for enemy in &self.enemies {
            enemy.render();
        }
    }

    pub fn add_enemy(&mut self, enemy: enemy::Enemy) {
        self.enemies.push(enemy);
    }
    pub unity_engine(&mut self) {
        // Unity engine specific logic here
        let mut unity_engine = UnityEngine::new();
        unity_engine.initialize();
        render();
    }
}
