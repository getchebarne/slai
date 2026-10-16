use crate::consts::DISCOVER_ROLL_COUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::effect::effect_discover_select;
use crate::relics::RelicTemplate;
use crate::types::CardColor;
use crate::types::CardPile;
use crate::types::RelicName;
use crate::types::RelicTier;

// End of turn: discover a Card and shuffle it into the draw pile, or skip it
// See:
//    - `process_effect_turn_end_character.rs`
pub static NILRYS_CODEX: RelicTemplate = RelicTemplate {
    name: RelicName::NilrysCodex,
    tier: RelicTier::Special,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[
        Effect {
            kind: EffectKind::CardDiscoverRoll {
                kind: None,
                color: CardColor::Green,
                exclude: &[],
                count: DISCOVER_ROLL_COUNT,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        effect_discover_select(None, CardPile::Draw, 1, true),
    ],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
