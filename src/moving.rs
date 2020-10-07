use crate::{Input,Position,Map,SCREEN_HEIGHT,SCREEN_WIDTH};
use specs::{Join, ReadStorage, System, WriteStorage, Write};
use piston::ButtonArgs;
use piston::Button::{Mouse,Keyboard};
use piston::input::{MouseButton,Key};

pub struct MovingSystem;
impl<'a> System<'a> for MovingSystem {
    type SystemData = (
        ReadStorage<'a, Input>,
        WriteStorage<'a, Position>,
        Write<'a, Map>
    );
    fn run(&mut self, (input, mut position, mut map_resource): Self::SystemData) {
        for (inp, pos) in (&input, &mut position).join() {
            match inp.key {
                Some(ButtonArgs {
                    button: Keyboard(Key::Up),
                    ..
                }) =>
                {
                    if !map_resource.map[(pos.y as usize) - 1][pos.x as usize].occupied && pos.y > 1 {
                        map_resource.map[pos.y as usize][pos.x as usize].occupied = false;
                        pos.y = pos.y - 1;
                        map_resource.map[pos.y as usize][pos.x as usize].occupied = true;
                    }
                }
                Some(ButtonArgs {
                    button: Keyboard(Key::Down),
                    ..
                }) =>
                {
                    if pos.y < SCREEN_HEIGHT - 2 && !map_resource.map[(pos.y as usize) + 1][pos.x as usize].occupied {
                        map_resource.map[pos.y as usize][pos.x as usize].occupied = false;
                        pos.y = pos.y + 1;
                        map_resource.map[pos.y as usize][pos.x as usize].occupied = true;
                    }
                }
                Some(ButtonArgs {
                    button: Keyboard(Key::Left),
                    ..
                }) =>
                {
                    if !map_resource.map[pos.y as usize][(pos.x as usize) - 1].occupied && pos.x > 1 {
                        map_resource.map[pos.y as usize][pos.x as usize].occupied = false;
                        pos.x = pos.x - 1;
                        map_resource.map[pos.y as usize][pos.x as usize].occupied = true;
                    }
                }
                Some(ButtonArgs {
                    button: Keyboard(Key::Right),
                    ..
                }) =>
                {
                    if pos.x < SCREEN_WIDTH - 2 && !map_resource.map[pos.y as usize][(pos.x as usize) + 1].occupied  {
                        map_resource.map[pos.y as usize][pos.x as usize].occupied = false;
                        pos.x = pos.x + 1;
                        map_resource.map[pos.y as usize][pos.x as usize].occupied = true;
                    }
                }
                _ => {}
            };
        }
    }
}
