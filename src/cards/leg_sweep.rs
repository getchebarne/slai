use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static LEG_SWEEP: Entity = make_entity_card(
    CardName::LegSweep,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 11 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static LEG_SWEEP_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = LEG_SWEEP.card_effects;
        a[0].kind = EffectKind::BlockGain { amount: 14 }; // +3 block
        a[1].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 3, // +1 stack
        };
        a
    },
    ..LEG_SWEEP
};
