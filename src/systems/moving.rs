use crate::{Player,SCREEN_HEIGHT,SCREEN_WIDTH};
use amethyst::core::{Transform,SystemDesc,timing::Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join,Read,ReadStorage,System,SystemData,World,WriteStorage};
use amethyst::input::{InputHandler,StringBindings};

pub struct MovingSystem;
impl<'a> System<'a> for MovingSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Player>,
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, Time>,
    );
    fn run(&mut self, (mut transforms, players, input, time): Self::SystemData) {
        for (transform, player) in (&mut transforms, &players).join() {
            if let Some(dx) = input.axis_value("x") {
                if dx != 0. {
                    let x = transform.translation().x;
                    transform.set_translation_x(
                        (x + dx * time.delta_seconds() * 30.)
                            .min(SCREEN_WIDTH)
                            .max(0.),
                    );
                }
            }
            if let Some(dy) = input.axis_value("y") {
                if dy != 0. {
                    let y = transform.translation().y;
                    transform.set_translation_y(
                        (y + dy * time.delta_seconds() * 30.)
                            .min(SCREEN_HEIGHT)
                            .max(0.),
                    );
                }
            }
        }
    }
}
