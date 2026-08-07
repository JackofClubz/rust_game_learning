// game engine

pub mod map;
pub mod player;
pub mod enemy;

pub struct GameEngine {
    pub map: map::Map,
    pub player: player::Player,
    pub enemies: Vec<enemy::Enemy>,
}