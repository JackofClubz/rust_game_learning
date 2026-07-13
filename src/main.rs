use bracket_lib::{prelude::*, terminal::VirtualKeyCode::D};
mod map;
mod player;
mod enemy;

use map::{Map, Position, DijkstraMap};
use player::{Player, PlayerAction, handle_input};

use crate::enemy::Enemy;


fn main () {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Game")
        .build()
        .unwrap();

    main_loop(context, State::new()).unwrap();
}

pub struct State{
    player: Player,
    map: Map,
    enemies: Vec<Enemy>, 
}

impl State {
    pub fn new() -> Self {
        let map = Map::new(80, 50);
        let starting_position = map.starting_position;
        let rooms = map.rooms.clone();
        State {
            player: Player::new(starting_position),
            map,
            // enemies only exist in 2 random rooms, not the starting room
            enemies: rooms.iter().skip(1)
                .take(2)
                .map(|room| Enemy::new(Position { x: room.x + room.width / 2, y: room.y + room.height / 2 }))
                .collect(),
        }
    }
}

impl GameState for State{
    fn tick(&mut self, ctx:&mut BTerm){
        // Read input and update player position
        if let Some(action) = handle_input(ctx){
            match action{
                PlayerAction::Move(dx, dy) => {
                    let new_x = self.player.position.x + dx;
                    let new_y = self.player.position.y + dy;
                    if self.map.can_enter(new_x, new_y){
                        self.player.position.x = new_x;
                        self.player.position.y = new_y; 
                    }
                    let dijkstra = DijkstraMap::new(&self.map, self.player.position);

                    for enemy in self.enemies.iter_mut() {
                        //check all neighbor tiles
                        let enemy_idx = idx(enemy.position.x, enemy.position.y);
                        for (dx, dy) in [(-1, 0), (1,0), (0,-1), (0,1)].iter(){
                            let nx = enemy.position.x + dx;
                            let ny = enemy.position.y + dy;
                            if self.map.can_enter(nx, ny){
                                let neighbor_idx = idx(nx,ny);
                                if dijkstra.distance[neighbor_idx] < dijkstra.distance[enemy_idx]{
                                    enemy.position.x = nx;
                                    enemy.position.y = ny;
                                    break;
                                }else{
                                    // Enemy is already at the closest position to the player, so it waits
                                    PlayerAction::Wait;
                                }
                        }


                        
                    }
                }
                PlayerAction::Wait => {},
                PlayerAction::Quit => ctx.quit(),
            }
            map::update_fov(self.player.position, 3, &mut self.map);
        }
        // Render the map and player
        self.map.render(ctx);
        self.player.render(ctx);
    }
}