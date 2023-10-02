use crate::{
    constants::*,
    extension_functions::*
};
use super::components::*;
use super::resources::*;

use bevy::{
    prelude::*,
    window::PrimaryWindow,
    audio::{ Volume, VolumeLevel }
};
use rand::prelude::*;

pub fn spawn_enemies_at_start(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {

        let /*mut*/ enemy_sprite = SpriteBundle {
            transform: Transform::from_translation(get_random_screen_point(window).into()),
            texture: asset_server.load(get_random_enemy_path()),
            ..default()
        };

        let random_point: Vec3 = get_random_screen_point(window) + Vec3 { x: 0.0, y: 0.0, z: -0.1 };

        //enemy_sprite.transform.scale *= 0.5;

        commands.spawn((
            enemy_sprite,
            Enemy {
                direction: Vec2::new(random_point.x, random_point.y).normalize(),
            },
        ));
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    warning_query: Query<Entity, With<WarningCircle>>
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
    for warning_entity in warning_query.iter() {
        commands.entity(warning_entity).despawn();
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for(mut transform, enemy) in enemy_query.iter_mut() {
        let direction: Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn spin_enemies(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    time: Res<Time>
) {
    for mut transform in enemy_query.iter_mut() {
        transform.rotate_z(time.delta_seconds() * 5.0);
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let playback_settings: PlaybackSettings = PlaybackSettings {
        volume: Volume::Relative(VolumeLevel::new(0.1)),
        ..default()
    };

    let window: &Window = window_query.get_single().unwrap();

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0 + 0.5;
    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for(transform, mut enemy) in enemy_query.iter_mut(){
        let mut direction_changed: bool = false;

        let translation: Vec3 = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            //play_randomized_sound(&mut commands, &asset_server);
            
            let sound_effect_1: Handle<AudioSource> = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2: Handle<AudioSource> = asset_server.load("audio/pluck_002.ogg");

            let sound_effect: Handle<AudioSource> = if random::<f32>() > 0.5 {
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
    }    
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size: f32 = ENEMY_SIZE / 2.0;
    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut(){
        let mut translation: Vec3 = transform.translation;
        
        if translation.x < x_min {
            translation.x = x_min;
        }
        else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        }
        else if translation.y > y_max {
            translation.y = y_max;
        }
        transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, 
    time: Res<Time>
) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_counter: ResMut<EnemyCount>,
    warning_query: Query<(Entity, &Transform, &WarningCircle), With<WarningCircle>>
) {
    for (warning_entity, warning_transform, warning) in warning_query.iter() {
        if warning.spawn_time <= 0.0 {
            let spawn_point: Vec3 = warning_transform.translation;

            let enemy_sprite = SpriteBundle {
                transform: Transform::from_translation(spawn_point.into()),
                texture: asset_server.load(get_random_enemy_path()),
                ..default()
            };

            commands.spawn((
                enemy_sprite,
                Enemy {
                    direction: Vec2::new(spawn_point.x, spawn_point.y).normalize(),
                },
            ));

            commands.entity(warning_entity).despawn();
            enemy_counter.count += 1;
        }
    }
}

pub fn tick_warning_timer(
    mut warning_query: Query<&mut WarningCircle, With<WarningCircle>>,
    time: Res<Time>,
) {
    for mut warning in &mut warning_query {
        warning.spawn_time -= time.delta().as_secs_f32();
    }
}

pub fn spawn_warning_point(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    enemy_counter: Res<EnemyCount>,
) {    
    if enemy_counter.count >= MAX_ENEMIES { return; }
    if enemy_spawn_timer.timer.finished() {
        let window: &Window = window_query.get_single().unwrap();
        let random_point: Vec3 = get_random_screen_point(window) + Vec3 { x: 0.0, y: 0.0, z: -0.1 };

        let mut warning_sprite = SpriteBundle {
            transform: Transform::from_translation(random_point.into()),
            texture: asset_server.load(WARNING_CIRCLE_SPRITE_PATH),
            ..default()
        };

        warning_sprite.transform.scale *= 1.5;

        commands.spawn((
            warning_sprite,
            WarningCircle { 
                spawn_time: WARNING_TIME
            }
        ));
    }
}

pub fn get_random_enemy_path() -> &'static str {
    if randomize_choice() {
        return ENEMY_SPRITE_PATH
    }
    else {
        return ENEMY_SPRITE_PATH_2
    }
}
