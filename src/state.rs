pub struct GameState {
    pub end: bool,
}

impl Default for GameState {
    
    fn default() -> Self {
        return GameState {
            end: false,
        };
    }
}