use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DEFLECT: Entity = make_entity_card(
    CardName::Deflect,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::BlockGain { amount: 4 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DEFLECT_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = DEFLECT.card_effects;
        a[0].kind = EffectKind::BlockGain { amount: 7 }; // +3 block
        a
    },
    ..DEFLECT
};
