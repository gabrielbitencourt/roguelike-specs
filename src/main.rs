use specs::{Builder, DispatcherBuilder, World, WorldExt};
use tcod::console::{FontLayout, FontType, Root, Offscreen};

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

pub mod room;
use room::{Room};

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
    
    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    tcod::system::set_fps(LIMIT_FPS);

    let mut world = World::new();
    world.register::<Room>();
    world.register::<Player>();
    world.register::<Input>();
    world.register::<Position>();
    world.register::<Glyph>();

    let mut gamestate = GameState::default();
    let room = Room::procedural(&mut gamestate.map);
    world
        .create_entity()
        .with(room)
        .build();
    
    world.insert(gamestate);

    world
        .create_entity()
        .with(Player::default())
        .with(Input::default())
        .with(Glyph { c: '@' })
        .with(Position {
            x: SCREEN_WIDTH / 2,
            y: SCREEN_HEIGHT / 2,
        })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(MovingSystem, "movingSys", &[])
        // .with(RoomSystem, "roomSys", &[])
        .with_thread_local(TcodSystem { root, con })
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
