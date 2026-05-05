use crate::effect::{CandidatePool, DamageCondition, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static UNLOAD: Entity = make_entity_card(
    CardName::Unload,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
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
                amount: 14,
                condition: DamageCondition::Always,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::UnloadDiscard,
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static UNLOAD_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = UNLOAD.card_effects;
        a[0].kind = EffectKind::DamagePhysical {
            amount: 18, // +4 damage
            condition: DamageCondition::Always,
        };
        a
    },
    ..UNLOAD
};
