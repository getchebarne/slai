use rand::Rng;

use crate::consts::WE_MEET_AGAIN_GOLD_ASK_MAX;
use crate::consts::WE_MEET_AGAIN_GOLD_ASK_MIN;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::bake_option_entities;
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
pub fn spawn_event_we_meet_again(state: &mut GameState) -> Vec<usize> {
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

    // Gold ask capped by holdings; unrolled (option unavailable) below the minimum
    let gold = state.entities[state.id_character].character_gold;
    let gold_ask = (gold >= WE_MEET_AGAIN_GOLD_ASK_MIN).then(|| {
        state
            .rng
            .random_range(WE_MEET_AGAIN_GOLD_ASK_MIN..=gold.min(WE_MEET_AGAIN_GOLD_ASK_MAX))
    });

    // Unrolled picks bake Direct(None) / a zero ask; availability gates those options off
    let option_give_potion = [
        Effect {
            kind: EffectKind::PotionDiscard,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::EventPotionPicks,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::Single,
            },
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
            target: Target::Resolve {
                candidate_pool: CandidatePool::EventCardPicks,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::Single,
            },
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
    state.event.id_card_picks.extend(id_card);
    state.event.id_potion_picks.extend(id_potion);
    bake_option_entities(state, &options)
}

const IDX_OPTION_GIVE_GOLD: usize = 1;

// The [Give Gold] option's baked ask; 0 = unrolled (spawned below the minimum)
fn baked_gold_ask(state: &GameState) -> u16 {
    let id_option = state.event.id_event_options[IDX_OPTION_GIVE_GOLD];
    match state.entities[id_option].event_option_effects[0].kind {
        EffectKind::GoldDelta {
            amount: Amount::Absolute(ask),
            ..
        } => ask,
        _ => unreachable!("[Give Gold] leads with the baked GoldDelta"),
    }
}

pub fn option_available(state: &GameState, idx: usize) -> bool {
    let gold = state.entities[state.id_character].character_gold;
    match idx {
        // Staged picks are validated at use: the pick must still be owned
        0 => state
            .event
            .id_potion_picks
            .first()
            .is_some_and(|&id| state.id_potions.contains(&Some(id))),
        1 => baked_gold_ask(state) > 0 && gold >= WE_MEET_AGAIN_GOLD_ASK_MIN,
        2 => state
            .event
            .id_card_picks
            .first()
            .is_some_and(|&id| state.id_card_deck.contains(&id)),
        3 => true,
        _ => unreachable!("We meet again option out of range: {idx}"),
    }
}
