/*
The player should be a # that represent a movable character. 
The player should be abel to move up and down and left and right using the arrow keys.
The player should not be able to move through walls. 
 */

use bracket_lib::prelude::*;


pub struct Player{
    pub position:Position,
    pub glyph: char,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position{
    pub x: i32,
    pub y: i32,
}

pub enum PlayerAction{
    Move(i32, i32),
    Wait,
    Quit,
}

impl Player{
    pub fn new(x:i32, y:i32)-> Self{
        Player{
            position:Position { x, y },
            glyph: '@',
        }
    }

    pub fn render(&self, ctx: &BTerm){
        ctx.set(self.position.x, self.position.y, RED, BLACK, to_cp437(self.glyph))
    }
}

pub fn handle_input(ctx:&BTerm) -> Option<PlayerAction>{
    match ctx.key{
        None => None,
        Some(key) => match key{
            VirtualKeyCode::Left => Some(PlayerAction::Move(-1, 0)),
            VirtualKeyCode::Right => Some(PlayerAction::Move(1, 0)),
            VirtualKeyCode::Up => Some(PlayerAction::Move(0, -1)),
            VirtualKeyCode::Down => Some(PlayerAction::Move(0, 1)),
            VirtualKeyCode::Escape => Some(PlayerAction::Quit),
            _ => None,
        }
        
    }
}