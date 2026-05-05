use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DEFEND: Entity = make_entity_card(
    CardName::Defend,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Basic,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::BlockGain { amount: 5 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
    &[],
);
// Upgraded
pub static DEFEND_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = DEFEND.card_effects;
        a[0].kind = EffectKind::BlockGain { amount: 8 }; // +3 block
        a
    },
    ..DEFEND
};
