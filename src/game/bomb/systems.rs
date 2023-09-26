use crate::constants::BOMB_DET_RADIUS;
use crate::constants::EXPLOSION_VISIBILITY_TIME;
use crate::game::enemy::components::Enemy;

use super::components::*;
use super::resources::*;

use bevy::{
    prelude::*,
    //audio::{ Volume, VolumeLevel }
};
//use bevy_xpbd_2d::prelude::Position;
use bevy_xpbd_2d::prelude::{
    Collider,
    //RigidBody,
    Sensor,
    //SpatialQuery,
    //SpatialQueryFilter,
    Collision
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
    //spatial_query: SpatialQuery,
    //enemy_query: Query<&Enemy>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bomb_counter: ResMut<BombCount>,
    bomb_query: Query<(Entity, &Transform, &Bomb)>
) {
    for (bomb_entity, bomb_transform, bomb) in bomb_query.iter() {
        if bomb.det_time <= 0.0 {

            let mut explosion_sprite = SpriteBundle {
                transform: Transform::from_translation(bomb_transform.translation),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            };

            commands.entity(bomb_entity).despawn();

            explosion_sprite.transform.scale *= BOMB_DET_RADIUS;

            commands.spawn((
                explosion_sprite,
                Explosion {
                    visible_time: EXPLOSION_VISIBILITY_TIME
                },
                Collider::ball(BOMB_DET_RADIUS),
                Sensor
            ));

            bomb_counter.count -= 1;
            /*
            let intersections = spatial_query.shape_intersections(
                &Collider::ball(BOMB_DET_RADIUS),                // Shape
                bomb_transform.translation.truncate(),  // Shape position
                f32::default(),                         // Shape rotation
                SpatialQueryFilter::default(),            // Query filter
            );
        
            for entity in intersections.iter() {
                println!("Entity: {:?}", entity);
                if enemy_query.contains(*entity) {
                    commands.entity(*entity).despawn();
                }
            }
            */
        }
    }
}

pub fn handle_bomb_collisions(
    mut commands: Commands,
    enemy_query: Query<&Enemy>,
    explosion_query: Query<&Explosion>,
    mut collision_event_reader: EventReader<Collision>
) {
    for Collision(contact) in collision_event_reader.iter() {
        //println!("{:?} and {:?} are colliding", contact.entity1, contact.entity2);
        if (enemy_query.contains(contact.entity1)) && (explosion_query.contains(contact.entity2)) {
            commands.entity(contact.entity1).despawn();
        }
        
        if (enemy_query.contains(contact.entity2)) && (explosion_query.contains(contact.entity1)) {
            commands.entity(contact.entity2).despawn();
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
        commands.entity(bomb_entity).despawn();
    }
    for explosion_entity in explosion_query.iter() {
        commands.entity(explosion_entity).despawn();
    }
}