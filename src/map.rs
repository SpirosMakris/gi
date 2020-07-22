use crate::bracket_random::prelude::*;
use crate::mq;
use crate::TRect;

use std::cmp::{min, max};

#[derive(PartialEq, Copy, Clone, Debug)]
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

/// Makes a map with solid boundaries and 400 randomly placed walls.
/// No guarantees that it won't look awful.
pub fn new_map_test() -> Vec<TileType> {
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

fn apply_room_to_map(room: &TRect, map: &mut [TileType]) {
    println!("Appying room:{:?} to map", room);
    for y in room.y1 + 1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;            
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2) ..= max(x1,x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2) ..= max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx] = TileType::Floor;
        }
    }
}

/// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
/// This gives a handful of random rooms and corridors joining them together
pub fn new_map_rooms_and_corridors() -> (Vec<TRect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; 80*50];

    let mut rooms: Vec<TRect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();
   

    for _i in 0..MAX_ROOMS {
        // Create a new room
        println!("Creating room: {}", _i);
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 80 - w - 1) - 1;
        let y = rng.roll_dice(1,  50 - h - 1) - 1;

        let new_room = TRect::new(x, y, w, h);
        let mut ok = true;

        // Check if it overlaps with previous rooms
        for other_room in rooms.iter() {
            if new_room.intersects(other_room) {
                ok = false;
            }
        }

        // If we are good to go, apply this room
        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }

    }

    println!("Applied {} of {} rooms", rooms.len(), MAX_ROOMS);

    (rooms, map)
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