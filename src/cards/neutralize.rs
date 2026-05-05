use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static NEUTRALIZE: Entity = make_entity_card(
    CardName::Neutralize,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Basic,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 3 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 1,
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
    &[],
);
// Upgraded
pub static NEUTRALIZE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = NEUTRALIZE.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 4 }; // +1 damage
        a[1].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 2, // +1 stack
        };
        a
    },
    ..NEUTRALIZE
};
