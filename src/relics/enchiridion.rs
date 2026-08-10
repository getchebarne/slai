use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardPile;
use crate::types::CostScope;
use crate::types::RelicName;
use crate::types::RelicTier;

// Combat start: a random Power lands in hand, costing 0 for the turn
pub static ENCHIRIDION: Entity = make_entity_relic(
    RelicName::Enchiridion,
    RelicTier::Special,
    0,
    &[Effect {
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
);
