use rand::Rng;

use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventKind;
use crate::events::bake_options;
use crate::events::make_entity_event_option;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::utils::card_is_non_basic_non_curse;

const RELIC_REWARD: Effect = Effect {
    kind: EffectKind::RelicGrantRandom { tier: None },
    id_source: None,
    target: Target::Direct(None),
};

const OPTION_ATTACK: &[Effect] = &[EVENT_CONSUME_EFFECT];

// Spawn rolls the picks and the ask, then bakes them into the options;
// availability re-validates the picks at selection (the offered potion can be
// drunk while standing here)
pub fn spawn_event_we_meet_again(state: &mut GameState) -> (EventKind, Vec<usize>) {
    // Card offer: uniform among non-Basic, non-Curse deck Cards
    let eligible: Vec<usize> = state
        .id_card_deck
        .iter()
        .copied()
        .filter(|&id| card_is_non_basic_non_curse(&state.entities[id]))
        .collect();
    let id_card =
        (!eligible.is_empty()).then(|| eligible[state.rng.random_range(0..eligible.len())]);

    // Potion offer: uniform among occupied belt slots
    let slotted: Vec<usize> = state.id_potions.iter().flatten().copied().collect();
    let id_potion =
        (!slotted.is_empty()).then(|| slotted[state.rng.random_range(0..slotted.len())]);

    // Gold ask: 50..=150, capped by holdings; unrolled (option unavailable) below 50
    let gold = state.entities[state.id_character].character_gold;
    let gold_ask = (gold >= 50).then(|| state.rng.random_range(50..=gold.min(150)));

    // Unrolled picks bake Direct(None) / a zero ask; availability gates those options off
    let option_give_potion = [
        Effect {
            kind: EffectKind::PotionDiscard,
            id_source: None,
            target: Target::Direct(id_potion),
        },
        RELIC_REWARD,
        EVENT_CONSUME_EFFECT,
    ];
    let option_give_gold = [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(gold_ask.unwrap_or(0)),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        RELIC_REWARD,
        EVENT_CONSUME_EFFECT,
    ];
    let option_give_card = [
        Effect {
            kind: EffectKind::CardPurge,
            id_source: None,
            target: Target::Direct(id_card),
        },
        RELIC_REWARD,
        EVENT_CONSUME_EFFECT,
    ];

    let options = [
        make_entity_event_option(
            "[Give Potion] Lose the offered potion. Obtain a random relic.",
            &option_give_potion,
        ),
        make_entity_event_option(
            "[Give Gold] Lose the asked gold. Obtain a random relic.",
            &option_give_gold,
        ),
        make_entity_event_option(
            "[Give Card] Lose the offered card. Obtain a random relic.",
            &option_give_card,
        ),
        make_entity_event_option("[Attack] Nothing happens.", OPTION_ATTACK),
    ];
    let id_event_options = bake_options(state, &options);
    (
        EventKind::WeMeetAgain {
            id_card,
            id_potion,
            gold_ask,
        },
        id_event_options,
    )
}

pub fn option_available(
    state: &GameState,
    id_card: Option<usize>,
    id_potion: Option<usize>,
    gold_ask: Option<u16>,
    idx: usize,
) -> bool {
    match idx {
        // Rolled ids are validated at use: the pick must still be owned
        0 => id_potion.is_some_and(|id| state.id_potions.contains(&Some(id))),
        1 => gold_ask.is_some() && state.entities[state.id_character].character_gold >= 50,
        2 => id_card.is_some_and(|id| state.id_card_deck.contains(&id)),
        3 => true,
        _ => unreachable!("We meet again option out of range: {idx}"),
    }
}
