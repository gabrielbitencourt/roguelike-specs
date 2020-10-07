use specs::{Builder, DispatcherBuilder, World, WorldExt};
use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

pub mod render;
use render::piston::PistonSystem;

pub mod position;
use position::Position;

pub mod input;
use input::Input;

pub mod player;
use player::Player;

pub mod moving;
use moving::MovingSystem;

pub mod state;
use state::GameState;

pub mod glyph;
use glyph::Glyph;

pub mod tile;
use tile::Tile;

pub mod map;
use map::Map;

const SCREEN_WIDTH: i32 = 100;
const SCREEN_HEIGHT: i32 = 56;
const TILE_SIZE: i32 = 10;

fn main() {
    let opengl = OpenGL::V3_2;
    let window: GlutinWindow = WindowSettings::new("Specssss game", [(SCREEN_WIDTH * TILE_SIZE) as u32, (SCREEN_HEIGHT * TILE_SIZE) as u32])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .expect("Failed to init window.");

    let mut world = World::new();
    world.register::<Player>();
    world.register::<Input>();
    world.register::<Position>();
    world.register::<Glyph>();

    world.insert(GameState::default());
    world.insert(Map::default());

    world
        .create_entity()
        .with(Player::default())
        .with(Input::default())
        .with(Glyph { c: '@', color: tcod::colors::RED })
        .with(Position {
            x: SCREEN_WIDTH / 2,
            y: SCREEN_HEIGHT / 2,
        })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(MovingSystem, "movingSys", &[])
        .with_thread_local(PistonSystem {
            gl: GlGraphics::new(opengl),
            window,
        })
        .build();

    dispatcher.setup(&mut world);
    loop {
        dispatcher.dispatch(&world);
        {
            let game_state = world.read_resource::<GameState>();
            if game_state.end {
                break;
            }
        }
		world.maintain();
	}
}
