use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::TransformBundle
};

pub mod state;
use state::GameState;

pub mod components;
use components::{
    position::Position,
    input::Input,
    player::Player
};

pub mod systems;
use systems::{
    moving::MovingSystem
};

pub mod resources;
use resources::{
    map::Map,
    tile::Tile
};

const SCREEN_WIDTH: i32 = 500;
const SCREEN_HEIGHT: i32 = 500;
const TILE_SIZE: i32 = 10;

fn main() {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir().unwrap();
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .unwrap()
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )
        .unwrap()
        .with_bundle(TransformBundle::new())
        .unwrap();

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, GameState::default(), game_data).unwrap();
    game.run();

    // let mut world = World::new();
    // world.register::<Player>();
    // world.register::<Input>();
    // world.register::<Position>();

    // world.insert(GameState::default());
    // world.insert(Map::default());

    // world
    //     .create_entity()
    //     .with(Player::default())
    //     .with(Input::default())
    //     .with(Position {
    //         x: SCREEN_WIDTH / 2,
    //         y: SCREEN_HEIGHT / 2,
    //     })
    //     .build();

    // let mut dispatcher = DispatcherBuilder::new()
    //     .with(MovingSystem, "movingSys", &[])
    //     .build();

    // dispatcher.setup(&mut world);
    // loop {
    //     dispatcher.dispatch(&world);
    //     {
    //         let game_state = world.read_resource::<GameState>();
    //         if game_state.end {
    //             break;
    //         }
    //     }
	// 	world.maintain();
	// }
}
