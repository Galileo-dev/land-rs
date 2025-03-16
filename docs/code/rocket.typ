
This module is responsible for handling the rocket body and its sub-components such as the rocket engine and rocket control.

The rocket consists of two main physical bodies:

Rocket body - the main fuselage of the rocket

```rust
// Rocket component definition
#[derive(Component, Default, Debug, Reflect)]
#[require(Name, RocketBody)]
pub struct Rocket;

// Rocket body component
#[derive(Component, Default, PartialEq, Debug, Reflect, Clone)]
#[require(Transform, RigidBody, Collider, RocketEngine, RocketControl)]
pub struct RocketBody;

// Rocket engine with physics properties
#[derive(Component, PartialEq, Debug, Reflect, Clone)]
#[require(Name, Transform, RigidBody, Collider)]
pub struct RocketEngine {
    pub degrees_of_freedom: Real,
    pub max_thrust: Real,
    pub motor_max_force: Real,
    pub motor_stiffness: Real,
    pub motor_damping: Real,
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

 commands.entity(rocket_body_id).add_child(nozzle_id);
    let joint: SphericalJoint = SphericalJointBuilder::new()
        .limits(JointAxis::AngX, [-angle, angle])
        .limits(JointAxis::AngZ, [-angle, angle])
        .limits(JointAxis::AngY, [0.0, 0.0])
        .local_anchor1(Vec3::new(0.0, -body_height / 2.0, 0.0)) // Rocket body's bottom
        .local_anchor2(Vec3::new(0.0, nozzle_height / 2.0, 0.0)) // Nozzle's top
        // Keep uprightness of the nozzle
        .motor(JointAxis::AngX, 0.0, 0.0, 0.0, 1.0)
        .motor(JointAxis::AngZ, 0.0, 0.0, 0.0, 1.0)
        // max force
        .motor_max_force(JointAxis::AngX, motor_max_force)
        .motor_max_force(JointAxis::AngZ, motor_max_force)
        .build();
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


