use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity, Tag};

pub static SNEAKY_STRIKE: Entity = make_entity_card(
    CardName::SneakyStrike,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 12 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::SneakyStrikeProc { energy: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
    &[Tag::Discard],
);
// Upgraded
pub static SNEAKY_STRIKE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = SNEAKY_STRIKE.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 16 }; // +4 damage
        a
    },
    ..SNEAKY_STRIKE
};
