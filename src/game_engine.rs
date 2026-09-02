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

            for enemy in &mut self.enemies {
                enemy.update();
            }

            //fps
            let fps = 60.0;
            if let Some(delta_time) = self.get_delta_time() {
                if delta_time < 1.0 / fps {
                    std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / fps - delta_time));
                }
            }

            let unity_engine = UnityEngine::new();
            if unity_engine.is_initialized() {
                unity_engine.render();
            }
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

    pub fn get_delta_time(&self) -> Option<f32> {
        // Implement logic to calculate delta time between frames
        // Return Some(delta_time) if available, otherwise None
        None
    }
}
