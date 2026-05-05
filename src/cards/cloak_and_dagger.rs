use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static CLOAK_AND_DAGGER: Entity = make_entity_card(
    CardName::CloakAndDagger,
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
            kind: EffectKind::BlockGain { amount: 6 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ShivAdd {
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static CLOAK_AND_DAGGER_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = CLOAK_AND_DAGGER.card_effects;
        a[1].kind = EffectKind::ShivAdd {
            count: 2, // +1 shiv
            upgraded: false,
        };
        a
    },
    ..CLOAK_AND_DAGGER
};
