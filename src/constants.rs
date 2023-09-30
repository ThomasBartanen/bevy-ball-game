// === Player ===
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // player SPRITE size in pixels

// === Enemies ===
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const MAX_ENEMIES: usize = 20;
pub const ENEMY_SPEED: f32 = 300.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPAWN_TIME: f32 = 4.0;
pub const WARNING_TIME: f32 = 1.0;
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

// === Currency ===
pub const BOMB_COST: f32 = 10.0;
pub const CURRENCY_PER_KILL: f32 = 5.0;

// === Sprites ===
pub const ENEMY_SPRITE_PATH: &str = "sprites/ball_red_large.png";
pub const PLAYER_SPRITE_PATH: &str = "sprites/ball_blue_large.png";
pub const BOMB_SPRITE_PATH: &str = "sprites/bomb_001.png";
pub const EXPLOSION_SPRITE_PATH: &str = "sprites/explosion_001.png";
pub const BLOOD_SPRITE_PATH: &str = "sprites/blood_pool_002.png";
pub const SCORCH_SPRITE_PATH: &str = "sprites/scorch_mark_003.png";
pub const STAR_SPRITE_PATH: &str = "sprites/star.png";
pub const WARNING_CIRCLE_SPRITE_PATH: &str = "sprites/warning_circle_001.png";