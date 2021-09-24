use appl::*;
use macroquad::prelude::*;

fn win_conf() -> Conf {
  Conf {
    window_width: 640,
    window_height: 480,
    ..Default::default()
  }
}

#[macroquad::main(win_conf)]
async fn main() {
  // Initialize Appl World
  let mut appl = ApplWorld::new(Vect2 { x: 0.0, y: 9.81 }, 50.0);
  // Create ground rigid body
  let ground = appl.add_rigid_body(
    BodyType::Static,
    BodyShape::Cuboid {
      width: 64.0,
      height: 32.0,
    },
    0.3,
    Vect2 { x: 0.0, y: 320.0 },
  );
  // Create box rigid body
  let box1 = appl.add_rigid_body(
    BodyType::Dynamic,
    BodyShape::Cuboid {
      width: 32.0,
      height: 32.0,
    },
    0.3,
    Vect2 { x: 0.0, y: 0.0 },
  );

  loop {
    clear_background(WHITE);

    // Update the world
    appl.step();

    // Get the current position of the rigid body and draw a rectangle
    let box1_pos = appl.get_rigid_body_pos(box1);
    draw_rectangle(box1_pos.x, box1_pos.y, 32.0, 32.0, RED);
    let ground_pos = appl.get_rigid_body_pos(ground);
    draw_rectangle(ground_pos.x, ground_pos.y, 64.0, 32.0, BLUE);

    next_frame().await
  }
}
