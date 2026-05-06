use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static STRIKE: Entity = make_entity_card(
    CardName::Strike,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Basic,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 6 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
    &[],
);
// Upgraded
pub static STRIKE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = STRIKE.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 9 }; // +3 damage
        a
    },
    ..STRIKE
};
