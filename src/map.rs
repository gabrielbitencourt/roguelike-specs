use crate::Tile;
use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use rand::Rng;

const MAX_ROOMS: usize = 6;
const MIN_ROOM_WIDTH: usize = 7;
const MAX_ROOM_WIDTH: usize = 20;
const MIN_ROOM_HEIGHT: usize = 7;
const MAX_ROOM_HEIGHT: usize = 20;

pub struct Map {
    pub map: [[Tile; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize]
}

impl Default for Map {
    fn default() -> Self {
        return Map {
            map: Map::generate_map()
        };
    }
}

impl Map {
    pub fn generate_map() -> [[Tile; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize] {
        let mut map = [[Tile::default(); SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

        let n_rooms = rand::thread_rng().gen_range(1, MAX_ROOMS + 1);
        for _ in 0..n_rooms {
            let room = Room::procedural();
            map[room.y + room.height - 2][room.x + room.width - 1] = Tile::wall();
            for y in room.y..(room.y + room.height - 2) {
                map[y][room.x] = Tile::wall();
                map[y][room.x + room.width - 1] = Tile::wall();
            }
            for x in room.x..(room.x + room.width) {
                map[room.y][x] = Tile::wall();
                map[room.y + room.height - 1][x] = Tile::wall();
            }
        }
        return map;
    }
}

pub struct Room {
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

impl Room {
    fn procedural() -> Self {
        let width = rand::thread_rng().gen_range(MIN_ROOM_WIDTH, MAX_ROOM_WIDTH + 1);
        let height = rand::thread_rng().gen_range(MIN_ROOM_HEIGHT, MAX_ROOM_HEIGHT + 1);
        let x = rand::thread_rng().gen_range(0, SCREEN_WIDTH as usize - width);
        let y = rand::thread_rng().gen_range(0, SCREEN_HEIGHT as usize - height);
        return Room {
            x, y, width, height
        };
    }
}