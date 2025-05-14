This module is responsible for handling the rocket body and its sub-components such as the rocket engine and rocket control.

The rocket consists of two main physical bodies:
- *Rocket body:* the main fuselage of the rocket @rocket-body-code

- *Rocket engine*: the rocket engine that is attached via a spherical joint to the rocket body @rocket-engine-code


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

  These anchor points are specified in each body's local coordinate system.Attach the joint from the nozzle to the rocket @rocket-engine-joint-code
