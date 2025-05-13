use crate::prelude::*;
use bevy::{color::palettes::css::GOLD, prelude::*};
use bevy_rapier3d::prelude::AdditionalMassProperties;

#[derive(Component)]
enum MetricKind {
    Altitude,
    Velocity,
    Mass,
    Fuel,
    Thrust,
    Tilt,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_hud)
        .add_systems(Update, update_hud);
}

fn setup_hud(mut commands: Commands, assets: Res<AssetServer>) {
    let font_bold = assets.load("fonts/FiraSans-Bold.ttf");
    let font_mono = assets.load("fonts/FiraMono-Medium.ttf");

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                padding: UiRect::all(Val::Px(8.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.45)),
        ))
        .with_children(|root| {
            add_line(root, &font_bold, &font_mono, MetricKind::Altitude, "Alt:");
            add_line(root, &font_bold, &font_mono, MetricKind::Velocity, "Vel:");
            add_line(root, &font_bold, &font_mono, MetricKind::Mass, "Mass:");
            add_line(root, &font_bold, &font_mono, MetricKind::Fuel, "Fuel:");
            add_line(root, &font_bold, &font_mono, MetricKind::Thrust, "Thrust:");
            add_line(root, &font_bold, &font_mono, MetricKind::Tilt, "Tilt:");
        });
}

fn add_line(
    parent: &mut ChildBuilder,
    label_font: &Handle<Font>,
    value_font: &Handle<Font>,
    kind: MetricKind,
    label: &str,
) {
    parent
        .spawn(
            (Node {
                flex_direction: FlexDirection::Row,
                ..default()
            }),
        )
        .with_children(|row| {
            // Label
            row.spawn((
                Text::new(format!("{label} ")),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::default(),
            ));
            // Value
            row.spawn((
                Text::default(),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(GOLD.into()),
                TextLayout::default(),
                kind,
            ));
        });
}

fn update_hud(
    rocket_q: Query<
        (
            &GlobalTransform,
            &Velocity,
            &AdditionalMassProperties,
            &rocket::RocketConfig,
        ),
        With<rocket::RocketBody>,
    >,
    engine_q: Query<&rocket::EngineControlState, With<rocket::RocketEngine>>,
    mut spans: Query<(&mut Text, &MetricKind)>,
) {
    let Ok((tf, vel, extra_mass, cfg)) = rocket_q.get_single() else {
        return;
    };

    let altitude = tf.translation().y;
    let velocity = vel.linvel.length();

    let fuel_mass = match *extra_mass {
        AdditionalMassProperties::Mass(m) => m.max(0.0),
        _ => 0.0,
    };
    let total_mass = cfg.0.m_dry as f32 + fuel_mass;
    let thrust: f32 = engine_q.iter().map(|e| e.thrust).sum();
    let tilt = tf.up().angle_between(Vec3::Y).to_degrees();

    for (mut text, kind) in &mut spans {
        text.0 = match kind {
            MetricKind::Altitude => format!("{altitude:>7.0} m"),
            MetricKind::Velocity => format!("{velocity:>6.1} m/s"),
            MetricKind::Mass => format!("{total_mass:>6.0} kg"),
            MetricKind::Fuel => format!("{fuel_mass:>6.0} kg"),
            MetricKind::Thrust => format!("{thrust:>6.0} N"),
            MetricKind::Tilt => format!("{tilt:>5.1} degs"),
        };
    }
}
