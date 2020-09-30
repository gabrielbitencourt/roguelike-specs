use specs::Component;
use specs::VecStorage;

pub struct Glyph {
    pub c: char
}

impl Component for Glyph {
    type Storage = VecStorage<Self>;
}