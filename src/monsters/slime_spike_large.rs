use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack_card_add;
use crate::monsters::move_debuff;
use crate::monsters::move_split;
use crate::monsters::slime_spike_medium::get_next_move_slime_spike;
use crate::types::CardName;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_FLAME_TACKLE_16: Move =
    move_attack_card_add("Flame Tackle", 16, CardName::Slimed, 2, false);
static MOVE_FLAME_TACKLE_18: Move =
    move_attack_card_add("Flame Tackle", 18, CardName::Slimed, 2, false);
static MOVE_LICK_FRAIL_2: Move = move_debuff("Lick", ModifierKind::Frail, 2, Intent::Debuff);
static MOVE_LICK_FRAIL_3: Move = move_debuff("Lick", ModifierKind::Frail, 3, Intent::Debuff);
static MOVE_SPLIT: Move = move_split(
    "Split",
    MonsterName::SlimeSpikeMedium,
    -1,
    MonsterName::SlimeSpikeMedium,
    0,
);

static MOVES_ASC0: [Move; 3] = [MOVE_FLAME_TACKLE_16, MOVE_LICK_FRAIL_2, MOVE_SPLIT];
static MOVES_ASC2: [Move; 3] = [MOVE_FLAME_TACKLE_18, MOVE_LICK_FRAIL_2, MOVE_SPLIT];
static MOVES_ASC17: [Move; 3] = [MOVE_FLAME_TACKLE_18, MOVE_LICK_FRAIL_3, MOVE_SPLIT];

pub const IDX_MOVE_SPLIT: usize = 2;

pub static SLIME_SPIKE_LARGE: MonsterTemplate = MonsterTemplate {
    name: MonsterName::SlimeSpikeLarge,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (64, 70)), (7, (67, 73))],
    block_start: 0,
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (2, &[&MOVES_ASC2]),
        (17, &[&MOVES_ASC17]),
    ],
    modifier_tiers: &[(0, &[modifier_fixed(ModifierKind::Splittable, 1)])],
};

// The medium's script: Flame Tackle and Lick share its move indices
pub fn get_next_move_slime_spike_large(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    // A Split forced during its own move survives the move's trailing roll
    if move_history.last() == Some(&(IDX_MOVE_SPLIT as u8)) {
        return IDX_MOVE_SPLIT;
    }
    get_next_move_slime_spike(move_history, ascension_level, rng)
}
