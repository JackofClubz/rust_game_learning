use bracket_lib::prelude::*;
mod map;
mod player;

use map::{Map, Position};
use player::{Player, PlayerAction, handle_input};


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
}

impl State{
    pub fn new() -> Self{
        State{
            player:Player::new(5,5),
            map: Map::new(40, 40),
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
                }
                PlayerAction::Wait => {},
                PlayerAction::Quit => ctx.quit(),
            }
        }
        // Render the map and player
        self.map.render(ctx);
        self.player.render(ctx);
    }
}