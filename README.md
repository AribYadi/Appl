# Appl

Appl is a physics engine powered by Rapier. It is currently 2d only.

## Example

```rust
use appl::*;
use macroquad::prelude::*;

fn main() {
  let mut appl = ApplWorld::new(new_vec2(0.0, 9.81), 50.0);
  let ground = appl.add_rigid_body(BodyType::Static, 0.3, new_vec2(64.0, 32.0), new_vec2(0.0, 320.0));
  let box1 = appl.add_rigid_body(BodyType::Dynamic, 0.3, new_vec2(32.0, 32.0), new_vec2(0.0, 0.0));

  for _ in 0..200 {
    appl.step();

    let box1_pos = appl.get_rigid_body_pos(box1);
    println("{}, {}", box1_pos.x, box1_pos.y);
  }
}
```

## Project Status

This project is in progress and is in very very early stages

## License

[MIT](https://choosealicense.com/licenses/mit/)
