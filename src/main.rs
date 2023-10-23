use bevy::input::common_conditions::input_toggle_active;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_tilemap::prelude::*;
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
        .add_plugins(
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
        )
        .add_plugins((
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            TilemapPlugin,
            HumanPlugin,
            GameUI,
            CharacterPlugin,
        ))
        .insert_resource(Money(100.0))
        .insert_resource(Energy(100.0))
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

    commands.spawn(camera);
}

fn setup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 128, y: 128 };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

fn swap_texture_or_hide(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut TilemapTexture, &mut Visibility)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let texture_a = TilemapTexture::Single(asset_server.load("tiles.png"));
        let texture_b = TilemapTexture::Single(asset_server.load("tiles2.png"));
        for (mut tilemap_tex, _) in &mut query {
            if *tilemap_tex == texture_a {
                *tilemap_tex = texture_b.clone();
            } else {
                *tilemap_tex = texture_a.clone();
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::H) {
        for (_, mut visibility) in &mut query {
            *visibility = match *visibility {
                Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
            };
        }
    }
}
