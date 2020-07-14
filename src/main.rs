use macroquad as mq;
use shipyard::*;


struct GameState {
    ecs: World,
}

impl GameState {
    fn render(&self) {
        // Draw renderables system
        self.ecs.run(|positions: View<Position>, renders: View<Renderable>| {
            for (pos, rend ) in (&positions, &renders).iter() {
                 
                mq::draw_circle(pos.x, pos.y, 12.0, rend.color);
            }
        });
    }
}

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
struct Player;

struct LeftMoverTag {}


#[macroquad::main("GI")]
async fn main() {
    let scr_width = mq::screen_width();
    let scr_height = mq::screen_height();

    // Create global state/world
    let gs = GameState {
        ecs: World::new(),
    };

    // Add screen width as unique
    gs.ecs.add_unique(scr_width);

    // Add a player entity
    gs.ecs.run(|mut entities: EntitiesViewMut, mut positions: ViewMut<Position>, mut renders: ViewMut<Renderable>, mut players: ViewMut<Player>| {
        entities.add_entity(
            (&mut positions, &mut renders, &mut players),
            (Position {x: scr_width / 2.0, y: scr_height / 2.0}, Renderable { color: mq::YELLOW }, Player {} ))
    });

    // Add LeftMover type entities
    gs.ecs.run(add_left_movers);

    
    // frame counter
    let mut frame_counter: u32 = 0;

    loop {
        // Print all positional entities
        if frame_counter % 120 == 0 {
            gs.ecs.run(|positions: View<Position>| {
                for (id, i) in positions.iter().with_id() {
                    println!("Entity # {:?} has id {:?}", i, id);
                }
            });
        }

        // UPDATE
        gs.ecs.run(left_mover_sys);

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
fn left_mover_sys(scr_width: UniqueView<f32>, mut positions: ViewMut<Position>, left_movers: View<LeftMoverTag>) {
    for (pos, _) in (&mut positions, &left_movers).iter() {
        pos.x += 2.0;
        if pos.x > *scr_width {
            pos.x = 0.0;
        }
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