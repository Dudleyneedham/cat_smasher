use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::random;

use crate::Money;

pub struct HumanPlugin;

impl Plugin for HumanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_human)
            .add_systems(
                Update,
                (
                    human_lifetime,
                    human_movement,
                    update_human_direction,
                    confine_human_movement,
                ),
            )
            .register_type::<Human>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]

pub struct Human {
    pub lifetime: Timer,
    pub speed: f32,
    pub size: f32,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct HumanParent;

fn human_movement(mut human_query: Query<(&mut Velocity, &Human)>, time: Res<Time>) {
    for (mut transform, human) in &mut human_query.iter_mut() {
        let movement_amount = human.speed * time.delta_seconds();
        let direction = Vec2::new(human.direction.x, human.direction.y);
        transform.linvel += direction * movement_amount
    }
}

fn spawn_human(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let window: &Window = window_query.get_single().unwrap();

    rapier_config.gravity = Vec2::ZERO;
    let sprite_size = 64.0;
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("cats_human.png"),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.1),
            sprite: Sprite {
                custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                ..default()
            },
            ..default()
        },
        Human {
            lifetime: Timer::from_seconds(1000.0, TimerMode::Once),
            speed: 100.0,
            direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            size: 64.0,
        },
        RigidBody::Dynamic,
        Velocity::zero(),
        Name::new("Human"),
    ));
}

fn human_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut humans: Query<(Entity, &mut Human)>,
    mut money: ResMut<Money>,
) {
    for (human_entity, mut human) in &mut humans {
        human.lifetime.tick(time.delta());

        if human.lifetime.finished() {
            money.0 += 15.0;

            commands.entity(human_entity).despawn();

            info!(
                "Fat humangy despawned you get money, Current Money: ${:?}",
                money.0
            )
        }
    }
}

pub fn update_human_direction(
    mut human_query: Query<(&mut Velocity, &mut Human)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (transform, mut human) in human_query.iter_mut() {
        let half_human_size = human.speed / 2.0;
        let x_min = 0.0 + half_human_size;
        let x_max = window.width() - half_human_size;
        let y_min = 0.0 + half_human_size;
        let y_max = window.height() - half_human_size;

        let translation = transform.linvel;
        if translation.x < x_min || translation.x > x_max {
            human.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            human.direction.y *= -1.0;
        }
    }
}

pub fn confine_human_movement(
    mut human_query: Query<(&mut Velocity, &Human)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (mut human_transform, human) in human_query.iter_mut() {
        let half_human_size = human.speed / 2.0;
        let x_min = 0.0 + half_human_size;
        let x_max = window.width() - half_human_size;
        let y_min = 0.0 + half_human_size;
        let y_max = window.height() - half_human_size;

        let mut translation = human_transform.linvel;

        if translation.x < x_min {
            translation.x = x_min
        } else if translation.x > x_max {
            translation.x = x_max
        }

        if translation.y < y_min {
            translation.y = y_min
        } else if translation.y > y_max {
            translation.y = y_max
        }

        human_transform.linvel = translation;
    }
}
