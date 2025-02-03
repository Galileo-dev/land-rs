use bon::Builder;

use crate::prelude::*;

use super::control::RocketControl;

#[derive(Component, Default, Debug, Reflect)]
#[require(Name, RocketBody)]
pub struct Rocket;

#[derive(Component, Default, PartialEq, Debug, Reflect, Clone)]
#[require(Transform, RigidBody, Collider, RocketEngine)]
pub struct RocketBody;

#[derive(Component, PartialEq, Debug, Reflect, Clone)]
#[require(Name, Transform, RigidBody, Collider)]
pub struct RocketEngine {
    degrees_of_freedom: Real,
    max_thrust: Real,
    motor_max_force: Real,
    motor_stiffness: Real,
    motor_damping: Real,
}

impl Default for RocketEngine {
    fn default() -> Self {
        Self {
            degrees_of_freedom: 45.0,
            max_thrust: 100.0,
            motor_max_force: 100.0,
            motor_stiffness: 0.1,
            motor_damping: 0.1,
        }
    }
}

// fn create_spherical_joint(
//     commands: &mut Commands, origin: Vect
// ) -> GenericJoint {
//     let shift = 1.0;

//     let angle = degrees_of_freedom.to_radians();
//     let spherical_joint = SphericalJointBuilder::new()
//         .limits(JointAxis::AngX, [-angle, angle])
//         .limits(JointAxis::AngZ, [-angle, angle])
//         .limits(JointAxis::AngY, [0.0, 0.0])
//         .motor_position(JointAxis::AngX, 0.0, motor_stiffness, motor_damping)
//         .motor_position(JointAxis::AngZ, 0.0, motor_stiffness, motor_damping)
//         .motor_max_force(JointAxis::AngX, motor_max_force)
//         .motor_max_force(JointAxis::AngZ, motor_max_force)
//         .build();
//     spherical_joint.into()
// }

pub fn spawn_rocket(mut commands: Commands) {
    // Define collision group for the rocket parts.
    let rocket_group = Group::GROUP_1;

    let start_height = 5.0;

    let rocket_body_height = 5.0;

    let rocket_engine_height = 2.0;

    // Define engine parameters.
    // Spawn the main rocket entity (body) with its required components.
    let rocket_id = commands
        .spawn((
            Name::new("Rocket"),
            RocketBody,
            RigidBody::Dynamic,
            Transform::from_xyz(0.0, start_height, 0.0),
            Collider::cylinder(rocket_body_height / 2.0, 1.0),
            ColliderMassProperties::Mass(100.0),
            CollisionGroups::new(rocket_group, !rocket_group),
        ))
        .id();

    let engine_id = commands.spawn((
        RocketEngine::default(),
        Name::new("Rocket Engine"),
        Transform::from_xyz(
            0.0,
            start_height - (rocket_body_height / 2.0) - (rocket_engine_height / 2.0),
            0.0,
        ),
        RigidBody::Dynamic,
        Collider::cone(rocket_engine_height / 2.0, 1.0),
        ColliderMassProperties::Mass(100.0),
        CollisionGroups::new(rocket_group, !rocket_group),
        // Attach the joint to connect with the rocket body.
        // joint,
    ));
}
