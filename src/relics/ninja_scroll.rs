use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static NINJA_SCROLL: RelicTemplate = RelicTemplate {
    name: RelicName::NinjaScroll,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Shiv,
            pile: CardPile::Hand,
            count: 3,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
};
