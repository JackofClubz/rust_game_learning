use bracket_lib::{prelude::*, terminal::VirtualKeyCode::D};
mod map;
mod player;
mod enemy;
mod world;

use map::{Map, Position, DijkstraMap};
use player::{Player, PlayerAction, handle_input};
use world::World;

use crate::enemy::Enemy;


fn main () {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Game")
        .build()
        .unwrap();

    main_loop(context, State::new()).unwrap();
}

pub struct State{
    world: World,
    map: Map,
}

impl State {
    pub fn new() -> Self {
        let map = Map::new(80, 50);
        let starting_position = map.starting_position;
        let rooms = map.rooms.clone();
        let mut world = World::new();

         // spawn player at starting position
        world.spawn_player(map.starting_position);
        
        // spawn enemies in other rooms
        map.rooms.iter()
            .skip(1)
            .take(2)
            .for_each(|room| {
                world.spawn_enemy(room.centre_point());
            });
        
        State { world, map }
    }
}

impl GameState for State{
    fn tick(&mut self, ctx:&mut BTerm){
        // Read input and update player position
        if let Some(action) = handle_input(ctx){
            match action{
                PlayerAction::Move(dx, dy) => {
                    let new_x = self.world.player_position().x + dx;
                    let new_y = self.world.player_position().y + dy;
                    if self.map.can_enter(new_x, new_y){
                        self.world.positions.insert(self.world.player_id(), Position { x: new_x, y: new_y });
                    }
                    let dijkstra = DijkstraMap::new(&self.map, self.world.player_position());

                    for enemy_id in self.world.enemies.iter(){
                        let enemy_position = self.world.positions.get(enemy_id).unwrap();
                        let enemy_idx = (enemy_position.y * dijkstra.width + enemy_position.x) as usize;
                        for (ex, ey) in [(-1, 0), (1,0), (0,-1), (0,1)].iter(){
                            let nx = enemy_position.x + ex;
                            let ny = enemy_position.y + ey;
                            if self.map.can_enter(nx, ny){
                                let neighbor_idx = (ny * dijkstra.width + nx) as usize;
                                if let Some(nd) = dijkstra.distance[neighbor_idx] {
                                    if let Some(ed) = dijkstra.distance[enemy_idx] {
                                        if nd < ed {
                                            self.world.positions.insert(*enemy_id, Position { x: nx, y: ny });
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // for enemy in self.world.enemies.iter() {
                    //     //check all neighbor tiles
                    //     let enemy_idx = (enemy.position.y * dijkstra.width + enemy.position.x) as usize;
                    //     for (ex, ey) in [(-1, 0), (1,0), (0,-1), (0,1)].iter(){
                    //         let nx = enemy.position.x + ex;
                    //         let ny = enemy.position.y + ey;
                    //         if self.map.can_enter(nx, ny){
                    //             let neighbor_idx = (ny * dijkstra.width + nx) as usize;
                    //             if let Some(nd) = dijkstra.distance[neighbor_idx] {
                    //                 if let Some(ed) = dijkstra.distance[enemy_idx] {
                    //                     if nd < ed {
                    //                         enemy.position.x = nx;
                    //                         enemy.position.y = ny;
                    //                         break;
                    //                     }
                    //                 }
                    //             }
                    //         }
                    //     }
                    
                    // }
                }
                PlayerAction::Wait => {},
                PlayerAction::Quit => ctx.quit(),
            }
            map::update_fov(self.world.player_position(), 3, &mut self.map);
        }
        // Render the map and player
        self.map.render(ctx);
        // Render all entities
        for (entity_id, position) in self.world.positions.iter() {
            // look up this entity's glyph
            if let Some(glyph) = self.world.glyphs.get(entity_id) {
                // decide colour based on whether it's player or enemy
                let colour = if self.world.players.contains(entity_id) {
                    YELLOW
                } else {
                    RED
                };
                ctx.set(position.x, position.y, colour, BLACK, to_cp437(*glyph));
            }
        }
        // self.player.render(ctx);
        // for enemy in self.enemies.iter(){
        //     enemy.render(ctx);
        // }
    }
}