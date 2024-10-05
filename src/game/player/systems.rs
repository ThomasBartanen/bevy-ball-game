use crate::{
    constants::*,
    events::GameOver,
    game::bomb::{components::Bomb, resources::*},
    game::player::components::*,
    game::score::resources::*,
    game::star::components::Star,
    game::{enemy::components::Enemy, sound::resources::SFXQueue},
};

use bevy::{audio::Volume, prelude::*, window::PrimaryWindow};

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
            texture: asset_server.load(PLAYER_SPRITE_PATH),
            ..default()
        },
        Player {},
    ));
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction: Vec3 = Vec3::ZERO;

        if keyboard_input.pressed(MOVE_LEFT_KEY) || keyboard_input.pressed(ALT_MOVE_LEFT_KEY) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(MOVE_RIGHT_KEY) || keyboard_input.pressed(ALT_MOVE_RIGHT_KEY) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(MOVE_UP_KEY) || keyboard_input.pressed(ALT_MOVE_UP_KEY) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(MOVE_DOWN_KEY) || keyboard_input.pressed(ALT_MOVE_DOWN_KEY) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        direction = direction.normalize_or_zero();

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movment(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window: &Window = window_query.get_single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE / 2.0;
        let x_min: f32 = 0.0 + half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_min: f32 = 0.0 + half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        let mut translation: Vec3 = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        player_transform.translation = translation;
    }
}

pub fn player_drop_bomb(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    mut placed_bomb_counter: ResMut<PlacedBombCount>,
    mut held_bomb_counter: ResMut<HeldBombCount>,
    mut cooldown: ResMut<BombCooldownTimer>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if held_bomb_counter.count == 0 || !cooldown.timer.finished() {
            return;
        }
        if keyboard_input.pressed(DROP_BOMB_KEY) {
            let /* mut */ bomb_sprite = SpriteBundle {
                transform: Transform::from_translation(player_transform.translation + Vec3 { x: 0.0, y: 0.0, z: -0.2 }),
                texture: asset_server.load(BOMB_SPRITE_PATH),
                ..default()
            };

            //bomb_sprite.transform.scale *= 0.5;

            commands.spawn((
                bomb_sprite,
                Bomb {
                    det_time: BOMB_DET_TIME,
                },
            ));
            cooldown.timer = Timer::from_seconds(BOMB_COOLDOWN_TIME, TimerMode::Once);
            held_bomb_counter.count -= 1;
            placed_bomb_counter.count += 1;
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    kills: Res<Kills>,
) {
    let playback_settings: PlaybackSettings = PlaybackSettings {
        volume: Volume::new(0.1),
        ..default()
    };

    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance: f32 = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius: f32 = PLAYER_SIZE / 2.0;
            let enemy_radius: f32 = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                println!("Game Over Suckaaaaa");
                let sound_effect: Handle<AudioSource> =
                    asset_server.load("audio/explosionCrunch_000.ogg");

                commands.spawn(AudioBundle {
                    source: sound_effect,
                    settings: playback_settings,
                });

                commands.entity(player_entity).despawn();
                game_over_event.send(GameOver {
                    score: score.value,
                    kills: kills.value,
                });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    mut score: ResMut<Score>,
    mut sfx_resource: ResMut<SFXQueue>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance: f32 = player_transform
                .translation
                .distance(star_transform.translation);

            let player_radius: f32 = PLAYER_SIZE / 2.0;
            let star_radius: f32 = STAR_SIZE / 2.0;

            if distance < player_radius + star_radius {
                sfx_resource.collects_requested += 1;

                score.value += STAR_SCORE;

                commands.entity(star_entity).despawn();
            }
        }
    }
}
