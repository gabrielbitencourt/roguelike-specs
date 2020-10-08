#[derive(Debug,Default,Clone,Copy)]
pub struct Tile {
    pub wall: bool,
    pub occupied: bool,
    pub blocked: bool,
    pub explored: bool
}

impl Tile {
    pub fn wall() -> Self {
        return Tile {
            wall: true,
            occupied: true,
            blocked: true,
            explored: false,
        };
    }
}