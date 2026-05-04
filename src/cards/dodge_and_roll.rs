use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DODGE_AND_ROLL: Entity = make_entity_card(
    CardName::DodgeAndRoll,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 4 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NextTurnBlock,
                stacks: 4,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::Single,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DODGE_AND_ROLL_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = DODGE_AND_ROLL.card_effects;
        a[0].kind = EffectKind::BlockGain { amount: 6 }; // +2 block
        a[1].kind = EffectKind::ModifierGain {
            kind: ModifierKind::NextTurnBlock,
            stacks: 6, // +2 next-turn-block
        };
        a
    },
    ..DODGE_AND_ROLL
};
