use std::time::Duration;

use crate::constants::{
    BLOOD_SPRITE_PATH, ENEMY_SIZE, EXPLOSION_SIZE, EXPLOSION_SPRITE_PATH, EXPLOSION_SPRITE_SIZE,
    EXPLOSION_VISIBILITY_TIME, SCORCH_SPRITE_PATH,
};
use crate::events::EnemyKilled;
use crate::game::enemy::components::Enemy;

use super::components::*;
use super::resources::*;

use bevy::{
    prelude::*, //audio::{ Volume, VolumeLevel }
};
use bevy_tweening::lens::*;
use bevy_tweening::*;

pub fn tick_bomb_timers(mut bomb_query: Query<&mut Bomb, With<Bomb>>, time: Res<Time>) {
    for mut bomb in &mut bomb_query {
        bomb.det_time -= time.delta().as_secs_f32();
    }
}

pub fn detonate_bomb(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bomb_counter: ResMut<PlacedBombCount>,
    bomb_query: Query<(Entity, &Transform, &Bomb)>,
) {
    for (bomb_entity, bomb_transform, bomb) in bomb_query.iter() {
        if bomb.det_time <= 0.0 {
            let tween = Tween::new(
                EaseFunction::CircularOut,
                Duration::from_secs_f32(EXPLOSION_VISIBILITY_TIME),
                TransformScaleLens {
                    start: Vec3::ZERO,
                    end: Vec2::splat(EXPLOSION_SPRITE_SIZE).extend(bomb_transform.translation.z),
                },
            );

            let explosion_sprite = SpriteBundle {
                transform: Transform::from_translation(bomb_transform.translation),
                texture: asset_server.load(EXPLOSION_SPRITE_PATH),
                ..default()
            };
            let mut scorch_sprite = SpriteBundle {
                transform: Transform::from_translation(
                    bomb_transform.translation
                        + Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: -0.3,
                        },
                ),
                texture: asset_server.load(SCORCH_SPRITE_PATH),
                ..default()
            };

            commands.entity(bomb_entity).despawn();

            //explosion_sprite.transform.scale *= 5.0;
            commands
                .spawn((
                    explosion_sprite,
                    Explosion {
                        visible_time: EXPLOSION_VISIBILITY_TIME,
                    },
                ))
                .insert(Animator::new(tween));

            scorch_sprite.transform.scale *= 3.5;
            commands.spawn((scorch_sprite, ScorchMark {}));

            bomb_counter.count -= 1;
        }
    }
}

pub fn tick_explosion_timers(
    mut explosion_query: Query<&mut Explosion, With<Explosion>>,
    time: Res<Time>,
) {
    for mut explosion in &mut explosion_query {
        explosion.visible_time -= time.delta().as_secs_f32();
    }
}

pub fn tick_cooldown_timer(mut cooldown_timer: ResMut<BombCooldownTimer>, time: Res<Time>) {
    cooldown_timer.timer.tick(time.delta());
}

pub fn remove_explosion(mut commands: Commands, explosion_query: Query<(Entity, &Explosion)>) {
    for (explosion_entity, explosion) in explosion_query.iter() {
        if explosion.visible_time <= 0.0 {
            commands.entity(explosion_entity).despawn();
        }
    }
}

pub fn enemy_hit_explosion(
    mut commands: Commands,
    mut gizmos: Gizmos,
    asset_server: Res<AssetServer>,
    mut enemy_killed_event: EventWriter<EnemyKilled>,
    explosion_query: Query<&Transform, With<Explosion>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for exp_transform in explosion_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance: f32 = exp_transform
                .translation
                .distance(enemy_transform.translation);

            let enemy_radius: f32 = ENEMY_SIZE / 2.0;
            let explosion_radius: f32 = EXPLOSION_SIZE;
            gizmos.circle_2d(
                exp_transform.translation.truncate(),
                EXPLOSION_SIZE,
                Color::BLACK,
            );

            if distance < enemy_radius + explosion_radius {
                enemy_killed_event.send(EnemyKilled {
                    entity: enemy_entity,
                });
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(
                            enemy_transform.translation
                                + Vec3 {
                                    x: 0.0,
                                    y: 0.0,
                                    z: -0.2,
                                },
                        ),
                        texture: asset_server.load(BLOOD_SPRITE_PATH),
                        ..default()
                    },
                    BloodPool {},
                ));
            }
        }
    }
}

pub fn despawn_bombs(
    mut commands: Commands,
    bomb_query: Query<Entity, With<Bomb>>,
    explosion_query: Query<Entity, With<Explosion>>,
    blood_query: Query<Entity, With<BloodPool>>,
    scorch_query: Query<Entity, With<ScorchMark>>,
) {
    for bomb_entity in bomb_query.iter() {
        commands.entity(bomb_entity).despawn();
    }
    for explosion_entity in explosion_query.iter() {
        commands.entity(explosion_entity).despawn();
    }
    for blood_entity in blood_query.iter() {
        commands.entity(blood_entity).despawn();
    }
    for scorch_entity in scorch_query.iter() {
        commands.entity(scorch_entity).despawn();
    }
}

pub fn insert_bombs(mut commands: Commands) {
    commands.insert_resource(HeldBombCount::default());
    commands.insert_resource(PlacedBombCount::default());
}

pub fn remove_bombs(mut commands: Commands) {
    commands.remove_resource::<HeldBombCount>();
    commands.remove_resource::<PlacedBombCount>();
}
