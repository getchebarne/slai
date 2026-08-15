use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BACKSTAB: Entity = make_entity_card(
    CardName::Backstab,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 11,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static BACKSTAB_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = BACKSTAB.card_effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 15,
            lifesteal: false,
        }; // +4 damage
        effects
    },
    ..BACKSTAB
};
