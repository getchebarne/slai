use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardPile;
use crate::types::CostScope;
use crate::types::RelicName;
use crate::types::RelicTier;

// Combat start: a random Power lands in hand, costing 0 for the turn
pub static ENCHIRIDION: RelicTemplate = RelicTemplate {
    name: RelicName::Enchiridion,
    tier: RelicTier::Special,
    counter_init: 0,
    effects_combat_start: &[Effect {
        kind: EffectKind::CardAddRandom {
            color: CardColor::Green,
            kind: Some(CardKind::Power),
            pile: CardPile::Hand,
            count: 1,
            cost_zero: Some(CostScope::Turn),
            upgraded: false,
            rarity: None,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
};
