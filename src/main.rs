fn main () {
    println!("Hello, world!");
}

pub struct State{
    player: Player,
    map: Map,
}

impl GameState for State{
    fn tick(&mut self, ctx:&BTerm){
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
            }

        }
    }
}