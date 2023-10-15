use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_rapier2d::prelude::*;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]

pub struct Character {
    #[inspector(min = 0.0)]
    pub speed: f32,
    size: f32,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character)
            .add_systems(Update, (update_system, read_result_system))
            .register_type::<Character>();
    }
}

fn spawn_character(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let window: &Window = window_query.get_single().unwrap();
    rapier_config.gravity = Vec2::ZERO;
    let sprite_size = 64.0;
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.1),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                    ..default()
                },
                texture: asset_server.load("fat_cat.png"),
                ..default()
            },
            Character {
                speed: 100.0,
                size: 64.0,
            },
            RigidBody::Dynamic,
            Velocity::zero(),
            Name::new("Fat Cat Character"),
        ))
        .insert(KinematicCharacterController::default());
}
fn update_system(mut controllers: Query<&mut KinematicCharacterController>) {
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(Vec2::new(1.0, -0.5));
    }
}
fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (entity, output) in controllers.iter() {
        println!(
            "Entity {:?} moved by {:?} and touches the ground: {:?}",
            entity, output.effective_translation, output.grounded
        );
    }
}
// fn character_movement(
//     mut characters: Query<(&mut Velocity, &Character)>,
//     input: Res<Input<KeyCode>>,
//     time: Res<Time>,
// ) {
//     if let Ok((mut transform, character)) = characters.get_single_mut() {
//         let mut direction = Vec2::ZERO;

//         let movement_amount = character.speed * time.delta_seconds();

//         if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
//             direction += Vec2::new(0.0, 1.0);
//         }
//         if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
//             direction += Vec2::new(0.0, -1.0);
//         }
//         if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
//             direction += Vec2::new(1.0, 0.0);
//         }
//         if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
//             direction += Vec2::new(-1.0, 0.0);
//         }
//         if direction.length() > 0.0 {
//             direction = direction.normalize();
//         }

//         transform.linvel += direction * movement_amount
//     }
// }

// pub fn character_confined_movement(
//     mut character_query: Query<(&mut Velocity, &Character)>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
// ) {
//     if let Ok((mut character_transformation, character)) = character_query.get_single_mut() {
//         let window = window_query.get_single().unwrap();

//         let half_character_size = character.speed / 2.0;
//         let x_min = 0.0 + half_character_size;
//         let x_max = window.width() - half_character_size;
//         let y_min = 0.0 + half_character_size;
//         let y_max = window.height() - half_character_size;

//         let mut translation = character_transformation.linvel;

//         if translation.x < x_min {
//             translation.x = x_min
//         } else if translation.x > x_max {
//             translation.x = x_max
//         }

//         if translation.y < y_min {
//             translation.y = y_min
//         } else if translation.y > y_max {
//             translation.y = y_max
//         }

//         character_transformation.linvel = translation;
//     }
// }
