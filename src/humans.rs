use bevy::prelude::*;

use crate::{Money, Player};

pub struct HumanPlugin;

impl Plugin for HumanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_human_parent)
            .add_systems(Update, (spawn_human, human_lifetime))
            .register_type::<Human>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]

pub struct Human {
    pub lifetime: Timer,
    pub speed: f32,
}

#[derive(Component)]
pub struct HumanParent;

fn spawn_human_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        HumanParent,
        Name::new("Human Parent"),
    ));
}

// fn human_movement(
//     mut characters: Query<(&mut Transform, &Human)>,
//     input: Res<Input<KeyCode>>,
//     time: Res<Time>,
// ) {
//     let mut rng = rand::thread_rng();
//     // let human_mover: HumanMover = rng.gen();

//     for (mut transform, human) in &mut characters {
//         let movement_amount = human.speed * time.delta_seconds();

//         if input.pressed(KeyCode::W) {
//             transform.translation.y += movement_amount;
//         }
//         if input.pressed(KeyCode::S) {
//             transform.translation.y -= movement_amount;
//         }
//         if input.pressed(KeyCode::D) {
//             transform.translation.x += movement_amount;
//         }
//         if input.pressed(KeyCode::A) {
//             transform.translation.x -= movement_amount;
//         }
//     }
// }

fn spawn_human(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<HumanParent>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();
    let parent = parent.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Human engaging!: ${:?}", money.0);
        let texture = asset_server.load("cats_human.png");
        commands.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: *player_transform,
                    ..default()
                },
                Human {
                    lifetime: Timer::from_seconds(10.0, TimerMode::Once),
                    speed: 50.0,
                },
                Name::new("Human"),
            ));
        });
    }
}

fn human_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut humans: Query<(Entity, &mut Human)>,
    parent: Query<Entity, With<HumanParent>>,
    mut money: ResMut<Money>,
) {
    let parent = parent.single();

    for (human_entity, mut human) in &mut humans {
        human.lifetime.tick(time.delta());

        if human.lifetime.finished() {
            money.0 += 15.0;

            commands.entity(parent).remove_children(&[human_entity]);
            commands.entity(human_entity).despawn();

            info!(
                "Fat humangy despawned you get money, Current Money: ${:?}",
                money.0
            )
        }
    }
}
