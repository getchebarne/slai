use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static FLASH_OF_STEEL: Entity = make_entity_card(
    CardName::FlashOfSteel,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 3 },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static FLASH_OF_STEEL_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = FLASH_OF_STEEL.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 6 }; // +3 damage
        a
    },
    ..FLASH_OF_STEEL
};
