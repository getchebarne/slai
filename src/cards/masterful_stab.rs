use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static MASTERFUL_STAB: Entity = make_entity_card(
    CardName::MasterfulStab,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::GrowsOnDamageInstanceTaken,
    false,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 12 },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static MASTERFUL_STAB_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = MASTERFUL_STAB.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 16 }; // +4 damage
        a
    },
    ..MASTERFUL_STAB
};
