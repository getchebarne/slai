use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// EndlessAgony: 0-cost attack, 4 (6+) damage, exhaust. When drawn, add a
// copy to your hand (overflow → discard, per StS MakeTempCardInHandAction).
// The on-draw hook fires from process_effect_card_draw via
// `card_on_draw_effects`. Spawned copies are themselves drawn by being put
// directly into hand/discard (NOT via CardDraw), so they don't re-trigger
// the on_draw hook — only being drawn from the draw pile does.
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

pub static ENDLESS_AGONY: Entity = {
    let mut e = make_entity_card(
        CardName::EndlessAgony,
        CardKind::Attack,
        CardColor::Green,
        CardRarity::Uncommon,
        0,
        false,
        true, // exhaust
        false,
        true,
        &[Effect {
            kind: EffectKind::DamagePhysical { amount: 4 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        }],
    );
    e.card_on_draw_effects = ENDLESS_AGONY_ON_DRAW;
    e
};
// Upgraded: damage 4 -> 6
pub static ENDLESS_AGONY_PLUS: Entity = {
    let mut e = make_entity_card(
        CardName::EndlessAgony,
        CardKind::Attack,
        CardColor::Green,
        CardRarity::Uncommon,
        0,
        true,
        true, // exhaust
        false,
        true,
        &[Effect {
            kind: EffectKind::DamagePhysical { amount: 6 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        }],
    );
    e.card_on_draw_effects = ENDLESS_AGONY_PLUS_ON_DRAW;
    e
};
