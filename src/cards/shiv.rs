use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity, Tag};

pub static SHIV: Entity = make_entity_card(
    CardName::Shiv,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Special,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 4 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
    &[Tag::Shiv],
);
// Upgraded
pub static SHIV_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = SHIV.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 6 }; // +2 damage
        a
    },
    ..SHIV
};
