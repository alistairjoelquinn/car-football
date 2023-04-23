pub mod resources;
pub mod systems;

use bevy::prelude::*;
use bevy::window::PresentMode;

use super::game::resources::*;
use super::game::systems::*;
use super::user_interface::systems::*;

use resources::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_startup_system(spawn_camera)
            .insert_resource(GameAsset::default())
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
            // loading systems
            .add_systems((
                spawn_loading_screen.in_schedule(OnEnter(AppState::Loading)),
                remove_loading_screen.in_schedule(OnExit(AppState::Loading)),
                load_assets.in_schedule(OnEnter(AppState::Loading)),
                check_assets.run_if(in_state(AppState::Loading)),
            ))
            // menu systems
            .add_systems(())
            // game systems
            .add_systems(())
            // game over systems
            .add_systems(());
    }
}
