use crate::{Input,Position,GameState,Tile};
use specs::{Join, ReadStorage, System, WriteStorage, Write};
use tcod::input::{Key, KeyCode};

pub struct MovingSystem;
impl<'a> System<'a> for MovingSystem {
    type SystemData = (
        ReadStorage<'a, Input>,
        WriteStorage<'a, Position>,
        Write<'a, GameState>
    );
    fn run(&mut self, (input, mut position, mut state): Self::SystemData) {
        for (inp, pos) in (&input, &mut position).join() {
            let tile = state.map[pos.y as usize][pos.x as usize];
            match inp.key {
                Some(Key {
                    code: KeyCode::Up,
                    ..
                }) =>
                {
                    if !state.map[(pos.y as usize) - 1][pos.x as usize].occupied {
                        state.map[pos.y as usize][pos.x as usize] = Tile::default();
                        pos.y = pos.y - 1;
                        state.map[pos.y as usize][pos.x as usize] = Tile::occupied(true);
                    }
                }
                Some(Key {
                    code: KeyCode::Down,
                    ..
                }) =>
                {
                    if !state.map[(pos.y as usize) + 1][pos.x as usize].occupied {
                        state.map[pos.y as usize][pos.x as usize] = Tile::default();
                        pos.y = pos.y + 1;
                        state.map[pos.y as usize][pos.x as usize] = Tile::occupied(true);
                    }
                }
                Some(Key {
                    code: KeyCode::Left,
                    ..
                }) =>
                {
                    if !state.map[pos.y as usize][(pos.x as usize) - 1].occupied {
                        state.map[pos.y as usize][pos.x as usize] = Tile::default();
                        pos.x = pos.x - 1;
                        state.map[pos.y as usize][pos.x as usize] = Tile::occupied(true);
                    }
                }
                Some(Key {
                    code: KeyCode::Right,
                    ..
                }) =>
                {
                    if !state.map[pos.y as usize][(pos.x as usize) + 1].occupied {
                        state.map[pos.y as usize][pos.x as usize] = Tile::default();
                        pos.x = pos.x + 1;
                        state.map[pos.y as usize][pos.x as usize] = Tile::occupied(true);
                    }
                }
                _ => {}
            };
        }
    }
}