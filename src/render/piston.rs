use crate::{GameState,Map,Input,Glyph,Player,Position,TILE_SIZE};
use specs::{Write,WriteStorage,ReadStorage,System,World,Join};

use glutin_window::*;
use opengl_graphics::*;
use piston::*;

type Colour = [f32; 4];

const RED: Colour = [1.0, 0.0, 0.0, 1.0];
const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];

pub struct PistonSystem {
    pub gl: GlGraphics,
    pub window: GlutinWindow
}
impl<'a> System<'a> for PistonSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Glyph>,
        WriteStorage<'a, Input>,
        Write<'a, Map>,
        Write<'a, GameState>
    );
    fn setup(&mut self, _: &mut World) {
        
    }
    fn run(&mut self, (position, player, glyph, mut input, mut map_resource, mut state): Self::SystemData) {
        let mut events = Events::new(EventSettings::new());
        if let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.gl.draw(args.viewport(), |canva, gl| {
                    graphics::clear(WHITE, gl);
                });
                // println!("render: {:?}", args);
                // let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
                // for (pos, _) in (&position, &glyph).join() {
                //     let player = rectangle::square(pos.x as f64, pos.y as f64, TILE_SIZE as f64);
                //     self.gl.draw(args.viewport(), |canva, gl| {
                //         clear(BLACK, gl);
                //         let transform = canva.transform.trans(x, y);
                //         rectangle(WHITE, player, transform, gl);
                //     });
                // }
            }
            if let Some(args) = e.update_args() {
                println!("update: {:?}", args);
            }

            if let Some(input_args) = e.button_args() {
                if input_args.state == ButtonState::Press {
                    // println!("button: {:?}", input_args);
                    for (inp, _) in (&mut input, &player).join() {
                        inp.key = Some(input_args);
                    }
                }
            }
        }
        else {
            state.end = true;
        }
    }
}