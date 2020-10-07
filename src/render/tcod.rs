use crate::{Input, Position,GameState,Map,Player,Glyph,SCREEN_HEIGHT,SCREEN_WIDTH};
use specs::{ReadStorage,World,WriteStorage,Join,System,Write};
use tcod::{
    console::{
        BackgroundFlag,Console,Root,Offscreen,blit
    },
    input::{
        Key,KeyCode
    },
    map::{
        Map as FovMap,FovAlgorithm
    },
    colors,
    colors::Color
};

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const LIGHT_WALLS: bool = true;
const PLAYER_RADIUS: i32 = 10;

const COLOR_INVISIBLE_WALL: Color = colors::BLACK;
const COLOR_VISIBLE_WALL: Color = colors::WHITE;
const COLOR_INVISIBLE_GROUND: Color = colors::BLACK;
const COLOR_VISIBLE_GROUND: Color = colors::YELLOW;

pub struct TcodSystem {
    pub root: Root,
    pub con: Offscreen,
    pub fov: FovMap
}

impl<'a> System<'a> for TcodSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Glyph>,
        WriteStorage<'a, Input>,
        Write<'a, Map>,
        Write<'a, GameState>
    );
    fn setup(&mut self, _: &mut World) {
        self.con.set_default_foreground(colors::WHITE);
    }
    fn run(&mut self, (position, player, glyph, mut input, mut map_resource, mut state): Self::SystemData) {
        self.con.clear();

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.fov.set(
                    x,
                    y,
                    !map_resource.map[y as usize][x as usize].occupied,
                    !map_resource.map[y as usize][x as usize].blocked,
                );
            }
        }

        // glyph-position-player
        for (pos, gl, _) in (&position, &glyph, &player).join() {
            self.con.put_char_ex(pos.x, pos.y, gl.c, gl.color, colors::BLACK);
            self.fov.compute_fov(pos.x, pos.y, PLAYER_RADIUS, LIGHT_WALLS, FOV_ALGO);
        }
        for (pos, gl, _) in (&position, &glyph, !&player).join() {
            if self.fov.is_in_fov(pos.x, pos.y) {
                self.con.put_char_ex(pos.x, pos.y, gl.c, gl.color, colors::BLACK);
            }
        }

        // walls
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let tile = &mut map_resource.map[y as usize][x as usize];
                let visible = self.fov.is_in_fov(x as i32, y as i32);
                let color = match (tile.explored, visible, tile.wall) {
                    (true, _, false) => COLOR_VISIBLE_GROUND,
                    (true, _, true) => COLOR_VISIBLE_WALL,
                    (false, false, false) => COLOR_INVISIBLE_GROUND,
                    (false, true, false) => {
                        tile.explored = true;
                        COLOR_VISIBLE_GROUND
                    },
                    (false, false, true) => COLOR_INVISIBLE_WALL,
                    (false, true, true) => {
                        tile.explored = true;
                        COLOR_VISIBLE_WALL
                    },
                };
                self.con.set_char_background(x as i32, y as i32, color, BackgroundFlag::Set);
            }
        }
        blit(&self.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut self.root,
            (0, 0),
            1.0,
            1.0);
        self.root.flush();

        if self.root.window_closed() {
            state.end = true;
        }
        let key = self.root.wait_for_keypress(false);
        match key {
            Key { code: KeyCode::Escape, .. } => {
                if self.root.is_fullscreen() {
                    self.root.set_fullscreen(false)
                }
                else {
                    state.end = true;
                }
            },
            Key { code: KeyCode::Enter, alt: true, .. } => {
                self.root.set_fullscreen(!self.root.is_fullscreen());
            },
            _ => {}
        }
        for (inp, _) in (&mut input, &player).join() {
            inp.key = Some(key);
        }
    }
}