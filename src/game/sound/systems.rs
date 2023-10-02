use crate::{constants::{
    ENEMY_BOUNCE_SOUND_PATH, 
    ENEMY_BOUNCE_SOUND_PATH_2, 
    MAX_BOUNCE_SOUNDS, 
    STAR_COLLECTED_SOUND_PATH
}, extension_functions::randomize_choice};

use super::{
    resources::*, 
    components::*
};

use rand::prelude::*;
use bevy::prelude::*;

pub fn play_queued_sfx(
    mut commands: Commands,
    mut sfx_resource: ResMut<SFXQueue>
) {
    if sfx_resource.sound_queue.is_empty() {return;}
    let sound = sfx_resource.sound_queue.pop();
    commands.spawn(sound.unwrap());
}

pub fn queue_collect_sound(
    asset_server: Res<AssetServer>,
    sfx_settings: Res<SFXValues>, 
    mut sfx_resource: ResMut<SFXQueue>
) {
    if sfx_resource.collects_requested <= 0 {return;}
    sfx_resource.sound_queue.push(AudioBundle {
        source: asset_server.load(STAR_COLLECTED_SOUND_PATH),
        settings: sfx_settings.settings,
        ..default()
    });
    sfx_resource.collects_requested -= 1;
}

pub fn queue_random_bounce_sound(
    asset_server: Res<AssetServer>,
    sfx_settings: Res<SFXValues>, 
    mut sfx_resource: ResMut<SFXQueue>
) {
    if sfx_resource.bounces_requested <= 0 {return;}
    if sfx_resource.bounces_requested < MAX_BOUNCE_SOUNDS {
        let path: &str;
        if randomize_choice() {
            path = ENEMY_BOUNCE_SOUND_PATH;
        } else {
            path = ENEMY_BOUNCE_SOUND_PATH_2;
        }    
        sfx_resource.sound_queue.push(AudioBundle {
            source: asset_server.load(path),
            settings: sfx_settings.settings,
            ..default()
        });
    }
    sfx_resource.bounces_requested -= 1;
}

fn pause_music(
    keyboard_input: Res<Input<KeyCode>>, 
    music_controller: Query<&AudioSink, With<MusicSource>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(sink) = music_controller.get_single() {
            sink.toggle();
        }
    }
}

fn set_music_volume(
    keyboard_input: Res<Input<KeyCode>>, 
    music_controller: Query<&AudioSink, With<MusicSource>>
) {
    if let Ok(sink) = music_controller.get_single() {
        if keyboard_input.just_pressed(KeyCode::Plus) {
            sink.set_volume(sink.volume() + 0.1);
        } else if keyboard_input.just_pressed(KeyCode::Minus) {
            sink.set_volume(sink.volume() - 0.1);
        }
    }
}

pub fn initialize_music_entity(
    mut commands: Commands
) {
    commands.spawn(MusicSource{});
}

pub fn despawn_music_entity(
    mut commands: Commands,
    music_source_query: Query<Entity, With<MusicSource>>
) {
    let source_entity = music_source_query.get_single().unwrap(); 
    commands.entity(source_entity).despawn();
}
