use crate::{Player,Position,Input,Map};
use amethyst::{GameData,SimpleState,StateData};
use amethyst::ecs::{WorldExt,Builder};
use amethyst::core::transform::Transform;

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Player>();
        world.register::<Input>();
        world.register::<Position>();

        world.insert(Map::default());
        let mut transform = Transform::default();
        transform.set_translation_xyz(25., 25., 0.);

        world.create_entity()
            .with(Player::default())
            .with(Input::default())
            .with(transform)
            .build();
    }
}