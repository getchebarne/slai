pub const MAX_MOVE_HISTORY: usize = 64;

// Per-Card effect array cap; bump if any Card legitimately exceeds 8
pub const MAX_EFFECTS_PER_CARD: usize = 6;

// Per-move effect array cap; sized for Book of Stabbing's Multi-Stab growth
pub const MAX_EFFECTS_PER_MOVE: usize = 20;

// Per-event-option effect array cap; Mushrooms' Stomp is the 5-effect maximum
pub const MAX_EFFECTS_PER_EVENT_OPTION: usize = 5;

pub const STARTING_GOLD: u16 = 99;
pub const MAX_GOLD: u16 = 9999;

// Silent starting vitals: base max HP, A14 penalty, A6+ current-HP fraction
pub const SILENT_HP_MAX_BASE: u16 = 70;
pub const ASCENSION_HP_MAX_CUT_LEVEL: u8 = 14;
pub const ASCENSION_HP_START_CUT_LEVEL: u8 = 6;
pub const SILENT_HP_MAX_A14_DELTA: u16 = 4;
pub const HEALTH_START_A6_NUMER: u16 = 9;
pub const HEALTH_START_A6_DENOM: u16 = 10;
pub const GOLD_MONSTER_MIN: u16 = 10;
pub const GOLD_MONSTER_MAX: u16 = 20;
pub const GOLD_ELITE_MIN: u16 = 25;
pub const GOLD_ELITE_MAX: u16 = 35;
pub const GOLD_BOSS_MIN: u16 = 95;
pub const GOLD_BOSS_MAX: u16 = 105;

// Widest reward-gold offer: boss roll + Golden Idol's 25% (rounded half-up)
pub const REWARD_GOLD_MAX: u16 = GOLD_BOSS_MAX + (GOLD_BOSS_MAX + 2) / 4;
pub const BOSS_RELIC_REWARD_COUNT: usize = 3;
pub const LIBRARY_CARD_COUNT: usize = 20;

// Largest single Card-reward roll (The Library's 20)
pub const MAX_CARD_REWARD_ROLL: usize = 20;
const _: () = assert!(LIBRARY_CARD_COUNT <= MAX_CARD_REWARD_ROLL);

// Combat
pub const MAX_SIZE_HAND: usize = 10;
pub const MAX_SIZE_DECK: usize = 99;
pub const MAX_ENTITIES: usize = 1024;
pub const CARDS_DRAWN_PER_TURN: u16 = 5;
pub const ENERGY_MAX_BASE: u8 = 3;

// Capacity: base 3 plus Question Card's +1; the roll base is CARD_REWARD_BASE_COUNT
pub const MAX_COMBAT_CARD_REWARD: usize = 4;
pub const CARD_REWARD_BASE_COUNT: usize = 3;

// Most bundles a Reward frame can hold: Orrery's five
pub const ORRERY_BUNDLE_COUNT: usize = 5;

// Cauldron's brew, staged as Potion rewards
pub const CAULDRON_POTION_COUNT: usize = 5;
pub const MAX_REWARD_CARD_BUNDLES: usize = ORRERY_BUNDLE_COUNT;
const _: () = assert!(CARD_REWARD_BASE_COUNT + 1 <= MAX_COMBAT_CARD_REWARD);
pub const FACTOR_WEAK: f32 = 0.75;
pub const FACTOR_WEAK_PAPER_KRANE: f32 = 0.60;
pub const FACTOR_VULN: f32 = 1.50;
pub const FACTOR_VULN_ODD_MUSHROOM: f32 = 1.25;
pub const FACTOR_FRAIL: f32 = 0.75;
pub const MODE_SHIFT_INCREASE_PER_CYCLE: i16 = 10;
pub const HEXAGHOST_DIVIDER_HITS: u8 = 6;

// Divider rolls `character_health / DIVISOR + 1` per hit at move selection
pub const HEXAGHOST_DIVIDER_DIVISOR: u16 = 12;
pub const MAX_MONSTERS: usize = 5;

// Girya: maximum rest-site lifts
pub const GIRYA_LIFT_MAX: i16 = 3;

// Matryoshka's extra chest Relic: 75% Common / 25% Uncommon
pub const MATRYOSHKA_TH_COMMON: u8 = 75;
pub const MATRYOSHKA_TH_UNCOMMON: u8 = 100;
pub const MAX_BLOCK: u16 = 999;
pub const NIGHTMARE_COPIES: u8 = 3;

// The Bomb: lazily armed detonation timer (see process_effect_turn_end)
pub const BOMB_FUSE_TURNS: u8 = 3;

// Event roll-pool stakes: the widest offer any event places in `id_roll_*`
pub const MAX_EVENT_ROLL_CARDS: usize = 1; // We Meet Again
pub const MAX_EVENT_ROLL_RELICS: usize = 2; // N'loth's two owned relics
pub const MAX_EVENT_ROLL_POTIONS: usize = 1; // We Meet Again

// Card rewards
pub const CARD_REWARD_ROLL_OFFSET_BASE: i8 = 5;
pub const CARD_REWARD_ROLL_OFFSET_MIN: i8 = -40;
pub const CARD_REWARD_ROLL_CHANCE_RARE: i32 = 3;
pub const CARD_REWARD_ROLL_CHANCE_UNCOMMON: i32 = 40;

// Acts, 1-based; the run ends at the ACT_FINAL boss
pub const ACT_FINAL: u8 = 2;

// Map
pub const MAP_HEIGHT: usize = 15;
pub const MAP_WIDTH: usize = 7;
pub const PATH_DENSITY: usize = 6;
pub const ANCESTOR_GAP_MIN: usize = 3;
pub const FACTOR_NUM_REST_SITE: f32 = 0.12;
pub const FACTOR_NUM_ELITE: f32 = 0.08;
pub const FACTOR_NUM_ELITE_A1_MULT: f32 = 1.6;
pub const FACTOR_NUM_EVENT: f32 = 0.22;
pub const FACTOR_NUM_SHOP: f32 = 0.05;
pub const MAP_ROW_TREASURE: usize = 8;
pub const UNKNOWN_CHANCE_BASE_MONSTER: f32 = 0.10;
pub const UNKNOWN_CHANCE_BASE_SHOP: f32 = 0.03;
pub const UNKNOWN_CHANCE_BASE_TREASURE: f32 = 0.02;
pub const EVENT_SPECIAL_CHANCE: f32 = 0.25;

// Chest size roll thresholds
pub const CHEST_SMALL_PCT: u8 = 50;
pub const CHEST_SMALL_PLUS_MEDIUM_PCT: u8 = 83;
pub const CHEST_SMALL_GOLD_CHANCE: u8 = 50;
pub const CHEST_SMALL_GOLD_BASE: u16 = 25;
pub const CHEST_SMALL_TH_COMMON: u8 = 75;
pub const CHEST_SMALL_TH_UNCOMMON: u8 = 100;
pub const CHEST_MEDIUM_GOLD_CHANCE: u8 = 35;
pub const CHEST_MEDIUM_GOLD_BASE: u16 = 50;
pub const CHEST_MEDIUM_TH_COMMON: u8 = 35;
pub const CHEST_MEDIUM_TH_UNCOMMON: u8 = 85;
pub const CHEST_LARGE_GOLD_CHANCE: u8 = 50;
pub const CHEST_LARGE_GOLD_BASE: u16 = 75;
pub const CHEST_LARGE_TH_COMMON: u8 = 0;
pub const CHEST_LARGE_TH_UNCOMMON: u8 = 75;

// Chest gold: base x U[0.9, 1.1] inclusive
pub const CHEST_GOLD_VARIANCE_MIN: f32 = 0.9;
pub const CHEST_GOLD_VARIANCE_MAX: f32 = 1.1;

// Cumulative thresholds for Relic-tier roll (elite reward, random grant)
pub const RELIC_TIER_TH_COMMON: u8 = 50;
pub const RELIC_TIER_TH_UNCOMMON: u8 = 83;

// Encounter sequence sizes
pub const NUM_ENCOUNTERS_EASY: usize = 3;
pub const NUM_ENCOUNTERS_EASY_ACT2: usize = 2;
pub const NUM_ENCOUNTERS_HARD: usize = MAP_HEIGHT - NUM_ENCOUNTERS_EASY;
pub const NUM_ENCOUNTERS_ELITE: usize = 10;
pub const ENCOUNTER_POOL_CAPACITY_NORMAL: usize = NUM_ENCOUNTERS_EASY + 1 + NUM_ENCOUNTERS_HARD;
pub const ENCOUNTER_POOL_CAPACITY_ELITE: usize = NUM_ENCOUNTERS_ELITE;

// Potions
pub const POTION_SLOTS_DEFAULT: u8 = 3;
pub const POTION_SLOTS_DEFAULT_A11: u8 = 2;
pub const POTION_SLOTS_MAX: usize = 5;

// Potion drop swing: chance = base + mod; +10 on miss, -10 on hit
pub const POTION_DROP_CHANCE_BASE: i8 = 40;
pub const POTION_DROP_CHANCE_MOD_HIT: i8 = -10;
pub const POTION_DROP_CHANCE_MOD_MISS: i8 = 10;
pub const POTION_DROP_CHANCE_MOD_MIN: i8 = -30;
pub const POTION_DROP_CHANCE_MOD_MAX: i8 = 60;

// Potion rarity roll thresholds
pub const POTION_TH_COMMON: u8 = 65;
pub const POTION_TH_UNCOMMON: u8 = 90;

// Discovery: number of Card options offered
pub const DISCOVER_PICK_COUNT: u8 = 3;

// Neow: Cards per offer, 33% Uncommon-else-Common roll, gold amounts, Lament charge count
pub const NEOW_CARD_COUNT: usize = 3;
pub const NEOW_POTION_COUNT: u8 = 3;
pub const NEOW_UNCOMMON_CHANCE: f64 = 0.33;
pub const NEOW_GOLD_SMALL: u16 = 100;
pub const NEOW_GOLD_LARGE: u16 = 250;
pub const NEOW_LAMENT_COMBATS: i16 = 3;

// We Meet Again: gold ask rolled MIN..=gold.min(MAX); option unavailable below MIN
pub const WE_MEET_AGAIN_GOLD_ASK_MIN: u16 = 50;
pub const WE_MEET_AGAIN_GOLD_ASK_MAX: u16 = 150;

// Shop pricing — Cards: base x U[0.9, 1.1], colorless x 1.2
pub const SHOP_PRICE_CARD_COMMON: u16 = 50;
pub const SHOP_PRICE_CARD_UNCOMMON: u16 = 75;
pub const SHOP_PRICE_CARD_RARE: u16 = 150;
pub const SHOP_PRICE_COLORLESS_NUMER: u16 = 6;
pub const SHOP_PRICE_COLORLESS_DENOM: u16 = 5;
pub const SHOP_PRICE_CARD_VARIANCE_MIN: f32 = 0.9;
pub const SHOP_PRICE_CARD_VARIANCE_MAX: f32 = 1.1;

// Shop pricing — Potions and Relics: base x U[0.95, 1.05]
pub const SHOP_PRICE_POTION_COMMON: u16 = 50;
pub const SHOP_PRICE_POTION_UNCOMMON: u16 = 75;
pub const SHOP_PRICE_POTION_RARE: u16 = 100;
pub const SHOP_PRICE_RELIC_COMMON: u16 = 150;
pub const SHOP_PRICE_RELIC_UNCOMMON: u16 = 250;
pub const SHOP_PRICE_RELIC_RARE: u16 = 300;
pub const SHOP_PRICE_RELIC_SHOP: u16 = 150;
pub const SHOP_PRICE_RELIC_POTION_VARIANCE_MIN: f32 = 0.95;
pub const SHOP_PRICE_RELIC_POTION_VARIANCE_MAX: f32 = 1.05;

pub const SHOP_PURGE_COST_BASE: u16 = 75;
pub const SHOP_PURGE_COST_INCREMENT: u16 = 25;

// One random colored-Card slot is floor-halved, pre-markup
pub const SHOP_SALE_DIVISOR: u16 = 2;

// A16+ markup: Card/Relic/Potion prices x 11/10 rounded half-up; purge cost is exempt
pub const ASCENSION_SHOP_PRICE_BUMP_LEVEL: u8 = 16;
pub const ASCENSION_SHOP_PRICE_BUMP_NUMER: u16 = 11;
pub const ASCENSION_SHOP_PRICE_BUMP_DENOM: u16 = 10;

// The one A16 markup formula; process_effect_shop_build applies it to live prices
pub const fn bump_price_a16(price: u16) -> u16 {
    ((price as u32 * ASCENSION_SHOP_PRICE_BUMP_NUMER as u32
        + ASCENSION_SHOP_PRICE_BUMP_DENOM as u32 / 2)
        / ASCENSION_SHOP_PRICE_BUMP_DENOM as u32) as u16
}

// Largest price the half-open variance roll in engine/shop.rs can produce
const fn price_sup(base: u16, variance_max: f32) -> u16 {
    let max = base as f32 * variance_max;
    let truncated = max as u16;
    if truncated as f32 == max {
        truncated - 1
    } else {
        truncated
    }
}

// Derived stock-price ceilings, post-A16 markup, pre-discount (discounts only lower)
pub const SHOP_PRICE_CARD_MAX: u16 = bump_price_a16(price_sup(
    SHOP_PRICE_CARD_RARE * SHOP_PRICE_COLORLESS_NUMER / SHOP_PRICE_COLORLESS_DENOM,
    SHOP_PRICE_CARD_VARIANCE_MAX,
));
pub const SHOP_PRICE_RELIC_MAX: u16 = bump_price_a16(price_sup(
    SHOP_PRICE_RELIC_RARE,
    SHOP_PRICE_RELIC_POTION_VARIANCE_MAX,
));
pub const SHOP_PRICE_POTION_MAX: u16 = bump_price_a16(price_sup(
    SHOP_PRICE_POTION_RARE,
    SHOP_PRICE_RELIC_POTION_VARIANCE_MAX,
));

// Shop inventory composition
pub const SHOP_SLOTS_CARD_COLORED: usize = 5;
const SHOP_SLOTS_CARD_COLORLESS: usize = 2;
pub const SHOP_SLOTS_CARD_TOTAL: usize = SHOP_SLOTS_CARD_COLORED + SHOP_SLOTS_CARD_COLORLESS;
pub const SHOP_SLOTS_RELIC: usize = 3;
pub const SHOP_SLOTS_POTION: usize = 3;

// Colored-Card rarity weights (cumulative < thresholds)
pub const SHOP_CARD_TH_COMMON: u8 = 60;
pub const SHOP_CARD_TH_UNCOMMON: u8 = 97;

// Relic-tier weights for the 2 non-shop Relic slots
pub const SHOP_RELIC_TH_COMMON: u8 = 48;
pub const SHOP_RELIC_TH_UNCOMMON: u8 = 82;

// Initial capacity for the per-handler effect builder on GameState
pub const MAX_EFFECTS_PER_HANDLER: usize = 32;

// Initial capacity for the per-resolve candidate buffer on GameState
pub const MAX_CANDIDATES: usize = 128;
