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

// End of turn: discover a Card and shuffle it into the draw pile
// See:
//    - `process_effect_turn_end.rs`
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
                count: DISCOVER_PICK_COUNT,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        effect_discover_pick(None, CardPile::Draw),
    ],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
