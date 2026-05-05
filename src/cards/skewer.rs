use crate::effect::{CandidatePool, DamageCondition, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static SKEWER: Entity = make_entity_card(
    CardName::Skewer,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::XCost { offset: 0 },
    false,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 7,
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
pub static SKEWER_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = SKEWER.card_effects;
        a[0].kind = EffectKind::DamagePhysical {
            amount: 10, // +3 damage
            condition: DamageCondition::Always,
        };
        a
    },
    ..SKEWER
};
