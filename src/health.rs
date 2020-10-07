use specs::VecStorage;
use specs::Component;

pub struct Health {
    pub deaths: usize,
    pub life: usize
}

impl Component for Health {
    type Storage = VecStorage<Self>;
}

impl Default for Health {
    fn default() -> Self {
        return Health {
            deaths: 0,
            life: 100
        };
    }
}