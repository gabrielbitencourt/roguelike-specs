use crate::Position;
use crate::Health;
use specs::WriteStorage;
use specs::Component;
use specs::NullStorage;
use specs::ReadStorage;
use specs::System;
use specs::Join;

#[derive(Default)]
pub struct Respawnable;
impl Component for Respawnable {
    type Storage = NullStorage<Self>;
}

pub struct RespawnSystem;
impl<'a> System<'a> for RespawnSystem {
    
    type SystemData = (
        ReadStorage<'a, Respawnable>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Position>
    );
    fn run(&mut self, (respawnable, mut health, mut position): Self::SystemData) {
        for (_, hp, mut pos) in (&respawnable, &mut health, &mut position).join() {
            if hp.life <= 0 {
                hp.deaths = hp.deaths + 1;
                pos.x = 80;
                pos.y = 45;
            }
        }

        for (_, hp, _) in (!&respawnable, &mut health, &mut position).join() {
            if hp.life <= 0 {
                // despawn
            }
        }
    }

}