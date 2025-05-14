use crate::prelude::*;
use bevy_rapier3d::prelude::AdditionalMassProperties;

const P_AMB: f32 = 100_000.0;
const G0: f32 = 9.807;

pub fn mass_depletion_system(
    time: Res<Time>,
    mut body_q: Query<
        (&mut AdditionalMassProperties, &rocket::RocketConfig),
        With<rocket::RocketBody>,
    >,
    engine_q: Query<&rocket::EngineControlState, With<rocket::RocketEngine>>,
) {
    let total_thrust: f32 = engine_q.iter().map(|ecs| ecs.thrust).sum();
    if total_thrust <= 0.0 {
        return;
    }

    for (mut add_mass, cfg) in &mut body_q {
        let isp = cfg.0.i_sp as f32;
        let a_noz = cfg.0.a_nozzle as f32;

        let alpha = 1.0 / (isp * G0);
        let m_dot_bp = (P_AMB * a_noz) / (isp * G0);
        let m_dot = alpha * total_thrust + m_dot_bp;

        let delta_m = m_dot * time.delta_secs();

        if let AdditionalMassProperties::Mass(current) = *add_mass {
            let new_mass = (current - delta_m).max(0.0);
            *add_mass = AdditionalMassProperties::Mass(new_mass);
        }
    }
}
