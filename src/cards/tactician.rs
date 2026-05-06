use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity, Tag};

pub static TACTICIAN: Entity = make_entity_card(
    CardName::Tactician,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[],
    &[Effect {
        kind: EffectKind::EnergyGain { amount: 1 },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    PlayRestriction::Never,
    &[Tag::Discard],
);
pub static TACTICIAN_PLUS: Entity = Entity {
    card_upgraded: true,
    card_on_discard_effects: &[Effect {
        kind: EffectKind::EnergyGain { amount: 2 }, // +1 energy
        id_source: None,
        target: Target::Direct(None),
    }],
    ..TACTICIAN
};
