use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::RelicName;
use crate::types::RelicTier;

// The first Attack costing 2+ each turn is played twice; grants Necronomicurse on pickup
// See:
//    - `process_effect_card_play.rs`
//    - `process_effect_relic_adopt.rs`
pub static NECRONOMICON: RelicTemplate = RelicTemplate {
    name: RelicName::Necronomicon,
    tier: RelicTier::Special,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Necronomicurse,
            pile: CardPile::Deck,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    effects_rest: &[],
    effects_counter: &[],
};
