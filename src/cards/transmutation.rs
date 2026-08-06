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
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::CostScope;

pub static TRANSMUTATION: Entity = make_entity_card(
    CardName::Transmutation,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Rare,
    0,
    CardCostKind::XCost { offset: 0 },
    false,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardAddRandom {
            color: CardColor::Colorless,
            kind: None,
            pile: CardPile::Hand,
            count: 1,
            cost_zero: Some(CostScope::Turn),
            upgraded: false,
            rarity: None,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static TRANSMUTATION_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = TRANSMUTATION.card_effects;
        a[0].kind = EffectKind::CardAddRandom {
            color: CardColor::Colorless,
            kind: None,
            pile: CardPile::Hand,
            count: 1,
            cost_zero: Some(CostScope::Turn),
            upgraded: true, // Added Cards are upgraded
            rarity: None,
        };
        a
    },
    ..TRANSMUTATION
};
