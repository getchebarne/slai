use crate::effect::{CandidatePool, DamageCondition, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DAGGER_SPRAY: Entity = make_entity_card(
    CardName::DaggerSpray,
    CardKind::Attack,
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
            kind: EffectKind::DamagePhysical {
                amount: 4,
                condition: DamageCondition::Always,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 4,
                condition: DamageCondition::Always,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DAGGER_SPRAY_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = DAGGER_SPRAY.card_effects;
        let upgraded_kind = EffectKind::DamagePhysical {
            amount: 6, // +2 damage
            condition: DamageCondition::Always,
        };
        a[0].kind = upgraded_kind;
        a[1].kind = upgraded_kind;
        a
    },
    ..DAGGER_SPRAY
};
