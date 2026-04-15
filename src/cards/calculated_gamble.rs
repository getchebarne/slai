use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{Entity, card_entity};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static CALCULATED_GAMBLE: Entity = card_entity(
    CardName::CalculatedGamble, CardKind::Skill, CardColor::Green, CardRarity::Uncommon,
    0, false, true, false, false,
    &[Effect {
        kind: EffectKind::CalculatedGamble,
        source: None,
        target: Target::Direct(None),
    }],
);
// Upgraded
pub static CALCULATED_GAMBLE_PLUS: Entity = card_entity(
    CardName::CalculatedGamble, CardKind::Skill, CardColor::Green, CardRarity::Uncommon,
    0, true, false, false, false,
    &[Effect {
        kind: EffectKind::CalculatedGamble,
        source: None,
        target: Target::Direct(None),
    }],
);
