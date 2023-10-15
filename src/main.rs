use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use character::CharacterPlugin;
use humans::HumanPlugin;

use ui::GameUI;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Money(pub f32);

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Energy(pub f32);

mod character;
mod humans;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cat Coin Stealer".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        ))
        .insert_resource(Money(100.0))
        .insert_resource(Energy(100.0))
        .register_type::<Money>()
        .register_type::<Energy>()
        .add_plugins((HumanPlugin, GameUI, CharacterPlugin))
        .add_systems(Startup, (setup_graphics, setup_physics))
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        ..default()
    });
}

// fn setup_phsyics(mut commands: Commands) {
//     commands
//         .spawn(Collider::cuboid(500.0, 50.0))
//         .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

//     commands
//         .spawn(RigidBody::Dynamic)
//         .insert(Collider::ball(50.0))
//         .insert(Restitution::coefficient(0.7))
//         .insert(TransformBundle::from(Transform::from_xyz(45.0, 45.0, 0.0)));
// }

pub fn setup_physics(mut commands: Commands) {
    let vertices = [Vec2::new(-640.0, 0.0), Vec2::new(640.0, 0.0)];

    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 640.0 + (1.0 / 2.0), 0.0),
                scale: Vec3::new(640.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::polyline(&vertices, 1.0));
}
