use std::ops::{
  Add,
  Div,
  Mul,
  Sub,
};

use nalgebra::{
  vector,
  Vector2,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
/// Vector2 struct
pub struct Vect2<T: std::fmt::Debug> {
  pub x: T,
  pub y: T,
}
impl<T: std::fmt::Debug> Vect2<T> {
  fn new(x: T, y: T) -> Self { Self { x, y } }

  fn zero_float() -> Vect2<f32> { Vect2 { x: 0.0, y: 0.0 } }
}

impl<T: std::fmt::Debug> From<Vect2<T>> for Vector2<T> {
  fn from(vect2: Vect2<T>) -> Vector2<T> { vector![vect2.x, vect2.y] }
}
impl<T: std::fmt::Debug + std::marker::Copy> From<Vector2<T>> for Vect2<T> {
  fn from(vector2: Vector2<T>) -> Self {
    Self {
      x: vector2.data.0[0][0],
      y: vector2.data.0[0][1],
    }
  }
}
impl<T: std::fmt::Debug + Add<Output = T>> Add for Vect2<T> {
  type Output = Vect2<T>;

  fn add(self, rhs: Vect2<T>) -> Self {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}
impl<T: std::fmt::Debug + Sub<Output = T>> Sub for Vect2<T> {
  type Output = Vect2<T>;

  fn sub(self, rhs: Vect2<T>) -> Self {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}
impl<T: std::fmt::Debug + Mul<Output = T>> Mul for Vect2<T> {
  type Output = Vect2<T>;

  fn mul(self, rhs: Vect2<T>) -> Self {
    Self {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
    }
  }
}
impl<T: std::fmt::Debug + Div<Output = T>> Div for Vect2<T> {
  type Output = Vect2<T>;

  fn div(self, rhs: Vect2<T>) -> Self {
    Self {
      x: self.x / rhs.x,
      y: self.y / rhs.y,
    }
  }
}
