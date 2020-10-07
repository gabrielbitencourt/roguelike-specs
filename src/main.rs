use specs::{Builder, DispatcherBuilder, World, WorldExt};
use tcod::console::{FontLayout, FontType, Root, Offscreen};
use tcod::map::{Map as FovMap};

pub mod render;
use render::TcodSystem;

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

pub mod health;
use health::Health;

pub mod respawn;
use respawn::{Respawnable, RespawnSystem};

const SCREEN_WIDTH: i32 = 128;
const SCREEN_HEIGHT: i32 = 80;
const LIMIT_FPS: i32 = 30;

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Specs roguelike")
        .init();

    tcod::system::set_fps(LIMIT_FPS);

    let mut world = World::new();
    world.register::<Player>();
    world.register::<Input>();
    world.register::<Position>();
    world.register::<Glyph>();
    world.register::<Health>();
    world.register::<Respawnable>();

    world.insert(GameState::default());
    world.insert(Map::default());

    world
        .create_entity()
        .with(Player::default())
        .with(Input::default())
        .with(Glyph { c: '@', color: tcod::colors::RED })
        .with(Health::default())
        .with(Respawnable::default())
        .with(Position {
            x: SCREEN_WIDTH / 2,
            y: SCREEN_HEIGHT / 2,
        })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(MovingSystem, "movingSys", &[])
        .with(RespawnSystem, "respawnSys", &[])
        .with_thread_local(TcodSystem {
            root,
            con: Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            fov: FovMap::new(SCREEN_WIDTH, SCREEN_HEIGHT)
        })
        .build();

    dispatcher.setup(&mut world);
    loop {
        println!("game loop");
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
