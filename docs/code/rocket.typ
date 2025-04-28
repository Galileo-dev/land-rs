This module is responsible for handling the rocket body and its sub-components such as the rocket engine and rocket control.

The rocket consists of two main physical bodies:
- *Rocket body:* the main fuselage of the rocket
  ```rust

  #[derive(Component, Default, Debug, Reflect)]
  #[require(Name, RocketBody)]
  pub struct RocketRoot;

  #[derive(Component, Default, PartialEq, Debug, Reflect, Clone)]
  #[require(Transform, RigidBody, Collider)]
  pub struct RocketBody;

  ...
  impl RocketSettings {
      ...
      pub fn spawn(self, commands: &mut Commands) -> Entity {
          ...
          // Body
          let body_id = commands
              .spawn((
                  Name::new(name.clone()),
                  RocketRoot,
                  RocketBody,
                  RigidBody::Dynamic,
                  initial_transform,
                  initial_velocity,
                  Collider::cylinder(body_height / 2.0, body_radius),
                  ColliderMassProperties::Mass(body_dry_mass),
                  AdditionalMassProperties::Mass(body_fuel_mass),
              ))
              .id();
          ...
      }
      ...
  }
  ```

- *Rocket engine*: the rocket engine that is attached via a spherical joint to the rocket body
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

The rocket engine is a `RigidBody` cone connected to the rocket body through a spherical joint. The joint can act as a motor to control the rocket engine's pitch and yaw. Both the rocket body and the rocket engine have `Collider` components to interact with the physics simulation, However they are part of the same `CollisionGroups` so they don't collide with each other.

==== Spawn engine nozzle
We initially spawn the engine nozzle just below the rocket body
- For the rocket body we choose the bottom centre as the attachment point:\
  ```rust
  (0.0, -body_height / 2.0, 0.0).
  ```
- For the nozzle we choose the apex as it's attachment point:
  ```rust
  (0.0, nozzle_height / 2.0, 0.0).
  ```

  These anchor points are specified in each body's local coordinate system.


```rust
...
impl RocketSettings {
  ...
  pub fn spawn(self, commands: &mut Commands) -> Entity {
    ...
    // ----- Spawn engine nozzle -----
    // We initially spawn the engine nozzle just below the rocket body
    // - For the rocket body we choose the bottom centre as the attachment point:
    //   (0.0, -body_height / 2.0, 0.0).
    // - For the nozzle we choose the apex as it's attachment point:
    //   (0.0, nozzle_height / 2.0, 0.0).
    //
    //   These anchor points are specified in each bodies local coordinate system.
    //

    let gimbal_angle = engine_degrees_of_freedom.to_radians();

    let joint = SphericalJointBuilder::new()
      // Constrain pitch / roll equally – yaw is locked.
      .limits(JointAxis::AngX, [-gimbal_angle, gimbal_angle])
      .limits(JointAxis::AngZ, [-gimbal_angle, gimbal_angle])
      .limits(JointAxis::AngY, [0.0, 0.0])
      // Anchor the joint so the engine apex sits beneath the body.
      .local_anchor1(Vec3::new(0.0, -body_height / 2.0, 0.0))
      .local_anchor2(Vec3::new(0.0, engine_height / 2.0, 0.0))
      // Keep uprightness of the nozzle
      .motor(JointAxis::AngX, 0.0, 0.0, 0.0, 1.0)
      .motor(JointAxis::AngZ, 0.0, 0.0, 0.0, 1.0)
      // max force
      .motor_max_force(JointAxis::AngX, engine_motor_max_force)
      .motor_max_force(JointAxis::AngZ, engine_motor_max_force)
      .build();

    // Attach the joint from the nozzle to the rocket
    commands
        .entity(engine_id)
        .insert(ImpulseJoint::new(body_id, joint));

    body_id
    ...
  }
  ...
}
...
```

==== Attach the joint from the nozzle to the rocket
```rust
    let joint: SphericalJoint = SphericalJointBuilder::new()
        //...
        .build();

    commands
        .entity(nozzle_id)
        .insert(ImpulseJoint::new(rocket_body_id, joint));
    }
}
```


