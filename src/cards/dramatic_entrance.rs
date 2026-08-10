use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DRAMATIC_ENTRANCE: Entity = make_entity_card(
    CardName::DramaticEntrance,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    true,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 8,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DRAMATIC_ENTRANCE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = DRAMATIC_ENTRANCE.card_effects;
        a[0].kind = EffectKind::DamagePhysical {
            amount: 12,
            lifesteal: false,
        }; // +4 damage
        a
    },
    ..DRAMATIC_ENTRANCE
};
