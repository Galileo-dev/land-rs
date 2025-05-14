use bon::*;
use gfold_rs::rocket_config::RocketConfig as GfoldRocketConfig;

use crate::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct RocketConfig(pub GfoldRocketConfig);

#[derive(Component, Default, Debug, Reflect)]
#[require(Name, RocketBody)]
pub struct RocketRoot;

#[derive(Component, Default, PartialEq, Debug, Reflect, Clone)]
#[require(Transform, RigidBody, Collider)]
pub struct RocketBody;

#[derive(Component, PartialEq, Debug, Reflect, Clone)]
#[require(Name, Transform, RigidBody, Collider, rocket::EngineControl)]
pub struct RocketEngine;

#[derive(Builder, Clone)]
pub struct RocketSettings {
    // General
    pub name: String,
    pub initial_transform: Transform,
    pub initial_velocity: Velocity,

    // Body
    pub body_height: Real,
    pub body_radius: Real,
    pub body_fuel_mass: Real,
    pub body_dry_mass: Real,

    // Engine
    pub engine_height: Real,
    pub engine_radius: Real,
    pub engine_mass: Real,

    // Engine control
    pub engine_degrees_of_freedom: Real,
    pub engine_max_thrust: Real,
    pub engine_motor_max_force: Real,
    pub engine_motor_stiffness: Real,
    pub engine_motor_damping: Real,

    pub specific_impulse: Real,
    pub nozzle_area: Real,
    pub min_vacuum_thrust: Real,
    pub min_thrust_rate: Real,
    pub max_thrust_rate: Real,
    pub drag_s_d: Real,
    pub drag_c_d: Real,

    // Physics
    #[builder(default = false)]
    pub ignore_internal_collisions: bool,
    #[builder(default = true)]
    pub enable_ccd: bool,
    #[builder(default = true)]
    pub disable_sleeping: bool,
}

impl RocketSettings {
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        // Unpack the settings
        let RocketSettings {
            name,
            mut initial_transform,
            initial_velocity,
            body_height,
            body_radius,
            body_fuel_mass,
            body_dry_mass,
            engine_height,
            engine_radius,
            engine_mass,
            engine_degrees_of_freedom,
            engine_max_thrust,
            engine_motor_max_force,
            engine_motor_stiffness,
            engine_motor_damping,
            specific_impulse,
            nozzle_area,
            min_vacuum_thrust,
            min_thrust_rate,
            max_thrust_rate,
            drag_s_d,
            drag_c_d,
            ignore_internal_collisions,
            enable_ccd,
            disable_sleeping,
        } = self;

        initial_transform.translation.y += engine_height + body_height / 2.0;

        let gfold_config_data = GfoldRocketConfig {
            m_dry: body_dry_mass.into(),
            m_fuel: body_fuel_mass.into(),
            i_sp: specific_impulse.into(),
            a_nozzle: nozzle_area.into(),
            t_min_vac: min_vacuum_thrust.into(),
            t_max_vac: engine_max_thrust.into(),
            tdot_min: min_thrust_rate.into(),
            tdot_max: max_thrust_rate.into(),
            theta_max: engine_degrees_of_freedom.into(),
            s_d: drag_s_d.into(),
            c_d: drag_c_d.into(),
        };
        let rocket_config_component = RocketConfig(gfold_config_data);

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
                rocket_config_component,
                ExternalForce::default(),
            ))
            .id();

        // Engine
        let engine_transform = {
            let mut t = initial_transform;
            t.translation.y -= (body_height / 2.0) + (engine_height / 2.0);
            t
        };

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
                    delta_thrust: 10_000.0,
                },
                Name::new(format!("{name} Engine")),
                engine_transform,
                RigidBody::Dynamic,
                Collider::cone(engine_height / 2.0, engine_radius),
                ColliderMassProperties::Mass(engine_mass),
                ExternalForce::default(),
            ))
            .id();

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

        // tidy up scene hierarchy.
        // commands.entity(body_id).add_child(engine_id);

        body_id
    }
}

pub fn spawn_rocket(mut commands: Commands) {
    // U-E-N -> E-U-N
    let _rocket_root = RocketSettings::builder()
        .name("Falcon 9".into())
        .initial_transform(Transform::from_xyz(500.0, 500.0, 0.0))
        .initial_velocity(Velocity::linear(Vec3::new(0.0, -50.0, 50.0)))
        .body_height(41.2)
        .body_radius(3.7)
        .body_dry_mass(10_000.0)
        .body_fuel_mass(5_000.0)
        .engine_height(10.0)
        .engine_radius(3.0)
        .engine_mass(1.0)
        .engine_degrees_of_freedom(15.0)
        .engine_max_thrust(250_000.0)
        .engine_motor_max_force(550_000.0)
        .engine_motor_stiffness(500_000.0)
        .engine_motor_damping(20_000.0)
        .specific_impulse(300.0)
        .nozzle_area(0.5)
        .min_vacuum_thrust(100_000.0)
        .min_thrust_rate(-100_000.0)
        .max_thrust_rate(100_000.0)
        .drag_s_d(10.0)
        .drag_c_d(1.0)
        .build()
        .spawn(&mut commands);
}
