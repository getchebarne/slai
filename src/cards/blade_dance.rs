use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static BLADE_DANCE: Entity = make_entity_card(
    CardName::BladeDance,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ShivAdd {
            count: 3,
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
pub static BLADE_DANCE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = BLADE_DANCE.card_effects;
        a[0].kind = EffectKind::ShivAdd {
            count: 4, // +1 shiv
            upgraded: false,
        };
        a
    },
    ..BLADE_DANCE
};
