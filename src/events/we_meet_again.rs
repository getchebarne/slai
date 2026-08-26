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
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
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

// Attack: refuse the offer
const OPTION_ATTACK: &[Effect] = &[EFFECT_EVENT_CONSUME];

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
    EFFECT_EVENT_CONSUME,
];

const fn option_give_gold(amount: u16) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(amount),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        RELIC_REWARD,
        EFFECT_EVENT_CONSUME,
    ]
}

// Give the staked Card
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
    EFFECT_EVENT_CONSUME,
];

// Catalog EOTs, option-table order; the ask enumerates its reachable values —
// 0 (unrolled, gated off but visible) and every rollable 50..=150
const GOLD_EOTS_LEN: usize = (WE_MEET_AGAIN_GOLD_ASK_MAX - WE_MEET_AGAIN_GOLD_ASK_MIN) as usize + 2;

static GOLD_EOTS: [[Effect; 3]; GOLD_EOTS_LEN] = {
    let mut eots = [option_give_gold(0); GOLD_EOTS_LEN];
    let mut idx = 1;
    while idx < GOLD_EOTS_LEN {
        eots[idx] = option_give_gold(WE_MEET_AGAIN_GOLD_ASK_MIN + idx as u16 - 1);
        idx += 1;
    }
    eots
};

// Catalog layout: give-Potion | every reachable gold ask | give-Card | attack
const IDX_GIVE_POTION: usize = 0;
const IDX_GIVE_GOLD: usize = IDX_GIVE_POTION + 1;
const IDX_GIVE_CARD: usize = IDX_GIVE_GOLD + GOLD_EOTS_LEN;
const IDX_ATTACK: usize = IDX_GIVE_CARD + 1;

// make_event_option_template() runs per row, so every catalog row is length-checked at compile time
static EOTS_BASE: [EventOptionTemplate; GOLD_EOTS_LEN + 3] = {
    let mut refs: [EventOptionTemplate; GOLD_EOTS_LEN + 3] =
        [make_event_option_template(OPTION_ATTACK); GOLD_EOTS_LEN + 3];
    refs[IDX_GIVE_POTION] = make_event_option_template(&OPTION_GIVE_POTION);
    let mut i = 0;
    while i < GOLD_EOTS_LEN {
        refs[IDX_GIVE_GOLD + i] = make_event_option_template(&GOLD_EOTS[i]);
        i += 1;
    }
    refs[IDX_GIVE_CARD] = make_event_option_template(&OPTION_GIVE_CARD);
    refs[IDX_ATTACK] = make_event_option_template(OPTION_ATTACK);
    refs
};

const _: () = assert!(EOTS_BASE.len() == 105);

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    &EOTS_BASE
}

// Spawn rolls the picks and the ask, then bakes them into the options;
// availability re-validates the picks at selection (the offered Potion can be
// drunk while standing here)
pub fn spawn(state: &mut GameState) -> Vec<usize> {
    // Card offer: uniform among non-Basic, non-Curse deck Cards
    let id_card_eligible: Vec<usize> = state
        .id_card_deck
        .iter()
        .copied()
        .filter(|&id| card_is_non_basic_non_curse(&state.entities[id]))
        .collect();
    let id_card = (!id_card_eligible.is_empty())
        .then(|| id_card_eligible[state.rng.random_range(0..id_card_eligible.len())]);

    // Potion offer: uniform among occupied belt slots
    let id_potion_eligible: Vec<usize> = state.id_potions.iter().flatten().copied().collect();
    let id_potion = (!id_potion_eligible.is_empty())
        .then(|| id_potion_eligible[state.rng.random_range(0..id_potion_eligible.len())]);

    // Gold ask capped by holdings; unrolled (option unavailable) below the minimum
    let gold = state.entities[state.id_character].character_gold;
    let gold_ask = (gold >= WE_MEET_AGAIN_GOLD_ASK_MIN).then(|| {
        state
            .rng
            .random_range(WE_MEET_AGAIN_GOLD_ASK_MIN..=gold.min(WE_MEET_AGAIN_GOLD_ASK_MAX))
    });

    // `extend` on an `Option` pushes only for `Some`, so an unrolled offer stays empty
    state.event.id_roll_card.extend(id_card);
    state.event.id_roll_potion.extend(id_potion);
    let eots = catalog(state.ascension);

    // GOLD_EOTS[0] is the unrolled 0; the rollable asks follow from MIN
    let idx_gold = match gold_ask {
        Some(ask) => IDX_GIVE_GOLD + 1 + (ask - WE_MEET_AGAIN_GOLD_ASK_MIN) as usize,
        None => IDX_GIVE_GOLD,
    };
    let options = [
        eots[IDX_GIVE_POTION],
        eots[idx_gold],
        eots[IDX_GIVE_CARD],
        eots[IDX_ATTACK],
    ];
    bake_options(state, &options)
}

// Slot in the spawned four-option menu, NOT a catalog index like IDX_GIVE_GOLD
const SLOT_GIVE_GOLD: usize = 1;

fn baked_gold_ask(state: &GameState) -> u16 {
    let id_option = state.event.id_event_options[SLOT_GIVE_GOLD];
    match state.entities[id_option].event_option_effects[0].kind {
        EffectKind::GoldDelta {
            amount: Amount::Absolute(ask),
            ..
        } => ask,
        _ => unreachable!("[Give Gold] leads with the baked GoldDelta"),
    }
}

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => state
            .event
            .id_roll_potion
            .first()
            .is_some_and(|&id| state.id_potions.contains(&Some(id))),
        // A rolled ask is always <= the gold held at spawn, and nothing
        // reachable from here spends gold, so affordability needs no re-check
        1 => baked_gold_ask(state) > 0,
        2 => state
            .event
            .id_roll_card
            .first()
            .is_some_and(|&id| state.id_card_deck.contains(&id)),
        3 => true,
        _ => unreachable!("We meet again option out of range: {idx}"),
    }
}
