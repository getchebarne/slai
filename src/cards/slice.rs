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

pub static SLICE: Entity = make_entity_card(
    CardName::Slice,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 6,
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
pub static SLICE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = SLICE.card_effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 9,
            lifesteal: false,
        }; // +3 damage
        effects
    },
    ..SLICE
};
