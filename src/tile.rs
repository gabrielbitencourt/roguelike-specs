#[derive(Debug,Default,Clone,Copy)]
pub struct Tile {
    pub glyph: Option<char>,
    pub wall: bool,
    pub occupied: bool,
    pub blocked: bool,
}

impl Tile {
    pub fn wall(side: bool) -> Self {
        return Tile {
            glyph: if side { Some('|') } else { Some('_') },
            wall: true,
            occupied: true,
            blocked: true,
        }
    }
    pub fn occupied(blocked: bool) -> Self {
        return Tile {
            glyph: None,
            wall: false,
            occupied: true,
            blocked,
        }
    }
}