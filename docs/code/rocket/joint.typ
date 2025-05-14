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
