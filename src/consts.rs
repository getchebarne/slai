// Entity
pub const MAX_MOVE_HISTORY: usize = 64;

// Combat
pub const MAX_SIZE_HAND: usize = 10;
pub const CARDS_DRAWN_PER_TURN: u8 = 5;
pub const MAX_COMBAT_CARD_REWARD: usize = 3;
pub const FACTOR_WEAK: f32 = 0.75;
pub const FACTOR_VULN: f32 = 1.50;
pub const FACTOR_FRAIL: f32 = 0.75;
pub const MODE_SHIFT_INCREASE_PER_CYCLE: i16 = 10;
pub const MAX_MONSTERS: usize = 5;
pub const MAX_BLOCK: u16 = 999;

// Card rewards
pub const CARD_REWARD_ROLL_OFFSET_BASE: i8 = 5;
pub const CARD_REWARD_ROLL_OFFSET_MIN: i8 = -40;
pub const CHANCE_RARE: i32 = 3;
pub const CHANCE_UNCOMMON: i32 = 40;

// Map
pub const MAP_HEIGHT: usize = 15;
pub const MAP_WIDTH: usize = 7;
pub const PATH_DENSITY: usize = 6;
pub const ANCESTOR_GAP_MIN: usize = 3;
pub const FACTOR_NUM_REST_SITE: f32 = 0.25;

// Rest site
pub const REST_SITE_HEAL_FACTOR: f32 = 0.30;
