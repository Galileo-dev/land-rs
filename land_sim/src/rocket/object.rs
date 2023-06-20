use bevy::{ecs::bundle, prelude::*};
use bevy_rapier3d::{
    prelude::*,
    rapier::prelude::{ColliderBuilder, JointAxis, RigidBodyBuilder},
};

use super::control::RocketControl;

#[derive(Component)]
pub struct Rocket;

#[derive(Bundle)]
pub struct RocketBundle {
    body: BodyBundle,
    nozzle: NozzleBundle,
    #[bundle(ignore)]
    joint: SphericalJoint,
}

#[derive(Bundle)]
struct BodyBundle {
    body: RigidBody,
    collider: Collider,
    transform: Transform,
    global_transform: GlobalTransform,
}

#[derive(Bundle)]
struct NozzleBundle {
    transform: Transform,
    global_transform: GlobalTransform,
    rigid_body: RigidBody,
    collider: Collider,
}

impl Default for RocketBundle {
    fn default() -> Self {
        let body = BodyBundle::default();
        let nozzle = NozzleBundle::default();

        let degrees_of_freedom = 15;
        // degrees to radians
        let angle = 2.0 * std::f32::consts::PI / degrees_of_freedom as f32;

        let joint = SphericalJointBuilder::new()
            // first anchor is at the bottom of the rocket
            .local_anchor1(Vec3::new(0.0, 1.0, 0.0))
            // second anchor is at the top of the nozzle
            .local_anchor2(Vec3::new(0.0, -3.0, 0.0))
            .limits(JointAxis::AngX, [-angle, angle])
            .limits(JointAxis::AngZ, [-angle, angle])
            .limits(JointAxis::AngY, [0.0, 0.0])
            // we want an x motor and z motor to keep the rocket pointed in the right direction
            .motor_position(JointAxis::AngX, 0.1, 10000.0, 100.0)
            .motor_position(JointAxis::AngZ, 0.1, 10000.0, 100.0)
            .motor_max_force(JointAxis::AngX, 100.0)
            .motor_max_force(JointAxis::AngZ, 100.0)
            .build();

        Self {
            body,
            nozzle,
            joint,
        }
    }
}

impl Default for NozzleBundle {
    fn default() -> Self {
        let rigid_body = RigidBody::Dynamic;
        let collider = Collider::cone(1.0, 1.0);
        let nozzle_transform = Transform::from_translation(Vec3::new(0.0, 2.0, 0.0));
        let nozzle_global_transform = GlobalTransform::default();

        Self {
            rigid_body,
            collider,
            transform: nozzle_transform,
            global_transform: nozzle_global_transform,
        }
    }
}

impl Default for BodyBundle {
    fn default() -> Self {
        let body = RigidBody::Dynamic;
        let collider = Collider::cuboid(1.0, 3.0, 1.0);
        let transform = Transform::from_translation(Vec3::new(0.0, 5.0, 0.0));
        let global_transform = GlobalTransform::default();

        Self {
            body,
            collider,
            transform,
            global_transform,
        }
    }
}

pub fn spawn_rocket(mut commands: Commands) {
    let rocket_bundle = RocketBundle::default();

    // spawn the body
    commands
        .spawn((rocket_bundle.body, RocketControl::default()))
        .with_children(|parent| {
            // spawn the nozzle
            let nozzle = parent.spawn(rocket_bundle.nozzle).id();
            // spawn the joint
            parent.spawn(ImpulseJoint::new(nozzle, rocket_bundle.joint));
        })
        .insert(Rocket);
}
