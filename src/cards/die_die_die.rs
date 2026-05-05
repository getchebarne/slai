use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DIE_DIE_DIE: Entity = make_entity_card(
    CardName::DieDieDie,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 13 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
    &[],
);
// Upgraded
pub static DIE_DIE_DIE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = DIE_DIE_DIE.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 17 }; // +4 damage
        a
    },
    ..DIE_DIE_DIE
};
