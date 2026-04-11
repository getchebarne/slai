use crate::cards::Card;
use crate::effect::{CandidatePool, EffectKind, Effect, SelectionKind, Targeting};
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static OUTMANEUVER: Card = Card {
    name: CardName::Outmaneuver,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::NextTurnEnergy,
            stacks: 2,
        },
        source: None,
        targeting: Targeting::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
};
// Upgraded
pub static OUTMANEUVER_PLUS: Card = Card {
    name: CardName::Outmaneuver,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::NextTurnEnergy,
            stacks: 3, // +1 next-turn-energy
        },
        source: None,
        targeting: Targeting::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
};
