use bevy::{ecs::bundle, prelude::*};
use bevy_rapier3d::{
    prelude::*,
    rapier::prelude::{ColliderBuilder, JointAxis, RigidBodyBuilder},
};

use super::control::RocketControl;

#[derive(Component)]
pub struct Rocket;

#[derive(Component)]
struct RocketName(String);

#[derive(Component)]
struct DegreesOfFreedom(pub u32);

#[derive(Bundle)]
pub struct RocketBundle {
    name: RocketName,
    body: BodyBundle,
    nozzle: NozzleBundle,
}

#[derive(Bundle)]
struct BodyBundle {
    rigid_body: RigidBody,
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
    #[bundle(ignore)]
    joint: SphericalJoint,
    degrees_of_freedom: DegreesOfFreedom,
}

impl Default for RocketBundle {
    fn default() -> Self {
        let body = BodyBundle::default();
        let nozzle = NozzleBundle::default();

        Self {
            name: RocketName("Rocket".into()),
            body,
            nozzle,
        }
    }
}

impl Default for NozzleBundle {
    fn default() -> Self {
        let rigid_body = RigidBody::Dynamic;
        let collider = Collider::cone(1.0, 1.0);
        let transform = Transform::from_translation(Vec3::new(0.0, 2.0, 0.0));
        let global_transform = GlobalTransform::default();

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
            rigid_body,
            collider,
            transform,
            global_transform,
            joint,
            degrees_of_freedom: DegreesOfFreedom(15),
        }
    }
}

impl Default for BodyBundle {
    fn default() -> Self {
        let rigid_body = RigidBody::Dynamic;
        let collider = Collider::cuboid(1.0, 3.0, 1.0);
        let transform = Transform::from_translation(Vec3::new(0.0, 5.0, 0.0));
        let global_transform = GlobalTransform::default();

        Self {
            rigid_body,
            collider,
            transform,
            global_transform,
        }
    }
}

pub fn spawn_rocket(mut commands: Commands) {
    let rocket_bundle = RocketBundle::default();

    let joint = rocket_bundle.nozzle.joint.clone();

    let body_ent = commands
        .spawn(())
        .insert(rocket_bundle.body)
        .insert(RocketControl::default())
        .insert(Rocket)
        .id();

    commands.entity(body_ent).with_children(|parent| {
        let nozzle_ent = parent.spawn(rocket_bundle.nozzle).id();

        parent.spawn(ImpulseJoint::new(body_ent, joint));
    });
}
