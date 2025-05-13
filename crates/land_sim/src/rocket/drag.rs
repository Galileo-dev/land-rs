use crate::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

const RHO: f32 = 1.0;

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct DragForce(pub Vec3);

pub fn aerodynamic_drag_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Velocity,
            &rocket::RocketConfig,
            &mut ExternalForce,
            Option<&mut DragForce>,
        ),
        With<rocket::RocketBody>,
    >,
) {
    for (entity, velocity, config, mut external_force, drag_force) in &mut query {
        let linvel = velocity.linvel;
        let speed = linvel.length();

        if speed == 0.0 {
            continue;
        }

        let s_d = config.0.s_d as f32;
        let c_d = config.0.c_d as f32;

        let drag_magnitude = 0.5 * RHO * c_d * s_d * speed * speed;
        let drag_vector = -linvel.normalize() * drag_magnitude;

        external_force.force = drag_vector;

        match drag_force {
            Some(mut drag_force_comp) => {
                drag_force_comp.0 = drag_vector;
            }
            None => {
                commands.entity(entity).insert(DragForce(drag_vector));
            }
        }
    }
}
