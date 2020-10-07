use specs::{Component, VecStorage};
use piston::ButtonArgs;

#[derive(Default)]
pub struct Input {
    pub key: Option<ButtonArgs>,
}

impl Component for Input {
    type Storage = VecStorage<Self>;
}