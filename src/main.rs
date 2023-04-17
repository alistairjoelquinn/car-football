use bevy::prelude::*;
use bevy::window::PresentMode;
use std::collections::HashMap;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use car_football::ball::BallPlugin;
use car_football::game::components::{AppState, GameAsset};
use car_football::game::systems::{check_assets, spawn_camera};
use car_football::player::PlayerPlugin;

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut game_assets: ResMut<GameAsset>,
) {
    game_assets.image_handles = HashMap::from([
        (
            "car_one_handle".into(),
            asset_server.load("sprites/car_one.png"),
        ),
        (
            "car_two_handle".into(),
            asset_server.load("sprites/car_two.png"),
        ),
        (
            "ball_handle".into(),
            asset_server.load("sprites/football.png"),
        ),
    ]);
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::DARK_GREEN))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Car Football".into(),
                resolution: (1200., 700.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .insert_resource(GameAsset::default())
        .add_startup_system(spawn_camera)
        .add_systems((
            load_assets.in_schedule(OnEnter(AppState::Loading)),
            check_assets.run_if(in_state(AppState::Loading)),
        ))
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .run();
}
