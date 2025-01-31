use crate::prelude::*;

use super::control::RocketControl;

#[derive(Component)]
pub struct Rocket;

#[derive(Component)]
pub struct RocketName(pub String);

#[derive(Component)]
pub struct RocketEngine {
    pub name: String,
    pub degrees_of_freedom: f32,
    pub max_thrust: f32,
    pub motor_max_force: f32,
    pub motor_stiffness: f32,
    pub motor_damping: f32,
    pub radius: f32,
    pub height: f32,
}

impl Default for NozzleBundle {
    fn default() -> Self {
        let engine = RocketEngine {
            name: "Nozzle Engine".to_string(),
            degrees_of_freedom: 7.0, // ° (degrees)
            max_thrust: 845.0,       // N (Newtons)
            motor_max_force: 5000.0, // N (Newtons)
            motor_stiffness: 2.5e4,  // N/m (Newtons per meter)
            motor_damping: 2.0e3,    // Ns/m (Newton-seconds per meter)
            radius: 0.095,           // m
            height: 0.2,             // m
        };

        let angle = engine.degrees_of_freedom.to_radians();

        let body_anchor = Vec3::new(0.0, -4.7 / 2.0, 0.0);
        let engine_anchor = Vec3::new(0.0, engine.height / 2.0, 0.0);

        let spherical_joint = SphericalJointBuilder::new()
            .local_anchor1(body_anchor)
            .local_anchor2(engine_anchor)
            .limits(JointAxis::AngX, [-angle, angle])
            .limits(JointAxis::AngZ, [-angle, angle])
            .limits(JointAxis::AngY, [0.0, 0.0])
            .motor_position(
                JointAxis::AngX,
                0.0,
                engine.motor_stiffness,
                engine.motor_damping,
            )
            .motor_position(
                JointAxis::AngZ,
                0.0,
                engine.motor_stiffness,
                engine.motor_damping,
            )
            .motor_max_force(JointAxis::AngX, engine.motor_max_force)
            .motor_max_force(JointAxis::AngZ, engine.motor_max_force)
            .build();

        Self {
            transform: Transform::from_translation(engine_anchor),
            visibility: Visibility::Visible,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cone(engine.height / 2.0, engine.radius),
            joint: ImpulseJoint::new(Entity::from_raw(0), spherical_joint),
            engine,
        }
    }
}

#[derive(Component)]
pub struct RocketBody {
    pub height: f32, // m
    pub radius: f32, // m
}

impl Default for RocketBody {
    fn default() -> Self {
        Self {
            height: 4.7,   // m
            radius: 0.185, // m
        }
    }
}

#[derive(Bundle)]
pub struct RocketBundle {
    name: RocketName,
    body: BodyBundle,
    nozzle: NozzleBundle,
}

#[derive(Bundle)]
pub struct BodyBundle {
    pub transform: Transform,
    pub visibility: Visibility,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub body: RocketBody,
}

#[derive(Bundle)]
pub struct NozzleBundle {
    pub transform: Transform,
    pub visibility: Visibility,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub joint: ImpulseJoint,
    pub engine: RocketEngine,
}

impl Default for RocketBundle {
    fn default() -> Self {
        Self {
            name: RocketName("Default Rocket".to_string()),
            body: BodyBundle::default(),
            nozzle: NozzleBundle::default(),
        }
    }
}

impl Default for BodyBundle {
    fn default() -> Self {
        let body = RocketBody::default();

        Self {
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
            visibility: Visibility::Visible,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cylinder(body.height / 2.0, body.radius),
            body,
        }
    }
}

pub fn spawn_rocket(mut commands: Commands) {
    let rocket_bundle = RocketBundle::default();

    let body_ent = commands
        .spawn((
            rocket_bundle.body,
            RocketControl::default(),
            Rocket,
            ExternalForce::default(),
        ))
        .id();
    commands.entity(body_ent).with_children(|parent| {
        let mut nozzle: NozzleBundle = rocket_bundle.nozzle;
        nozzle.joint.parent = body_ent;
        parent.spawn(nozzle);
    });
}
