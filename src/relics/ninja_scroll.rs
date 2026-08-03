use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static NINJA_SCROLL: Entity = make_entity_relic(
    RelicName::NinjaScroll,
    RelicTier::Uncommon,
    0,
    &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Shiv,
            pile: CardPile::Hand,
            count: 3,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
);
