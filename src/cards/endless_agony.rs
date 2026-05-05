use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

const ENDLESS_AGONY_ON_DRAW: &[Effect] = &[Effect {
    kind: EffectKind::EndlessAgonyAddCopy { upgraded: false },
    id_source: None,
    target: Target::Direct(None),
}];
const ENDLESS_AGONY_PLUS_ON_DRAW: &[Effect] = &[Effect {
    kind: EffectKind::EndlessAgonyAddCopy { upgraded: true },
    id_source: None,
    target: Target::Direct(None),
}];

pub static ENDLESS_AGONY: Entity = make_entity_card(
    CardName::EndlessAgony,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
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
    ENDLESS_AGONY_ON_DRAW,
    PlayRestriction::Always,
);
// Upgraded
pub static ENDLESS_AGONY_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = ENDLESS_AGONY.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 6 }; // +2 damage
        a
    },
    card_on_draw_effects: ENDLESS_AGONY_PLUS_ON_DRAW,
    ..ENDLESS_AGONY
};
