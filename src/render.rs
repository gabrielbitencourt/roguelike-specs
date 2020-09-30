use crate::{Input, Position,GameState,Player,Glyph,SCREEN_HEIGHT,SCREEN_WIDTH};
use specs::{ReadStorage, World, WriteStorage,Join,System,Write};
use tcod::colors::WHITE;
use tcod::console::{BackgroundFlag, Console, Root, Offscreen,blit};
use tcod::input::{Key,KeyCode};

pub struct TcodSystem {
    pub root: Root,
    pub con: Offscreen
}

impl<'a> System<'a> for TcodSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Glyph>,
        WriteStorage<'a, Input>,
        Write<'a, GameState>
    );
    fn setup(&mut self, _: &mut World) {
        self.con.set_default_foreground(WHITE);
    }
    fn run(&mut self, (position, player, glyph, mut input, mut state): Self::SystemData) {
        self.con.clear();
        for (pos, gl) in (&position, &glyph).join() {
            self.con.put_char(pos.x, pos.y, gl.c, BackgroundFlag::None);
        }
        for (r, row) in state.map.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if tile.wall {
                    let wall_glyph = match tile.glyph {
                        Some(t) => t,
                        _ => '_'
                    };
                    self.con.put_char(c as i32, r as i32, wall_glyph, BackgroundFlag::None)
                }
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
        let key = self.root.wait_for_keypress(true);
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
