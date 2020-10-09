use crate::{Player,Position,Input,Map};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

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

        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "hero.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                "hero.ron",
                SpriteSheetFormat(texture_handle),
                (),
                &sprite_sheet_store,
            )
        };
        
        let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);  // paddle is the first sprite in the sprite_sheet

        world.create_entity()
            .with(sprite_render)
            .with(Player::default())
            .with(Input::default())
            .with(transform)
            .build();
    }
}