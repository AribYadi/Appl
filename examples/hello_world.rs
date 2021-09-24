use appl::*;
use macroquad::prelude::*;
use rapier2d::na::vector;

fn win_conf() -> Conf {
  Conf {
    window_width: 640,
    window_height: 480,
    ..Default::default()
  }
}

#[macroquad::main(win_conf)]
async fn main() {
  let mut appl = ApplWorld::new(vector![0.0, 9.81], 50.0);
  let ground = appl.add_rigid_body(BodyType::Static, vector![64.0, 32.0], vector![0.0, 320.0]);
  let box1 = appl.add_rigid_body(BodyType::Dynamic, vector![32.0, 32.0], vector![0.0, 0.0]);

  loop {
    clear_background(WHITE);

    appl.step();

    let box1_pos = appl.get_rigid_body(box1);
    draw_rectangle(box1_pos.x, box1_pos.y, 32.0, 32.0, RED);
    let ground_pos = appl.get_rigid_body(ground);
    draw_rectangle(ground_pos.x, ground_pos.y, 64.0, 32.0, BLUE);

    next_frame().await
  }
}
