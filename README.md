# Appl

![version](https://img.shields.io/crates/v/appl?style=flat-square)
![downloads](https://img.shields.io/crates/dv/appl?style=flat-square)
![license](https://img.shields.io/github/license/AribYadi/Appl?style=flat-square)

## Description

<img src="https://github.com/AribYadi/Appl/blob/master/Appl.svg" align="right" alt="Appl Logo" width="128" height="128">
Appl is a library that provides abstraction over the Rapier library.<br />
Appl was created due to the lack of any physics engine for beginners to use.<br />
Appl's main purpose is to be simple and easy to use.<br />

## Example

```rust
use appl::*;

fn main() {
  // Initialize Appl World
  let mut appl = ApplWorld::new(Vect2 { x: 0.0, y: 9.81 }, 50.0);
  // Create ground rigid body
  let ground = appl.add_rigid_body(
    BodyType::Static,
    0.3,
    BodyShape::Cuboid {
      width: 64.0,
      height: 32.0,
    },
    0.3,
    Vect2 { x: 0.0, y: 320.0 }
  );
  // Create box1 rigid body
  let box1 = appl.add_rigid_body(
    BodyType::Dynamic,
    0.3,
    BodyShape::Cuboid {
      width: 32.0,
      height: 32.0,
    },
    0.3,
    Vect2 { x: 0.0, y: 0.0 }
  );

  for _ in 0..200 {
    // Update the world
    appl.step();

    // Prints box1 position
    let box1_pos = appl.get_rigid_body_pos(box1);
    println("{}, {}", box1_pos.x, box1_pos.y);
  }
}
```

## How to contribute

Any pull request are definitely welcomed and appreciated! <br />
<br />
If you have any issues or questions, please open an issue and i will try to help you.

## Project Status

This project is in it's very very early stage and currently only works for 2d.

## License

[MIT](https://choosealicense.com/licenses/mit/)
