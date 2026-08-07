use bracket_lib::{prelude::*};
mod map;
mod player;
mod enemy;
mod world;

use map::{Map, Position, DijkstraMap};
use player::{PlayerAction, handle_input};
use world::World;
use attack;

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
                        if let Some(entity_id) = self.world.occupied_tile(new_x, new_y) {
                            // attack the entity
                            if let Some(health) = self.world.health.get_mut(&entity_id) {
                                health.current -= 1;
                                attack::perform_attack(&mut self.world, &self.map, self.world.player_id(), entity_id, 1);
                                if health.current <= 0 {
                                    // entity dies — remove all its components
                                    self.world.positions.remove(&entity_id);
                                    self.world.glyphs.remove(&entity_id);
                                    self.world.health.remove(&entity_id);
                                    self.world.enemies.remove(&entity_id);
                                }
                            }
                        }else{
                            // empty tile — move player
                            self.world.positions.insert(
                                self.world.player_id(),
                                Position { x: new_x, y: new_y }
                            );
                        }
                    }
                    let dijkstra = DijkstraMap::new(&self.map, self.world.player_position());

                    for enemy_id in self.world.enemies.iter(){
                        let enemy_position = *self.world.positions.get(enemy_id).unwrap();
                        let enemy_idx = (enemy_position.y * dijkstra.width + enemy_position.x) as usize;
                        for (ex, ey) in [(-1, 0), (1,0), (0,-1), (0,1)]{
                            let nx = enemy_position.x + ex;
                            let ny = enemy_position.y + ey;
                            if self.map.can_enter(nx, ny) && self.world.occupied_tile(nx, ny).is_none(){
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
                }
                PlayerAction::Wait => {},
                PlayerAction::Quit => ctx.quit(),
            }
            map::update_fov(self.world.player_position(), 3, &mut self.map);
        }
        // Render the map and player
        self.map.render(ctx);
        for (id, position) in self.world.positions.iter(){
            if let Some(glyph) = self.world.glyphs.get(id){
                let colour = if self.world.players.contains(id){
                    YELLOW
                } else{
                    RED
                };
                ctx.set(position.x, position.y, colour, BLACK, to_cp437(*glyph));
            }

        }
    }
}