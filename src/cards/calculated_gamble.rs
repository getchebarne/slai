use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity, Tag};

pub static CALCULATED_GAMBLE: Entity = make_entity_card(
    CardName::CalculatedGamble,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::CalculatedGamble,
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
    &[Tag::Discard],
);
// Upgraded
pub static CALCULATED_GAMBLE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_exhaust: false, // upgrade removes exhaust
    ..CALCULATED_GAMBLE
};
