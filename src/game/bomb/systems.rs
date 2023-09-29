use crate::constants::{
    EXPLOSION_RADIUS,
    ENEMY_SIZE,
    EXPLOSION_VISIBILITY_TIME
};
use crate::events::EnemyKilled;
use crate::game::enemy::components::Enemy;

use super::components::*;
use super::resources::*;

use bevy::{
    prelude::*,
    //audio::{ Volume, VolumeLevel }
};

pub fn tick_bomb_timers(
    mut bomb_query: Query<&mut Bomb, With<Bomb>>,
    time: Res<Time>,
) {
    for mut bomb in &mut bomb_query {
        bomb.det_time -= time.delta().as_secs_f32();
    }
}

pub fn detonate_bomb(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bomb_counter: ResMut<PlacedBombCount>,
    bomb_query: Query<(Entity, &Transform, &Bomb)>
) {
    for (bomb_entity, bomb_transform, bomb) in bomb_query.iter() {
        if bomb.det_time <= 0.0 {

            let mut explosion_sprite = SpriteBundle {
                transform: Transform::from_translation(bomb_transform.translation),
                texture: asset_server.load("sprites/explosion_001.png"),
                ..default()
            };

            commands.entity(bomb_entity).despawn();

            explosion_sprite.transform.scale *= 5.0;

            commands.spawn((
                explosion_sprite,
                Explosion {
                    visible_time: EXPLOSION_VISIBILITY_TIME
                },
            ));

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

pub fn tick_cooldown_timer(
    mut cooldown_timer: ResMut<BombCooldownTimer>, 
    time: Res<Time>
) {
    cooldown_timer.timer.tick(time.delta());
}

pub fn remove_explosion(
    mut commands: Commands,
    explosion_query: Query<(Entity, &Explosion)>
) {
    for (explosion_entity, explosion) in explosion_query.iter() {
        if explosion.visible_time <= 0.0 {
            commands.entity(explosion_entity).despawn();
        }
    }
}

pub fn enemy_hit_explosion(
    mut commands: Commands,
    mut enemy_killed_event: EventWriter<EnemyKilled>,
    explosion_query: Query<&Transform, With<Explosion>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>
) {
    for exp_transform in explosion_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance: f32 = exp_transform
                .translation
                .distance(enemy_transform.translation);

            let enemy_radius: f32 = ENEMY_SIZE / 2.0;
            let explosion_radius: f32 = (EXPLOSION_RADIUS * 5.0) / 2.0;

            if distance < enemy_radius + explosion_radius {
                enemy_killed_event.send(EnemyKilled {  });
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}

pub fn despawn_bombs(
    mut commands: Commands,
    bomb_query: Query<Entity, With<Bomb>>,
    explosion_query: Query<Entity, With<Explosion>>
) {
    for bomb_entity in bomb_query.iter() {
        commands.entity(bomb_entity).despawn();
    }
    for explosion_entity in explosion_query.iter() {
        commands.entity(explosion_entity).despawn();
    }
}


pub fn insert_bombs(
    mut commands: Commands
) {
    commands.insert_resource(HeldBombCount::default());
    commands.insert_resource(PlacedBombCount::default());
}

pub fn remove_bombs(
    mut commands: Commands
) {
    commands.remove_resource::<HeldBombCount>();
    commands.remove_resource::<PlacedBombCount>();
}