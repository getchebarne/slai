use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static EXPERTISE: Entity = make_entity_card(
    CardName::Expertise,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DrawUpTo { amount: 6 },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static EXPERTISE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = EXPERTISE.card_effects;
        a[0].kind = EffectKind::DrawUpTo { amount: 7 }; // +1 draw
        a
    },
    ..EXPERTISE
};
