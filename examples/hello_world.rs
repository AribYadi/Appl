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
  let mut appl = ApplWorld::new(new_vec2(0.0, 9.81), 50.0);
  // Create ground rigid body
  let ground = appl.add_rigid_body(
    BodyType::Static,
    0.3,
    new_vec2(64.0, 32.0),
    new_vec2(0.0, 320.0),
  );
  // Create box rigid body
  let box1 = appl.add_rigid_body(
    BodyType::Dynamic,
    0.3,
    new_vec2(32.0, 32.0),
    new_vec2(0.0, 0.0),
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
