```rust

  #[derive(Component, PartialEq, Debug, Reflect, Clone)]
  #[require(Name, Transform, RigidBody, Collider, rocket::EngineControl)]
  pub struct RocketEngine;
   ...
  impl RocketSettings {
    ...
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        ...
        // Engine
        let engine_id = commands
          .spawn((
              RocketEngine,
              rocket::EngineControl,
              rocket::EngineSettings {
                  degrees_of_freedom: engine_degrees_of_freedom,
                  max_thrust: engine_max_thrust,
                  motor_max_force: engine_motor_max_force,
                  motor_stiffness: engine_motor_stiffness,
                  motor_damping: engine_motor_damping,
                  delta_angle: 1.0,
                  delta_thrust: 10.0,
              },
              Name::new(format!("{name} Engine")),
              engine_transform,
              RigidBody::Dynamic,
              Collider::cone(engine_height / 2.0, engine_radius),
              ColliderMassProperties::Mass(body_dry_mass),
              AdditionalMassProperties::Mass(engine_mass),
              // Engine needs to be able to apply forces to the rocket body.
              ExternalForce::default(),
          ))
          .id();
        ...
    }
    ...
}
```
