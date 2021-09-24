use rapier2d::{
  na::Vector2,
  prelude::*,
};

use crate::{
  vector::Vect2,
  world::ApplWorld,
};

/// Rigid body types
#[derive(Debug, Clone, Copy)]
pub enum BodyType {
  Dynamic,
  Static,
}
/// Collider shapes
#[derive(Debug, Clone, Copy)]
pub enum BodyShape {
  Cuboid { width: f32, height: f32 },
  Circle { radius: f32 },
}
impl ApplWorld {
  /// Adds a rigid body to the world
  pub fn add_rigid_body(
    &mut self,
    rigid_body_type: BodyType,
    collider_shape: BodyShape,
    restitution: f32,
    position: Vect2<f32>,
  ) -> RigidBodyHandle {
    let position: Vector2<f32> = position.into();

    let rigid_body_type = match rigid_body_type {
      BodyType::Dynamic => RigidBodyType::Dynamic,
      BodyType::Static => RigidBodyType::Static,
    };
    let collider_shape = match collider_shape {
      BodyShape::Cuboid { width, height } => {
        SharedShape::cuboid(width / 2.0 / self.scale, height / 2.0 / self.scale)
      },
      BodyShape::Circle { radius } => SharedShape::ball(radius / self.scale),
    };

    let rigid_body = RigidBodyBuilder::new(rigid_body_type)
      .translation(position / self.scale)
      .build();
    let collider = ColliderBuilder::new(collider_shape)
      .restitution(restitution)
      .build();
    let rb_handle = self.rigid_bodies.insert(rigid_body);
    self
      .colliders
      .insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);

    rb_handle
  }

  /// Returns a reference to the rigid body using the rigid body handle
  pub fn get_rigid_body(&self, handle: RigidBodyHandle) -> &RigidBody { &self.rigid_bodies[handle] }

  /// Returns the position of a rigid body got using the rigid body handle
  pub fn get_rigid_body_pos(&self, handle: RigidBodyHandle) -> Vect2<f32> {
    ((&self.rigid_bodies[handle]).translation() * self.scale).into()
  }
}
