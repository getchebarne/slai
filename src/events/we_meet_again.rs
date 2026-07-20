use rand::Rng;

use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventPayload;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::CardRarity;
use crate::types::DeltaSign;

const RELIC_REWARD: Effect = Effect {
    kind: EffectKind::RelicGrantRandom,
    id_source: None,
    target: Target::Direct(None),
};

pub const LABELS: &[&str] = &[
    "[Give Potion] Lose the offered potion. Obtain a random relic.",
    "[Give Gold] Lose the asked gold. Obtain a random relic.",
    "[Give Card] Lose the offered card. Obtain a random relic.",
    "[Attack] Nothing happens.",
];

pub fn spawn_event_we_meet_again(state: &mut GameState) -> EventPayload {
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
    let id_card =
        (!eligible.is_empty()).then(|| eligible[state.rng.random_range(0..eligible.len())]);

    // Potion offer: uniform among occupied belt slots
    let slotted: Vec<usize> = state.id_potions.iter().flatten().copied().collect();
    let id_potion =
        (!slotted.is_empty()).then(|| slotted[state.rng.random_range(0..slotted.len())]);

    // Gold ask: 50..=150, capped by holdings; unrolled (option unavailable) below 50
    let gold = state.entities[state.id_character].character_gold;
    let gold_ask = (gold >= 50).then(|| state.rng.random_range(50..=gold.min(150)));

    EventPayload::WeMeetAgain {
        id_card,
        id_potion,
        gold_ask,
    }
}

pub fn push_option_effects(
    buf: &mut Vec<Effect>,
    id_card: Option<usize>,
    id_potion: Option<usize>,
    gold_ask: Option<u16>,
    idx: usize,
) {
    // Unrolled picks build as Direct(None)/0: the snapshot still renders gated-out
    // options, and availability keeps the action path off them
    match idx {
        0 => {
            buf.push(Effect {
                kind: EffectKind::PotionDiscard,
                id_source: None,
                target: Target::Direct(id_potion),
            });
            buf.push(RELIC_REWARD);
            buf.push(EVENT_CONSUME_EFFECT);
        }
        1 => {
            buf.push(Effect {
                kind: EffectKind::GoldDelta {
                    sign: DeltaSign::Loss,
                    amount: Amount::Absolute(gold_ask.unwrap_or(0)),
                },
                id_source: None,
                target: Target::Direct(None),
            });
            buf.push(RELIC_REWARD);
            buf.push(EVENT_CONSUME_EFFECT);
        }
        2 => {
            buf.push(Effect {
                kind: EffectKind::CardPurge,
                id_source: None,
                target: Target::Direct(id_card),
            });
            buf.push(RELIC_REWARD);
            buf.push(EVENT_CONSUME_EFFECT);
        }
        3 => buf.push(EVENT_CONSUME_EFFECT),
        _ => unreachable!("we meet again option out of range: {idx}"),
    }
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
        2 => id_card.is_some_and(|id| state.id_deck.contains(&id)),
        3 => true,
        _ => unreachable!("we meet again option out of range: {idx}"),
    }
}
