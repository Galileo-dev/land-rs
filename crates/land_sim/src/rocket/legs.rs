use crate::prelude::*;
use crate::rocket::control::EngineControlState;

/// Altitude below which the landing legs deploy [m].
const DEPLOY_ALT: f32 = 70.0;
/// Leg tilt when folded (nearly against the body) and deployed.
const FOLDED: f32 = 6.0_f32.to_radians();
const DEPLOYED: f32 = 40.0_f32.to_radians();
/// Deployment rate [rad/s].
const RATE: f32 = 45.0_f32.to_radians();

/// Folded leg tilt, used at spawn.
pub const FOLDED_ANGLE: f32 = 6.0_f32.to_radians();

/// One landing leg: a strut pivoted at the body's lower edge, folded against
/// the hull and rotating outward for touchdown. Visual only (no collider).
#[derive(Component, Debug)]
pub struct LandingLeg {
    /// Tangential tilt axis; positive rotation swings the leg outward.
    pub axis: Vec3,
    /// Current tilt angle [rad].
    pub angle: f32,
}

/// A cold-gas RCS thruster pod on the upper hull.
#[derive(Component, Debug)]
pub struct RcsPod {
    /// Position in the body's local frame.
    pub local_pos: Vec3,
}

/// Deploy the landing legs once the vehicle is low enough.
pub fn deploy_legs_system(
    time: Res<Time>,
    body_q: Query<&GlobalTransform, With<rocket::RocketRoot>>,
    mut legs: Query<(&mut Transform, &mut LandingLeg)>,
) {
    let Ok(body) = body_q.get_single() else {
        return;
    };
    let target = if body.translation().y < DEPLOY_ALT {
        DEPLOYED
    } else {
        FOLDED
    };

    for (mut transform, mut leg) in &mut legs {
        let step = RATE * time.delta_secs();
        leg.angle = if leg.angle < target {
            (leg.angle + step).min(target)
        } else {
            (leg.angle - step).max(target)
        };
        transform.rotation = Quat::from_axis_angle(leg.axis, leg.angle);
    }
}

/// Cold-gas RCS visual: puff from the upper pods against body rotation.
pub fn rcs_effect_system(
    body_q: Query<(&GlobalTransform, &Velocity), With<rocket::RocketRoot>>,
    pods: Query<(&GlobalTransform, &rocket::RcsPod)>,
    mut gizmos: Gizmos,
) {
    let Ok((body, vel)) = body_q.get_single() else {
        return;
    };
    let omega = vel.angvel;
    let intensity = omega.length() / 0.3;
    if intensity < 0.15 {
        return;
    }

    // Thrusters fire perpendicular to the pod's lever arm, against the rotation.
    for (pod_tf, pod) in &pods {
        let lever = pod_tf.translation() - body.translation();
        let dir = -omega.cross(lever).normalize_or_zero();
        if !dir.is_finite() || dir.length_squared() < 0.5 {
            continue;
        }
        let len = 2.5 * intensity.min(1.5);
        gizmos.ray(pod_tf.translation(), dir * len, Color::srgba(0.4, 0.9, 1.0, 0.8));
    }
}

/// Engine plume: a translucent flame cone under the nozzle, scaled by thrust.
pub fn engine_plume_system(
    mut gizmos: Gizmos,
    engine_q: Query<(&GlobalTransform, &EngineControlState), With<rocket::RocketEngine>>,
) {
    const MAX_THRUST: f32 = 250_000.0;
    for (engine_tf, state) in &engine_q {
        let intensity = (state.thrust / MAX_THRUST).clamp(0.0, 1.0);
        if intensity <= 0.0 {
            continue;
        }

        let tf = engine_tf.compute_transform();
        let nozzle = tf.translation - tf.up() * 5.0; // bell exit
        let len = 3.0 + 9.0 * intensity;
        let color = Color::srgba(1.0, 0.6, 0.2, 0.35 * intensity);

        gizmos.ray(nozzle, -tf.up() * len, color);
        gizmos.ray(nozzle, (-tf.up() * len).cross(Vec3::Y).normalize_or_zero() * len * 0.15, color);
        gizmos.sphere(nozzle - tf.up() * len * 0.5, 0.6 * intensity, color);
    }
}
