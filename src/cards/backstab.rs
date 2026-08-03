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
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 11 },
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
        let mut a = BACKSTAB.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 15 }; // +4 damage
        a
    },
    ..BACKSTAB
};
