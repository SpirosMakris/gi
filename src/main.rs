use macroquad as mq;
use shipyard::*;

use bracket_random;

mod map;
pub use map::*;
mod trect;
pub use trect::TRect;


struct GameState {
    ecs: World,
}

impl GameState {
    fn render(&self) {
        // @TODO: Draw map
        let map = self.ecs.borrow::<UniqueView<Vec<TileType>>>();
        render_map(&*map);
        
        // Draw renderables system
        self.ecs.run(|positions: View<Position>, renders: View<Renderable>| {
            for (pos, rend ) in (&positions, &renders).iter() {
                 
                mq::draw_circle(pos.x, pos.y, 12.0, rend.color);
            }
        });
    }

    fn run_systems(&self) {
        self.ecs.run(left_mover_sys);
    }
}

// Utility structs for Unique Resources
struct ScreenDims {
    w: f32,
    h: f32,
}

// struct PlayerMovement {
//     delta_x: f32,
//     delta_y: f32,
// }


// Components
#[derive(Debug)]
struct Position {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Renderable {
    color: mq::Color,
}

// Tag component
struct PlayerTag {}

struct LeftMoverTag {}


#[macroquad::main("GI")]
async fn main() {
    let scr_dims =  ScreenDims {
        w: mq::screen_width(),
        h: mq::screen_height(),
    };

    // Create global state/world
    let gs = GameState {
        ecs: World::new(),
    };

    // Add screen width as unique
    gs.ecs.add_unique(scr_dims);

    // Add map as unique
    gs.ecs.add_unique(new_map_rooms_and_corridors());

    // Add a player entity
    gs.ecs.run(add_player);

    // Add LeftMover type entities
    gs.ecs.run(add_left_movers);

    
    // frame counter
    let mut frame_counter: u32 = 0;

    loop {
        // Print all positional entities
        // if frame_counter % 120 == 0 {
        //     gs.ecs.run(|positions: View<Position>| {
        //         for (id, i) in positions.iter().with_id() {
        //             println!("Entity # {:?} has id {:?}", i, id);
        //         }
        //     });
        // }

        // UPDATE
        // INPUT
        player_input(&gs.ecs);

        // Update ECS
        gs.run_systems();


        // RENDERING
        // Clear the BG
        mq::clear_background(mq::GRAY);


        gs.render();

        // Set up fps counter
        let fps = format!("FPS: {} / FrameTime: {} ms", mq::get_fps(), mq::get_frame_time() * 1000.0);
        mq::draw_text(&fps, 20.0, 2.0, 20.0, mq::DARKGRAY);

        frame_counter += 1;

        mq::next_frame().await
    }
}

// Systems
fn left_mover_sys(scr_dims: UniqueView<ScreenDims>, mut positions: ViewMut<Position>, left_movers: View<LeftMoverTag>) {
    for (pos, _) in (&mut positions, &left_movers).iter() {
        pos.x += 2.0;
        if pos.x > scr_dims.w {
            pos.x = 0.0;
        }
    }
}

/// Try to update player position with incoming delta values
fn try_move_player(delta_x: f32, delta_y: f32, ecs: &World) {
    let (map, scr_dims, mut positions, players) = 
        ecs.borrow::<(UniqueView<Vec<TileType>>, UniqueView<ScreenDims>, ViewMut<Position>, View<PlayerTag>)>();
        

    for (pos, _player) in (&mut positions, &players).iter() {
        // Very naive 'collision detection'
        // Convert world to tile coords
        let (tx, ty) =  world_xy(pos.x + delta_x, pos.y + delta_y);
        let dest_idx = xy_idx(tx, ty);

        // println!("(x,y)= ({},{}) => ({},{})", pos.x, pos.y, tx, ty);
        println!("wx = {} =>  tx = {}", pos.x, tx);

        if map[dest_idx] != TileType::Wall {
            pos.x = scr_dims.w.min(0.0f32.max(pos.x + delta_x));
            pos.y = scr_dims.h.min(0.0f32.max(pos.y + delta_y));
        } else {
            println!("Hit wall at (tx, ty) = {}/{}", tx, ty);
            // panic!();
        }
        
    }
}

fn player_input(ecs: &World) {
    if mq::is_key_down(mq::KeyCode::Left) {
        try_move_player(-1.0, 0.0, ecs);
    } 
    if mq::is_key_down(mq::KeyCode::Right) {
        try_move_player(1.0, 0.0, ecs);
    }
    if mq::is_key_down(mq::KeyCode::Up) {
        try_move_player(0.0, -1.0, ecs);
    }
    if mq::is_key_down(mq::KeyCode::Down) {
        try_move_player(0.0, 1.0, ecs);
    }
}

// Utils
fn add_left_movers(mut entities: EntitiesViewMut, mut positions: ViewMut<Position>, mut renders: ViewMut<Renderable>, mut left_movers: ViewMut<LeftMoverTag>) {
    
    for i in 0..10 {
        entities.add_entity(
            (&mut positions, &mut renders, &mut left_movers),
            (
                Position { x: i as f32 * 40.0, y: 20.0 },
                Renderable { color: mq::RED },
                LeftMoverTag {}
            ));
    }
}

fn add_player(scr_dims: UniqueView<ScreenDims>, mut entities: EntitiesViewMut, mut positions: ViewMut<Position>, mut renders: ViewMut<Renderable>, mut players: ViewMut<PlayerTag>) {
        entities.add_entity(
            (&mut positions, &mut renders, &mut players),
            (Position {x: scr_dims.w / 2.0, y: scr_dims.h / 2.0}, Renderable { color: mq::YELLOW }, PlayerTag {} )
        );
}