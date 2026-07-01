/*
The enemy should be a @ that represent a movable character. 
The enemy should be abel to move up and down and left and right using the arrow keys.
The enemy should not be able to move through walls. 
 */

use bracket_lib::prelude::*;
use crate::map::{Position, Map};


pub struct Enemy{
    pub position:Position,
    pub glyph: char,
    pub radius: i32,
}
