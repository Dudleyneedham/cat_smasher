use bevy::input::common_conditions::input_toggle_active;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
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
#[derive(Default, Bundle, LdtkEntity)]
pub struct MyBundle {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

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
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            PanCamPlugin::default(),
        ))
        .add_plugins((
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            TilemapPlugin,
            LdtkPlugin,
            HumanPlugin,
            GameUI,
            CharacterPlugin,
        ))
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            ..Default::default()
        })
        .insert_resource(Money(100.0))
        .insert_resource(Energy(100.0))
        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .register_type::<Money>()
        .register_type::<Energy>()
        .add_systems(Startup, (setup_camera, setup_tilemap, swap_texture_or_hide))
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 640.0,
        min_height: 480.0,
    };

    commands.spawn(camera).insert(PanCam::default());
}
