use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::effect::TARGET_SOURCE;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static GLASS_KNIFE: Entity = make_entity_card(
    CardName::GlassKnife,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 8 },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 8 },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::GlassKnifeDecay { delta: -2 },
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static GLASS_KNIFE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = GLASS_KNIFE.card_effects;
        let upgraded_kind = EffectKind::DamagePhysical { amount: 12 }; // +4 damage
        a[0].kind = upgraded_kind;
        a[1].kind = upgraded_kind;
        a
    },
    ..GLASS_KNIFE
};
