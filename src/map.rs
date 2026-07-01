/* THE MAP
Here we define the map, which is a grid of tiles. Each tile can be either a wall or a floor.
We also define the map generation algorithm, which is a simple random walk. 
We start at a random position and then randomly move in one of the four cardinal directions. 
We repeat this process until we have created a certain number of floor tiles. 
We also define a function to render the map to the console.
*/

use std::ops::AddAssign;

use rand::{Rng, random};
use bracket_lib::prelude::*;


#[derive(Clone, PartialEq)]
pub struct Map{
    pub tiles: Vec<TileType>,
    pub visibility: Vec<Visibility>,
    pub width: i32,
    pub height: i32,
    pub starting_position: Position,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position{
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Visibility {
    Unseen,
    Remembered,
    Visible,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rectangle{
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub enum BSPNode{
    Leaf(Rectangle),
    Node{
        left: Box<BSPNode>,
        right: Box<BSPNode>,
    }
}

pub fn bresenham_line(start: Position, end: Position) -> Vec<Position>{
        // calculate dx dy
        let dx = (end.x - start.x).abs();
        let dy = (end.y - start.y).abs();

        // decide whether we move positively or negatively based on direction
        let x_step = if end.x > start.x {1} else {-1};
        let y_step = if end.y > start.y {1} else {-1};

        //choose primary axis
        let mut x = start.x;
        let mut y = start.y;
        let mut error = 0;
        let mut position = Vec::new();

        if dx >= dy{
            while x != end.x{
                //x is primary_axis
                x += x_step;
                error += 2*dy;
                if error >= dx{
                    y += y_step;
                    error -= 2*dx;
                }
                position.push(Position{x, y});
            }
        }else{
            while y != end.y{
                //y is primary_axis
                y += y_step;
                error += 2*dx;
                if error >= dy{
                    x += x_step;
                    error -= 2*dy;
                }
                position.push(Position{x, y});
            }
        }
        position 
    }

pub fn update_fov(position: Position, radius: i32, map: &mut Map){
    // based on position look at radius and reset all tiles within radius to Remembered
    for vis in map.visibility.iter_mut() {
        match *vis {
            Visibility::Visible => *vis = Visibility::Remembered,
            _ => {}
        }
    }
    for y in (position.y - radius) ..= (position.y + radius){
        for x in (position.x - radius) ..= (position.x + radius){
            if !map.in_bounds(x, y){
                continue;
            }

            let dx = x - position.x;
            let dy = y - position.y;
            if (dx*dx + dy*dy) > radius*radius{
                continue;
            }

            let line = bresenham_line(position, Position{x, y});
            for pos in line.iter(){
                let idx = map.idx(pos.x, pos.y);
                map.visibility[idx] = Visibility::Visible;
                if map.tiles[idx] == TileType::Wall{
                    break;
                }

            }
        }
            
    }
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
            let mut tiles = vec![TileType::Wall; (width * height) as usize];
            let mut visibility = vec![Visibility::Unseen; (width * height) as usize];
            let root_region = Rectangle::new(0, 0, width, height);
            let bsp_tree = BSPNode::build(root_region,6,4);
            let starting_position = bsp_tree.find_first_room();
            let mut map = Map{tiles, visibility, width, height, starting_position};
            bsp_tree.traversal(&mut map);
            return map;
        }

    pub fn render(&self, ctx:&mut BTerm){
        ctx.cls();
        for i in 0..self.tiles.len(){
            let  x = i % self.width as usize;
            let  y = i / self.width as usize;
            let vis = self.visibility[i];
            let tile = self.tiles[i];

            match vis {
                Visibility::Unseen =>{
                    ctx.set(x as i32, y as i32, BLACK, BLACK, to_cp437(' '));
                }
                Visibility::Remembered =>{
                    match tile {
                        TileType::Wall => {
                            ctx.set(x as i32, y as i32, DARK_GRAY, BLACK, to_cp437('#'));
                        }
                        TileType::Floor => {
                            ctx.set(x as i32, y as i32, DARK_GREY, BLACK, to_cp437('.'));
                        }
                    }
                }
                Visibility::Visible =>{
                    match tile{
                        TileType::Wall =>{
                            ctx.set(x as i32, y as i32, WHITE, BLACK, to_cp437('#'));
                        }
                        TileType::Floor =>{
                            ctx.set(x as i32, y as i32, WHITE, BLACK, to_cp437('.'));
                        }
                    }
                }
                
            }
        }
    }
}

impl Rectangle{
    pub fn new(x:i32, y:i32, width:i32, height:i32) -> Self{
        Self { x, y, width, height }
    }
    pub fn carve_map(&self, map: &mut Map) {
        let width = map.width;
        for x in self.x + 1 .. self.x + self.width - 1 {
            for y in self.y + 1 .. self.y + self.height - 1 {
                let idx = (y * width + x) as usize;
                map.tiles[idx] = TileType::Floor;
            }
        }
    }
    pub fn centre_point(&self) -> Position{
        Position{
            x: self.x + self.width / 2,
            y: self.y + self.height / 2,
        }
    }
    pub fn is_too_small(&self, min_width: i32, min_height: i32) -> bool {
        self.width <= min_width * 2 || self.height <= min_height * 2
    }
}

impl BSPNode{
    pub fn build(region:Rectangle, min_width:i32, min_height:i32) -> Self{
        if region.is_too_small(min_width, min_height){
            return BSPNode::Leaf(region)
        }

        //randomly choose vertical or horizontal split
        let mut rng = rand::thread_rng();
        let split: bool = rng.gen_bool(0.5);

        let (left_region, right_region) = if split{
            //vertical split
            let cut = rng.gen_range(min_width..=region.width - min_width);
            (
                Rectangle::new(region.x, region.y, cut, region.height),
                Rectangle::new(region.x + cut, region.y, region.width - cut, region.height),
            )

        }else{
            //horizontal split
            let cut = rng.gen_range(min_height..=region.height-min_height);
            (
                Rectangle::new(region.x, region.y, region.width, cut),
                Rectangle::new(region.x, region.y + cut, region.width, region.height - cut),
            )
        };
        // build the nodes
        BSPNode::Node{
            left: Box::new(BSPNode::build(left_region, min_width, min_height)),
            right: Box::new(BSPNode::build(right_region, min_width, min_height)),   
        }
        
    }

    pub fn traversal(&self, map:&mut Map){
        match self{
            BSPNode::Leaf(region) => region.carve_map(map),
            BSPNode::Node{left, right} => {
                left.traversal(map);
                right.traversal(map);
            } 
        }
    }

    pub fn find_first_room(&self) -> Position{
        match self{
            BSPNode::Leaf(region) => region.centre_point(),
            BSPNode::Node{left, right} => {
                left.find_first_room()
            }
        }
    }
}




