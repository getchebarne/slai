use crate::effect::{CandidatePool, DamageCondition, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static BACKSTAB: Entity = make_entity_card(
    CardName::Backstab,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    true,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 11,
            condition: DamageCondition::Always,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static BACKSTAB_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = BACKSTAB.card_effects;
        a[0].kind = EffectKind::DamagePhysical {
            amount: 15, // +4 damage
            condition: DamageCondition::Always,
        };
        a
    },
    ..BACKSTAB
};
