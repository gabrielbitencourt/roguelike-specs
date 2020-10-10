use crate::{Player,SCREEN_WIDTH,SCREEN_HEIGHT};
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

        let mut transform = Transform::default();
        transform.set_translation_xyz(SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.5, 1.0);

        world
            .create_entity()
            .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
            .with(transform)
            .build();


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
        
        let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
        let mut transform = Transform::default();
        transform.set_translation_xyz(SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.5, 0.0);
        world.create_entity()
            .with(sprite_render)
            .with(Player::default())
            .with(transform)
            .build();
    }
}