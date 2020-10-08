use amethyst::ecs::{Component,VecStorage};

#[derive(Default)]
pub struct Input {
    pub key: Option<usize>,
}

impl Component for Input {
    type Storage = VecStorage<Self>;
}