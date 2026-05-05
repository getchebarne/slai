use crate::effect::{CandidatePool, DamageCondition, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static BANE: Entity = make_entity_card(
    CardName::Bane,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 7,
                condition: DamageCondition::Always,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 7,
                condition: DamageCondition::IfPoisoned,
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
pub static BANE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = BANE.card_effects;
        a[0].kind = EffectKind::DamagePhysical {
            amount: 10, // +3 damage 
            condition: DamageCondition::Always,
        };
        a[1].kind = EffectKind::DamagePhysical {
            amount: 10, // +3 damage
            condition: DamageCondition::IfPoisoned,
        };
        a
    },
    ..BANE
};
