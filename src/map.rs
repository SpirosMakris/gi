use crate::bracket_random::prelude::*;
use crate::mq;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn xy_world(x: i32, y: i32) -> (f32, f32) {
    (x as f32 * 16.0, y as f32 * 16.0)
}

pub fn world_xy(x: f32, y: f32) -> (i32, i32) {
    (x.div_euclid(16.0) as i32, y.div_euclid(16.0) as i32)
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];

    // Make the boundaries walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Now we'll randomly splat a bunch of walls 
    let mut rng = RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);

        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn render_map(map: &[TileType]) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render a tile depending on it's type
        match tile {
            TileType::Floor => {
                mq::draw_rectangle(x as f32 * 16.0, y as f32 * 16.0, 16.0, 16.0, mq::GREEN);
            }
            TileType::Wall => {
                mq::draw_rectangle(x as f32 * 16.0, y as f32 * 16.0, 16.0, 16.0, mq::BEIGE);
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}