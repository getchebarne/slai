use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static STORM_OF_STEEL: Entity = make_entity_card(
    CardName::StormOfSteel,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::StormOfSteelProc { upgraded: false },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static STORM_OF_STEEL_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = STORM_OF_STEEL.card_effects;
        a[0].kind = EffectKind::StormOfSteelProc { upgraded: true }; // Shivs are upgraded
        a
    },
    ..STORM_OF_STEEL
};
