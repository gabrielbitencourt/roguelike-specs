use tcod::Color;
use specs::Component;
use specs::VecStorage;

pub struct Glyph {
    pub c: char,
    pub color: Color
}

impl Component for Glyph {
    type Storage = VecStorage<Self>;
}