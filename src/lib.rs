mod rigidbody;
mod world;

use nalgebra::{
  vector,
  Vector2,
};

pub use rigidbody::*;
pub use world::*;

pub fn new_vec2<T>(x: T, y: T) -> Vector2<T> { vector![x, y] }
