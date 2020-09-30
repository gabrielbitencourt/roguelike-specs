use specs::Component;
use specs::NullStorage;

#[derive(Default)]
pub struct Player;
impl Component for Player {
    type Storage = NullStorage<Self>;
}
