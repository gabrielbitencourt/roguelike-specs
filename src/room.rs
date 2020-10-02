use crate::{GameState,Tile};

pub struct RoomSystem;
impl RoomSystem {
    fn setup(gamestate: &mut GameState) {
        println!("generating rooms");
        gamestate.map[0][0] = Tile::wall(false);
    }
}