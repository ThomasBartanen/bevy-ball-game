use bevy::prelude::KeyCode;

// === Controls ===
pub const CLOSE_APPLICATION_KEY: KeyCode = KeyCode::Escape;
pub const PAUSE_GAME_KEY: KeyCode = KeyCode::Return;
pub const MAIN_MENU_KEY: KeyCode = KeyCode::M;
pub const DROP_BOMB_KEY: KeyCode = KeyCode::Space;
pub const BUY_BOMB_KEY: KeyCode = KeyCode::Q;
pub const MOVE_LEFT_KEY: KeyCode = KeyCode::A;
pub const MOVE_RIGHT_KEY: KeyCode = KeyCode::D;
pub const MOVE_UP_KEY: KeyCode = KeyCode::W;
pub const MOVE_DOWN_KEY: KeyCode = KeyCode::S;
pub const ALT_MOVE_LEFT_KEY: KeyCode = KeyCode::Left;
pub const ALT_MOVE_RIGHT_KEY: KeyCode = KeyCode::Right;
pub const ALT_MOVE_UP_KEY: KeyCode = KeyCode::Up;
pub const ALT_MOVE_DOWN_KEY: KeyCode = KeyCode::Down;

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

// === Sprite Paths ===
pub const ENEMY_SPRITE_PATH: &str = "sprites/spinning_gem_001.png";
pub const ENEMY_SPRITE_PATH_2: &str = "sprites/spinning_gem_003.png";
pub const PLAYER_SPRITE_PATH: &str = "sprites/ball_blue_large.png";
pub const BOMB_SPRITE_PATH: &str = "sprites/bomb_001.png";
pub const EXPLOSION_SPRITE_PATH: &str = "sprites/explosion_001.png";
pub const BLOOD_SPRITE_PATH: &str = "sprites/blood_pool_002.png";
pub const SCORCH_SPRITE_PATH: &str = "sprites/scorch_mark_003.png";
pub const STAR_SPRITE_PATH: &str = "sprites/star.png";
pub const WARNING_CIRCLE_SPRITE_PATH: &str = "sprites/warning_circle_001.png";

// === Sound Paths ===
pub const MUSIC_PATH: &str = "";
pub const MUSIC_PATH_2: &str = "";
pub const ENEMY_BOUNCE_SOUND_PATH: &str = "audio/pluck_001.ogg";
pub const ENEMY_BOUNCE_SOUND_PATH_2: &str = "audio/pluck_002.ogg";
pub const PLAYER_KILLED_SOUND_PATH: &str = "audio/explosionCrunch_000.ogg";
pub const STAR_COLLECTED_SOUND_PATH: &str = "audio/laserLarge_000.ogg";

// === Sounds ===
pub const MAX_BOUNCE_SOUNDS: u16 = 5;