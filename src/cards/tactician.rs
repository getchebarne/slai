use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

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
