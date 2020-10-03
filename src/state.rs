use crate::Tile;
use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;

pub struct GameState {
    pub end: bool,
    pub map: [[Tile; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize]
}

impl Default for GameState {
    
    fn default() -> Self {
        return GameState {
            end: false,
            map: GameState::generate_map()
        };
    }
}

impl GameState {
    fn generate_map() -> [[Tile; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize] {
        return [[Tile::default(); SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];
    }
}