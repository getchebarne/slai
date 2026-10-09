use crate::consts::DISCOVER_PICK_COUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::effect::effect_discover_pick;
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
                count: DISCOVER_PICK_COUNT,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        effect_discover_pick(None, CardPile::Hand, 1, false),
    ],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
