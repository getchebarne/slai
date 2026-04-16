use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static OUTMANEUVER: Entity = card_entity(
    CardName::Outmaneuver, CardKind::Skill, CardColor::Green, CardRarity::Common,
    1, false, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::NextTurnEnergy,
            stacks: 2,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static OUTMANEUVER_PLUS: Entity = card_entity(
    CardName::Outmaneuver, CardKind::Skill, CardColor::Green, CardRarity::Common,
    1, true, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::NextTurnEnergy,
            stacks: 3, // +1 next-turn-energy
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
