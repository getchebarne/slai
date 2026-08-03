use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static MASTER_OF_STRATEGY: Entity = make_entity_card(
    CardName::MasterOfStrategy,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardDraw { count: 3 },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static MASTER_OF_STRATEGY_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = MASTER_OF_STRATEGY.card_effects;
        a[0].kind = EffectKind::CardDraw { count: 4 }; // +1 draw
        a
    },
    ..MASTER_OF_STRATEGY
};
