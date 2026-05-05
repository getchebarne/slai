use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DEADLY_POISON: Entity = make_entity_card(
    CardName::DeadlyPoison,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 5,
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
pub static DEADLY_POISON_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = DEADLY_POISON.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 7, // +2 poison
        };
        a
    },
    ..DEADLY_POISON
};
