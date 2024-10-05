use crate::{
    constants::{CLOSE_APPLICATION_KEY, MAIN_MENU_KEY},
    events::*,
    game::SimulationState,
    AppState,
};

use bevy::{
    app::AppExit,
    prelude::*,
    window::PrimaryWindow, //, core_pipeline::{tonemapping::Tonemapping, bloom::{BloomSettings, BloomPrefilterSettings, BloomCompositeMode}}
};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
            camera: Camera {
                hdr: true,
                ..default()
            },
            //tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        }, /*,
           BloomSettings{
               intensity: 0.15,
               composite_mode: BloomCompositeMode::Additive,
               ..default()
           },*/
    );
}

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(MAIN_MENU_KEY) && app_state.get() != &AppState::Game {
        next_app_state.set(AppState::Game);
        println!("Entered AppState::Game");
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(MAIN_MENU_KEY) && app_state.get() != &AppState::MainMenu {
        next_app_state.set(AppState::MainMenu);
        next_sim_state.set(SimulationState::Paused);
        println!("Entered AppState::MainMenu");
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(CLOSE_APPLICATION_KEY) {
        app_exit_events.send(AppExit::Success);
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.read() {
        println!("Your final score is: {}", event.score);
        next_app_state.set(AppState::MainMenu);
    }
}

/*      Alternative I was trying
pub fn setup_audio(

) {

}
pub fn play_randomized_sound(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) {
    let playback_settings = PlaybackSettings {
        volume: Volume::Relative(VolumeLevel::new(0.1)),
        ..default()
    };

    let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
    let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

    let sound_effect = if random::<f32>() > 0.5 {
        sound_effect_1
    } else {
        sound_effect_2
    };

    commands.spawn(AudioBundle {
        source: sound_effect,
        settings: playback_settings,
        ..default()
    });
}
*/
