use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static FLYING_KNEE: Entity = card_entity(
    CardName::FlyingKnee, CardKind::Attack, CardColor::Green, CardRarity::Common,
    1, false, false, false, true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { base: 8 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NextTurnEnergy,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
);
// Upgraded
pub static FLYING_KNEE_PLUS: Entity = card_entity(
    CardName::FlyingKnee, CardKind::Attack, CardColor::Green, CardRarity::Common,
    1, true, false, false, true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                base: 11, // +3 damage
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NextTurnEnergy,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
);
