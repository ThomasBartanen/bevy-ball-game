// === Player ===
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // player SPRITE size in pixels

// === Enemies ===
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const MAX_ENEMIES: usize = 15;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPAWN_TIME: f32 = 10.0;

// === Stars ===
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SCORE: u32 = 1;
pub const STAR_SPAWN_TIME: f32 = 1.0;

// === Bombs ===
pub const BOMB_COST: u32 = 2;
pub const BOMB_DET_TIME: f32 = 0.75;
pub const BOMB_DET_RADIUS: f32 = 5.0;
pub const EXPLOSION_VISIBILITY_TIME: f32 = 0.2;