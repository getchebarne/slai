use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static SETUP: Entity = make_entity_card(
    CardName::Setup,
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
        kind: EffectKind::CardSetupPick,
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Hand,
            selection: SelectionKind::Input { count: 1 },
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
    &[],
);
// Upgraded
pub static SETUP_PLUS: Entity = Entity {
    card_upgraded: true,
    card_cost: 0, // -1 cost
    ..SETUP
};
