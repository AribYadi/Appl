use rapier2d::prelude::*;

pub struct ApplWorld {
  pub gravity: Vector<f32>,
  pub integration_parameters: IntegrationParameters,
  pub physics_pipeline: PhysicsPipeline,
  pub island_manager: IslandManager,
  pub broad_phase: BroadPhase,
  pub narrow_phase: NarrowPhase,
  pub joints: JointSet,
  pub ccd_solver: CCDSolver,
  pub physics_hooks: (),
  pub event_handler: (),

  pub rigid_bodies: RigidBodySet,
  pub colliders: ColliderSet,

  pub scale: f32,
}
impl ApplWorld {
  pub fn new(gravity: Vector<f32>, scale: f32) -> Self {
    let rigid_bodies = RigidBodySet::new();
    let colliders = ColliderSet::new();

    let integration_parameters = IntegrationParameters::default();
    let physics_pipeline = PhysicsPipeline::new();
    let island_manager = IslandManager::new();
    let broad_phase = BroadPhase::new();
    let narrow_phase = NarrowPhase::new();
    let joints = JointSet::new();
    let ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    Self {
      gravity,
      integration_parameters,
      physics_pipeline,
      island_manager,
      broad_phase,
      narrow_phase,
      joints,
      ccd_solver,
      physics_hooks,
      event_handler,

      rigid_bodies,
      colliders,

      scale,
    }
  }

  pub fn step(&mut self) {
    self.physics_pipeline.step(
      &self.gravity,
      &self.integration_parameters,
      &mut self.island_manager,
      &mut self.broad_phase,
      &mut self.narrow_phase,
      &mut self.rigid_bodies,
      &mut self.colliders,
      &mut self.joints,
      &mut self.ccd_solver,
      &self.physics_hooks,
      &self.event_handler,
    );
  }
}
