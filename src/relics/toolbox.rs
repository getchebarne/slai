use crate::consts::NUM_DISCOVER_ROLLS;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::effect::effect_discover_select;
use crate::relics::RelicTemplate;
use crate::types::CardColor;
use crate::types::CardPile;
use crate::types::RelicName;
use crate::types::RelicTier;

// At combat start, choose 1 of 3 colorless Cards to add to the hand
// See:
//    - `process_effect_turn_start_character.rs`
pub static TOOLBOX: RelicTemplate = RelicTemplate {
    name: RelicName::Toolbox,
    tier: RelicTier::Shop,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[
        Effect {
            kind: EffectKind::CardDiscoverRoll {
                kind: None,
                color: CardColor::Colorless,
                exclude: &[],
                count: NUM_DISCOVER_ROLLS,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        effect_discover_select(None, CardPile::Hand, 1, false),
    ],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
