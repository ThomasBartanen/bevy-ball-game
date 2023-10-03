pub mod constants;
pub mod events;
mod systems;
mod game;
mod main_menu;
pub mod extension_functions;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;


use bevy::{
    prelude::*,
    window::WindowMode
};

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgba(0.5, 0.5, 0.9, 0.1)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            mode: WindowMode::Windowed,
            ..default()
        }),
        ..default()
    }))

    .add_state::<AppState>()

    .add_plugins((
        GamePlugin,
        MainMenuPlugin
    ))

    .add_systems(
        Startup, 
        (
            //setup_audio,
            spawn_camera,
        ))

    .add_systems(
        Update, 
        (
            transition_to_game_state,
            transition_to_main_menu_state,
            exit_game,
            handle_game_over,
        ))

    .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver
}