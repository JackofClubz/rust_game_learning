/* THE MAP
Here we define the map, which is a grid of tiles. Each tile can be either a wall or a floor.
We also define the map generation algorithm, which is a simple random walk. 
We start at a random position and then randomly move in one of the four cardinal directions. 
We repeat this process until we have created a certain number of floor tiles. 
We also define a function to render the map to the console.
*/

use rand::Rng;
use bracket_lib::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Map{
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rectangle{
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

//check in_bounds first → then call idx → then index the Vec
impl Map{
    // calculate the index of a tile in the tiles vector based on its x and y coordinates
    pub fn idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }
    //check if the given coordinates are within the bounds of the map
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
    // verify whether we can enter a tile at the given coordinates (i.e., if it's a floor tile)
    pub fn can_enter(&self, x: i32, y: i32) -> bool {
        self.in_bounds(x, y) && self.tiles[self.idx(x, y)] == TileType::Floor
    }

    // generate a new map with the given width and height
    pub fn new(width: i32, height: i32) -> Self {
            let mut tiles = vec![TileType::Floor; (width * height) as usize];
            
            for x in 0..width {
                tiles[(0 * width + x) as usize]            = TileType::Wall;
                tiles[((height-1) * width + x) as usize]   = TileType::Wall;
            }
            for y in 0..height {
                tiles[(y * width + 0) as usize]             = TileType::Wall;
                tiles[(y * width + (width-1)) as usize]     = TileType::Wall;
            }
            
            Self { tiles, width, height }
        }


    pub fn render(&self, ctx:&mut BTerm){
        ctx.cls();
        for(i, tile) in self.tiles.iter().enumerate(){
            let  x = i % self.width as usize;
            let  y = i / self.width as usize;

            match tile {
                &TileType::Wall =>{
                    ctx.set(x as i32,y as i32, WHITE, BLACK, to_cp437('#'))
                }
                &TileType::Floor =>{
                    ctx.set(x as i32,y as i32, GREY, BLACK, to_cp437('.'))
                }
                
            }
        }
    }
}




