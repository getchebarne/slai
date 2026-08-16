use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_apply;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move_attack_card_add;
use crate::monsters::make_move_debuff;
use crate::monsters::make_move_split;
use crate::types::CardName;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_FLAME_TACKLE_16: Move =
    make_move_attack_card_add("Flame Tackle", 16, CardName::Slimed, 2, false);
static MOVE_FLAME_TACKLE_18: Move =
    make_move_attack_card_add("Flame Tackle", 18, CardName::Slimed, 2, false);
static MOVE_LICK_FRAIL_2: Move = make_move_debuff("Lick", ModifierKind::Frail, 2, Intent::Debuff);
static MOVE_LICK_FRAIL_3: Move = make_move_debuff("Lick", ModifierKind::Frail, 3, Intent::Debuff);
static MOVE_SPLIT: Move = make_move_split(
    "Split",
    MonsterName::SlimeSpikeMedium,
    MonsterName::SlimeSpikeMedium,
);

static MOVES_ASC0: [Move; 3] = [MOVE_FLAME_TACKLE_16, MOVE_LICK_FRAIL_2, MOVE_SPLIT];
static MOVES_ASC2: [Move; 3] = [MOVE_FLAME_TACKLE_18, MOVE_LICK_FRAIL_2, MOVE_SPLIT];
static MOVES_ASC17: [Move; 3] = [MOVE_FLAME_TACKLE_18, MOVE_LICK_FRAIL_3, MOVE_SPLIT];

pub const IDX_MOVE_SPLIT: usize = 2;

pub fn spawn_monster_slime_spike_large(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (64, 70)
    } else {
        (67, 73)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    let mut modifiers = MODIFIERS_ZERO;
    modifier_apply(&mut modifiers, ModifierKind::Splittable, 1);

    make_entity_monster(
        MonsterName::SlimeSpikeLarge,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        modifiers,
        moves,
    )
}
