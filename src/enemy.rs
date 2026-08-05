/*
The enemy should be a @ that represent a movable character. 
The enemy should be abel to move up and down and left and right using the arrow keys.
The enemy should not be able to move through walls. 
 */

use bracket_lib::prelude::*;
use crate::map::{Position, Map};


pub struct Enemy{
    pub entity: u32,
    pub position:Position,
    pub glyph: char,
    pub radius: i32,
    pub health: i32,
}


pub enum EnemyAction{
    Move(i32, i32),
    Wait,
    Quit,
}

impl Enemy{
    pub fn new(position: Position)-> Self{
        Enemy{
            entity: 1,
            position,
            glyph: 'E',
            radius: 8,
            health: 50,
        }
    }

    pub fn render(&self, ctx: &mut BTerm){
        ctx.set(self.position.x, self.position.y, RED, BLACK, to_cp437(self.glyph))
    }

    pub fn update_position(&mut self, dx: i32, dy: i32, map: &Map){
        let new_x = self.position.x + dx;
        let new_y = self.position.y + dy;

        if map.can_enter(new_x, new_y){
            self.position.x = new_x;
            self.position.y = new_y;
        }
    }
}
