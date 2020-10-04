use specs::{Component, VecStorage};
use tcod::input::Key;

#[derive(Default)]
pub struct Input {
    pub key: Option<Key>,
}

impl Component for Input {
    type Storage = VecStorage<Self>;
}