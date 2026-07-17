use rand::Rng;

use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::entity::make_entity_event;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::CardRarity;
use crate::types::DeltaSign;
use crate::types::EventName;

const RELIC_REWARD: Effect = Effect {
    kind: EffectKind::RelicGrantRandom,
    id_source: None,
    target: Target::Direct(None),
};

// Give potion
const OPTION_GIVE_POTION: &[Effect] = &[
    Effect {
        kind: EffectKind::PotionDiscard,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::EventPickPotion,
            selection_kind: SelectionKind::Single,
        },
    },
    RELIC_REWARD,
    EVENT_CONSUME_EFFECT,
];

// Give give gold
const OPTION_GIVE_GOLD: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::EventRoll { idx: 0 },
        },
        id_source: None,
        target: Target::Direct(None),
    },
    RELIC_REWARD,
    EVENT_CONSUME_EFFECT,
];

const OPTION_GIVE_CARD: &[Effect] = &[
    Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::EventPickCard,
            selection_kind: SelectionKind::Single,
        },
    },
    RELIC_REWARD,
    EVENT_CONSUME_EFFECT,
];

// Attack
const OPTION_ATTACK: &[Effect] = &[EVENT_CONSUME_EFFECT];

// All options
const OPTIONS_ALL: &[EventOption] = &[
    EventOption {
        label: "[Give Potion] Lose the offered potion. Obtain a random relic.",
        effects: OPTION_GIVE_POTION,
        gate: EventGate::EventPickValid(EntityKind::Potion),
    },
    EventOption {
        label: "[Give Gold] Lose the asked gold. Obtain a random relic.",
        effects: OPTION_GIVE_GOLD,
        gate: EventGate::GoldAtLeast(50),
    },
    EventOption {
        label: "[Give Card] Lose the offered card. Obtain a random relic.",
        effects: OPTION_GIVE_CARD,
        gate: EventGate::EventPickValid(EntityKind::Card),
    },
    EventOption {
        label: "[Attack] Nothing happens.",
        effects: OPTION_ATTACK,
        gate: EventGate::None,
    },
];

// Export event
static EVENT_WE_MEET_AGAIN: Entity = make_entity_event(EventName::WeMeetAgain, OPTIONS_ALL);
pub fn spawn_event_we_meet_again(state: &mut GameState) -> Entity {
    let mut event = EVENT_WE_MEET_AGAIN;

    // Card offer: uniform among non-Basic, non-Curse deck cards
    let eligible: Vec<usize> = state
        .id_deck
        .iter()
        .copied()
        .filter(|&id| {
            let entity = &state.entities[id];
            entity.card_rarity != CardRarity::Basic && entity.card_kind != CardKind::Curse
        })
        .collect();
    if !eligible.is_empty() {
        let id = eligible[state.rng.random_range(0..eligible.len())];
        state.id_event_picks.push(id);
    }

    // Potion offer: uniform among occupied belt slots
    let slotted: Vec<usize> = state.id_potions.iter().flatten().copied().collect();
    if !slotted.is_empty() {
        let id = slotted[state.rng.random_range(0..slotted.len())];
        state.id_event_picks.push(id);
    }

    // Gold ask into roll slot 0: 50..=150, capped by holdings; unrolled (option
    // gated out by GoldAtLeast) below 50
    let gold = state.entities[state.id_character].character_gold;
    if gold >= 50 {
        event.event_rolls[0] = state.rng.random_range(50..=gold.min(150));
        event.event_rolls_len = 1;
    }

    event
}
