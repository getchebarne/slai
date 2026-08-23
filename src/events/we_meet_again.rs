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
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::utils::card_is_non_basic_non_curse;

const RELIC_REWARD: Effect = Effect {
    kind: EffectKind::RelicGrantRandom { tier: None },
    id_source: None,
    target: Target::Direct(None),
};

const OPTION_ATTACK: &[Effect] = &[EVENT_CONSUME_EFFECT];

// The offer options consume the staked roll; only the gold ask varies per visit
const OPTION_GIVE_POTION: [Effect; 3] = [
    Effect {
        kind: EffectKind::PotionDiscard,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::EventRollPotion,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    },
    RELIC_REWARD,
    EVENT_CONSUME_EFFECT,
];

fn option_give_gold(ask: u16) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(ask),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        RELIC_REWARD,
        EVENT_CONSUME_EFFECT,
    ]
}

const OPTION_GIVE_CARD: [Effect; 3] = [
    Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::EventRollCard,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    },
    RELIC_REWARD,
    EVENT_CONSUME_EFFECT,
];

// Catalog rows, option-table order; the ask enumerates its reachable values —
// 0 (unrolled, gated off but visible) and every rollable 50..=150
pub fn catalog() -> Vec<Vec<Effect>> {
    let mut rows = Vec::with_capacity(105);
    rows.push(OPTION_GIVE_POTION.to_vec());
    rows.push(option_give_gold(0).to_vec());
    for ask in WE_MEET_AGAIN_GOLD_ASK_MIN..=WE_MEET_AGAIN_GOLD_ASK_MAX {
        rows.push(option_give_gold(ask).to_vec());
    }
    rows.push(OPTION_GIVE_CARD.to_vec());
    rows.push(OPTION_ATTACK.to_vec());
    rows
}

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

    // Unrolled offers leave their roll vec empty / a zero ask; availability gates them off
    state.event.id_roll_card.extend(id_card);
    state.event.id_roll_potion.extend(id_potion);
    let effects_gold = option_give_gold(gold_ask.unwrap_or(0));
    let options = [
        make_event_option_template(
            "[Give Potion] Lose the offered potion. Obtain a random relic.",
            &OPTION_GIVE_POTION,
        ),
        make_event_option_template(
            "[Give Gold] Lose the asked gold. Obtain a random relic.",
            &effects_gold,
        ),
        make_event_option_template(
            "[Give Card] Lose the offered card. Obtain a random relic.",
            &OPTION_GIVE_CARD,
        ),
        make_event_option_template("[Attack] Nothing happens.", OPTION_ATTACK),
    ];
    bake_options(state, &options)
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
        // Staked rolls are validated at use: the pick must still be owned
        0 => state
            .event
            .id_roll_potion
            .first()
            .is_some_and(|&id| state.id_potions.contains(&Some(id))),
        1 => baked_gold_ask(state) > 0 && gold >= WE_MEET_AGAIN_GOLD_ASK_MIN,
        2 => state
            .event
            .id_roll_card
            .first()
            .is_some_and(|&id| state.id_card_deck.contains(&id)),
        3 => true,
        _ => unreachable!("We meet again option out of range: {idx}"),
    }
}
