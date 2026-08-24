use rand::Rng;

use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_DECK_PURGE_PICK_1;
use crate::events::EFFECT_DECK_TRANSFORM_PICK_2;
use crate::events::EFFECT_DECK_UPGRADE_PICK_1;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::deck_has_purgeable;
use crate::events::deck_has_two_transformable;
use crate::events::deck_has_upgradable;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::DeltaSign;

const COST_ADJUST: u16 = 40;
const COST_ADJUST_A15: u16 = 50;
const COST_CLEANUP: u16 = 60;
const COST_CLEANUP_A15: u16 = 75;
const COST_FULL: u16 = 90;
const COST_FULL_A15: u16 = 110;

const fn gold_delta(amount: u16) -> Effect {
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(amount),
        },
        id_source: None,
        target: Target::Direct(None),
    }
}

const fn upgrade_random(count: u8) -> Effect {
    Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter: CandidateFilter::Upgradeable,
            selection_kind: SelectionKind::Random { count },
        },
    }
}

const fn punch(damage: u16) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(damage),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        EFFECT_EVENT_CONSUME,
    ]
}

// One paid shape covers four options; availability keys on the payload
const fn paid_option(cost: u16, middle: Effect) -> [Effect; 3] {
    [gold_delta(cost), middle, EFFECT_EVENT_CONSUME]
}
const fn full_service(cost: u16) -> [Effect; 4] {
    [
        gold_delta(cost),
        EFFECT_DECK_PURGE_PICK_1,
        upgrade_random(1),
        EFFECT_EVENT_CONSUME,
    ]
}

// Adjustments: upgrade a chosen Card
const ADJUST_ONE_BASE: [Effect; 3] = paid_option(COST_ADJUST, EFFECT_DECK_UPGRADE_PICK_1);

// Adjustments: upgrade two random Cards
const ADJUST_TWO_BASE: [Effect; 3] = paid_option(COST_ADJUST, upgrade_random(2));

// Clean Up: purge a chosen Card
const CLEAN_REMOVE_BASE: [Effect; 3] = paid_option(COST_CLEANUP, EFFECT_DECK_PURGE_PICK_1);

// Clean Up: transform two chosen Cards
const CLEAN_TRANSFORM_BASE: [Effect; 3] = paid_option(COST_CLEANUP, EFFECT_DECK_TRANSFORM_PICK_2);

// Full Service: upgrade and purge
const FULL_BASE: [Effect; 4] = full_service(COST_FULL);

// Punch: take the hit for free
const PUNCH_BASE: [Effect; 2] = punch(3);

// Adjustments at A15+: dearer
const ADJUST_ONE_A15: [Effect; 3] = paid_option(COST_ADJUST_A15, EFFECT_DECK_UPGRADE_PICK_1);

// Adjustments at A15+: dearer
const ADJUST_TWO_A15: [Effect; 3] = paid_option(COST_ADJUST_A15, upgrade_random(2));

// Clean Up at A15+: dearer
const CLEAN_REMOVE_A15: [Effect; 3] = paid_option(COST_CLEANUP_A15, EFFECT_DECK_PURGE_PICK_1);

// Clean Up at A15+: dearer
const CLEAN_TRANSFORM_A15: [Effect; 3] =
    paid_option(COST_CLEANUP_A15, EFFECT_DECK_TRANSFORM_PICK_2);

// Full Service at A15+: dearer
const FULL_A15: [Effect; 4] = full_service(COST_FULL_A15);

// Punch at A15+: hits harder
const PUNCH_A15: [Effect; 2] = punch(5);

const EOT_ADJUST_ONE_BASE: EventOptionTemplate = make_event_option_template(&ADJUST_ONE_BASE);
const EOT_ADJUST_TWO_BASE: EventOptionTemplate = make_event_option_template(&ADJUST_TWO_BASE);
const EOT_CLEAN_REMOVE_BASE: EventOptionTemplate = make_event_option_template(&CLEAN_REMOVE_BASE);
const EOT_CLEAN_TRANSFORM_BASE: EventOptionTemplate =
    make_event_option_template(&CLEAN_TRANSFORM_BASE);
const EOT_FULL_BASE: EventOptionTemplate = make_event_option_template(&FULL_BASE);
const EOT_PUNCH_BASE: EventOptionTemplate = make_event_option_template(&PUNCH_BASE);
const EOT_ADJUST_ONE_A15: EventOptionTemplate = make_event_option_template(&ADJUST_ONE_A15);
const EOT_ADJUST_TWO_A15: EventOptionTemplate = make_event_option_template(&ADJUST_TWO_A15);
const EOT_CLEAN_REMOVE_A15: EventOptionTemplate = make_event_option_template(&CLEAN_REMOVE_A15);
const EOT_CLEAN_TRANSFORM_A15: EventOptionTemplate =
    make_event_option_template(&CLEAN_TRANSFORM_A15);
const EOT_FULL_A15: EventOptionTemplate = make_event_option_template(&FULL_A15);
const EOT_PUNCH_A15: EventOptionTemplate = make_event_option_template(&PUNCH_A15);

// The six distinct services; a spawn shows one adjust variant, one cleanup
// variant, then the two fixed options
const EOTS_BASE: [EventOptionTemplate; 6] = [
    EOT_ADJUST_ONE_BASE,
    EOT_ADJUST_TWO_BASE,
    EOT_CLEAN_REMOVE_BASE,
    EOT_CLEAN_TRANSFORM_BASE,
    EOT_FULL_BASE,
    EOT_PUNCH_BASE,
];
const EOTS_A15: [EventOptionTemplate; 6] = [
    EOT_ADJUST_ONE_A15,
    EOT_ADJUST_TWO_A15,
    EOT_CLEAN_REMOVE_A15,
    EOT_CLEAN_TRANSFORM_A15,
    EOT_FULL_A15,
    EOT_PUNCH_A15,
];

const _: () = assert!(EOTS_BASE.len() == 6 && EOTS_A15.len() == 6);

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 {
        &EOTS_BASE
    } else {
        &EOTS_A15
    }
}

// Two coin flips pick the adjust and cleanup variants; both index the catalog
pub fn spawn(state: &mut GameState) -> Vec<usize> {
    // Rolls
    let adjust_upgrades_one = state.rng.random_bool(0.5);
    let cleanup_removes = state.rng.random_bool(0.5);

    // Get EOT catalog and bake options
    let eots = catalog(state.ascension);
    let options = [
        eots[if adjust_upgrades_one { 0 } else { 1 }],
        eots[if cleanup_removes { 2 } else { 3 }],
        eots[4],
        eots[5],
    ];
    bake_options(state, &options)
}

pub fn option_available(state: &GameState, idx: usize) -> bool {
    let gold = state.entities[state.id_character].character_gold;
    let (adjust_cost, cleanup_cost, full_cost) = if state.ascension < 15 {
        (COST_ADJUST, COST_CLEANUP, COST_FULL)
    } else {
        (COST_ADJUST_A15, COST_CLEANUP_A15, COST_FULL_A15)
    };
    match idx {
        0 => gold >= adjust_cost && deck_has_upgradable(state),
        1 => {
            // The baked variant carries its own requirement: the remove pick
            // needs a purgeable Card, the transform pair needs two
            let id_option = state.event.id_event_options[idx];
            match state.entities[id_option].event_option_effects[1].kind {
                EffectKind::CardPurge => gold >= cleanup_cost && deck_has_purgeable(state),
                EffectKind::CardTransform { .. } => {
                    gold >= cleanup_cost && deck_has_two_transformable(state)
                }
                kind => unreachable!("Designer cleanup option with unexpected effect: {kind:?}"),
            }
        }
        2 => gold >= full_cost && deck_has_purgeable(state),
        _ => true,
    }
}
