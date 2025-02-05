use bon::Builder;

use crate::prelude::*;

use super::control::RocketControl;

#[derive(Component, Default, Debug, Reflect)]
#[require(Name, RocketBody)]
pub struct Rocket;

#[derive(Component, Default, PartialEq, Debug, Reflect, Clone)]
#[require(Transform, RigidBody, Collider, RocketEngine, RocketControl)]
pub struct RocketBody;

#[derive(Component, PartialEq, Debug, Reflect, Clone)]
#[require(Name, Transform, RigidBody, Collider)]
pub struct RocketEngine {
    pub degrees_of_freedom: Real,
    pub max_thrust: Real,
    pub motor_max_force: Real,
    pub motor_stiffness: Real,
    pub motor_damping: Real,
}

impl Default for RocketEngine {
    fn default() -> Self {
        Self {
            degrees_of_freedom: 45.0,
            max_thrust: 2000.0,
            motor_max_force: 3000.0,
            motor_stiffness: 8000.0,
            motor_damping: 700.0,
        }
    }
}

pub fn spawn_rocket(mut commands: Commands) {
    // Define collision group for the rocket parts.
    let rocket_group = Group::GROUP_1;

    let start_height = 0.001;

    let body_height = 4.0;
    let body_radius = 0.5;

    let nozzle_height = 0.5;
    let nozzle_radius = 0.3;

    // Define engine parameters.
    // Spawn the main rocket entity (body) with its required components.
    let rocket_body_id = commands
        .spawn((
            Rocket,
            RocketBody,
            Name::new("Rocket"),
            RigidBody::Dynamic,
            Transform::from_xyz(0.0, start_height + nozzle_height + (body_height / 2.0), 0.0),
            Collider::cylinder(body_height / 2.0, body_radius),
            Friction::new(0.1),
            ColliderMassProperties::Mass(100.0),
            CollisionGroups::new(rocket_group, !rocket_group),
        ))
        .id();

    let rocket_engine_bundle = RocketEngine::default();
    let angle = rocket_engine_bundle.degrees_of_freedom.to_radians();
    let motor_max_force = rocket_engine_bundle.motor_max_force;

    let nozzle_id = commands
        .spawn((
            rocket_engine_bundle,
            Name::new("Rocket Engine"),
            Transform::from_xyz(0.0, start_height + (nozzle_height / 2.0), 0.0),
            RigidBody::Dynamic,
            Collider::cone(nozzle_height / 2.0, nozzle_radius),
            Friction::new(0.1),
            ColliderMassProperties::Mass(10.0),
            CollisionGroups::new(rocket_group, !rocket_group),
            // Engine needs to be able to apply forces to the rocket body.
            ExternalForce::default(),
        ))
        .id();

    // Make nozzle a child of the rocket body
    // commands.entity(rocket_body_id).add_child(nozzle_id);

    // ----- Spawn engine nozzle -----
    // We initially spawn the engine nozzle just below the rocket body
    // - For the rocket body we choose the bottom centre as the attachment point:
    //   (0.0, -body_height / 2.0, 0.0).
    // - For the nozzle we choose the apex as it's attachment point:
    //   (0.0, nozzle_height / 2.0, 0.0).
    //
    //   These anchor points are specified in each bodies local coordinate system.
    //
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

    // Attach the joint from the nozzle to the rocket
    commands
        .entity(nozzle_id)
        .insert(ImpulseJoint::new(rocket_body_id, joint));
}
