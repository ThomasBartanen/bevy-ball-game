use crate::constants::BOMB_DET_RADIUS;
use crate::constants::EXPLOSION_VISIBILITY_TIME;

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
        //println!("Bomb timer {}", bomb.det_time);
    }
}

pub fn detonate_bomb(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bomb_counter: ResMut<BombCount>,
    bomb_query: Query<(Entity, &Transform, &Bomb)>
) {
    for (bomb_entity, bomb_transform, bomb) in bomb_query.iter() {
        if bomb.det_time <= 0.0 {
            println!("Detonating Bomb");
            commands.entity(bomb_entity).despawn();

            let mut explosion_sprite = SpriteBundle {
                transform: Transform::from_translation(bomb_transform.translation),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            };

            explosion_sprite.transform.scale *= BOMB_DET_RADIUS;

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
        //println!("Bomb timer {}", explosion.visible_time);
    }
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

pub fn despawn_bombs(
    mut commands: Commands,
    bomb_query: Query<Entity, With<Bomb>>,
    explosion_query: Query<Entity, With<Explosion>>
) {
    for bomb_entity in bomb_query.iter() {
        println!("Despawning Bomb");
        commands.entity(bomb_entity).despawn();
    }
    for explosion_entity in explosion_query.iter() {
        println!("Despawning Explosions");
        commands.entity(explosion_entity).despawn();
    }
}