use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static METAMORPHOSIS: Entity = make_entity_card(
    CardName::Metamorphosis,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardAddRandom {
            color: CardColor::Green,
            kind: Some(CardKind::Attack),
            count: 3,
            into_draw: true,
            cost_zero_turn: false,
            cost_zero_combat: true,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static METAMORPHOSIS_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = METAMORPHOSIS.card_effects;
        a[0].kind = EffectKind::CardAddRandom {
            color: CardColor::Green,
            kind: Some(CardKind::Attack),
            count: 5, // +2 cards
            into_draw: true,
            cost_zero_turn: false,
            cost_zero_combat: true,
            upgraded: false,
        };
        a
    },
    ..METAMORPHOSIS
};
