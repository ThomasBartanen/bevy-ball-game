use bevy::ecs::{entity::Entity, event::Event};

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
    pub kills: u32,
}

#[derive(Event)]
pub struct EnemyKilled {
    pub entity: Entity,
}
