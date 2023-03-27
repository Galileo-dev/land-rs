use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::*,
    rapier::prelude::{ColliderBuilder, JointAxis, RigidBodyBuilder},
};

struct Rocket {
    body: Entity,
    nozzle: Entity,
    motor: Option<FixedJoint>,
}

pub fn setup_rocket(mut commands: Commands) {
    let nozzle_transform = Transform::from_translation(Vec3::new(0.0, 2.0, 0.0));
    let nozzle_global_transform = GlobalTransform::default();

    let nozzle = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cone(1.0, 1.0))
        // flip the cone so that the bottom is at the origin
        .insert(nozzle_transform)
        .insert(nozzle_global_transform)
        .id();

    let degrees_of_freedom = 15;
    // degrees to radians
    let angle = 2.0 * std::f32::consts::PI / degrees_of_freedom as f32;
    //

    let joint = SphericalJointBuilder::new()
        // first anchor is at the bottom of the rocket
        .local_anchor1(Vec3::new(0.0, 1.0, 0.0))
        // second anchor is at the top of the nozzle
        .local_anchor2(Vec3::new(0.0, -3.0, 0.0))
        .limits(JointAxis::AngX, [-angle, angle])
        .limits(JointAxis::AngZ, [-angle, angle])
        .limits(JointAxis::AngY, [0.0, 0.0])
        // we want an x motor and z motor to keep the rocket pointed in the right direction
        .motor_position(JointAxis::AngX, 1.0, 10000.0, 100.0)
        .motor_position(JointAxis::AngZ, 0.0, 10000.0, 100.0)
        .motor_max_force(JointAxis::AngX, 100000.0)
        .motor_max_force(JointAxis::AngZ, 100000.0)
        .build();

    // a rocket is made up of multiple parts
    // 1. the rocket body. this has no use other than to be a parent to the other parts
    let rocket_body = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(1.0, 3.0, 1.0))
        .insert(Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)))
        .insert(GlobalTransform::default())
        .insert(ImpulseJoint::new(nozzle, joint))
        .id();

    // 2. the rocket nose. this is the part that points forward

    // 3. the rocket engine exhaust. this is the part that points backwards and is the only part that moves

    // the rocket body
}
