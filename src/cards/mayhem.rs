use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static MAYHEM: Entity = make_entity_card(
    CardName::Mayhem,
    CardKind::Power,
    CardColor::Colorless,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Mayhem,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static MAYHEM_PLUS: Entity = Entity {
    card_upgraded: true,
    card_cost: 1, // -1 cost
    ..MAYHEM
};
