use crate::effect::{
    CandidatePool, DiscardSource, Effect, EffectKind, SelectionKind, Target,
};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static ALL_OUT_ATTACK: Entity = make_entity_card(
    CardName::AllOutAttack,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 10 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDiscard {
                source: DiscardSource::Explicit,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Random { count: 1 },
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static ALL_OUT_ATTACK_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = ALL_OUT_ATTACK.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 14 }; // +4 damage
        a
    },
    ..ALL_OUT_ATTACK
};
