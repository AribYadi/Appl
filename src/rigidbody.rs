use rapier2d::{
  na::Vector2,
  prelude::*,
};

use crate::world::ApplWorld;

pub enum BodyType {
  Dynamic,
  Static,
}
impl ApplWorld {
  /// Adds a rigid body to the world
  pub fn add_rigid_body(
    &mut self,
    rigid_body_type: BodyType,
    size: Vector2<f32>,
    position: Vector2<f32>,
  ) -> RigidBodyHandle {
    let rigid_body_type = match rigid_body_type {
      BodyType::Dynamic => RigidBodyType::Dynamic,
      BodyType::Static => RigidBodyType::Static,
    };
    let rigid_body = RigidBodyBuilder::new(rigid_body_type)
      .translation(position / self.scale)
      .build();
    let collider =
      ColliderBuilder::cuboid(size.x / 2.0 / self.scale, size.y / 2.0 / self.scale).build();
    let rb_handle = self.rigid_bodies.insert(rigid_body);
    self
      .colliders
      .insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);

    rb_handle
  }

  /// Returns a reference to the rigid body using the rigid body handle
  pub fn get_rigid_body(&self, handle: RigidBodyHandle) -> &RigidBody { &self.rigid_bodies[handle] }

  // Returns the position of a rigid body got using the rigid body handle
  pub fn get_rigid_body_pos(&self, handle: RigidBodyHandle) -> Vector<f32> {
    (&self.rigid_bodies[handle]).translation() * self.scale
  }
}
