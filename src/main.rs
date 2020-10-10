use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    input::{InputBundle, StringBindings},
    utils::application_root_dir,
    core::TransformBundle
};

pub mod state;
use state::GameState;

pub mod components;
use components::{
    player::Player
};

pub mod systems;
use systems::{
    moving::MovingSystem
};

const SCREEN_WIDTH: f32 = 200.0; // in pixels
const SCREEN_HEIGHT: f32 = 200.0; // in pixels

fn main() {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir().unwrap();
    let display_config_path = app_root.join("config").join("display.ron");
    let input_config_path = app_root.join("config").join("input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .unwrap()
                        .with_clear([0.0, 1.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )
        .unwrap()
        .with_bundle(TransformBundle::new())
        .unwrap()
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(input_config_path)
                .unwrap()
        )
        .unwrap()
        .with(MovingSystem, "moving_system", &["input_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, GameState::default(), game_data).unwrap();
    game.run();
}
