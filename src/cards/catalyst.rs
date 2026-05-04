use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static CATALYST: Entity = make_entity_card(
    CardName::Catalyst,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::ModifierMultiply {
            kind: ModifierKind::Poison,
            factor: 2,
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
// Upgraded: triples instead of doubles
pub static CATALYST_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = CATALYST.card_effects;
        a[0].kind = EffectKind::ModifierMultiply {
            kind: ModifierKind::Poison,
            factor: 3, // +1 factor
        };
        a
    },
    ..CATALYST
};
