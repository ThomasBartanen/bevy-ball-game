// === Player ===
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // player SPRITE size in pixels

// === Enemies ===
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const MAX_ENEMIES: usize = 20;
pub const ENEMY_SPEED: f32 = 300.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;
pub const ENEMY_KILL_SCORE: u32 = 2;

// === Stars ===
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SCORE: u32 = 1;
pub const STAR_SPAWN_TIME: f32 = 1.0;

// === Bombs ===
pub const BOMB_DET_TIME: f32 = 0.75;
pub const BOMB_COOLDOWN_TIME: f32 = 1.2;

// === Explosions ===
pub const EXPLOSION_RADIUS: f32 = 64.0;
pub const EXPLOSION_VISIBILITY_TIME: f32 = 0.2;

// === CURRENCY ===
pub const BOMB_COST: f32 = 10.0;
pub const CURRENCY_PER_KILL: f32 = 5.0;