use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EFFECT_DECK_PURGE_PICK;
use crate::events::EFFECT_DECK_TRANSFORM_PICK_TWO;
use crate::events::EFFECT_DECK_UPGRADE_PICK;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::deck_has_purgeable;
use crate::events::deck_has_upgradable;
use crate::events::make_entity_event_option;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::utils::card_is_transformable;

pub const DESIGNER_COST_ADJUST: u16 = 40;
pub const DESIGNER_COST_ADJUST_A15: u16 = 50;
pub const DESIGNER_COST_CLEANUP: u16 = 60;
pub const DESIGNER_COST_CLEANUP_A15: u16 = 75;
pub const DESIGNER_COST_FULL: u16 = 90;
pub const DESIGNER_COST_FULL_A15: u16 = 110;

const fn gold_loss(amount: u16) -> Effect {
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
            target: Target::Direct(None),
        },
        EVENT_CONSUME_EFFECT,
    ]
}

// One paid shape covers four options; availability keys on the payload
const fn paid_option(cost: u16, middle: Effect) -> [Effect; 3] {
    [gold_loss(cost), middle, EVENT_CONSUME_EFFECT]
}
const fn full_service(cost: u16) -> [Effect; 4] {
    [
        gold_loss(cost),
        EFFECT_DECK_PURGE_PICK,
        upgrade_random(1),
        EVENT_CONSUME_EFFECT,
    ]
}

const ADJ_ONE_BASE: [Effect; 3] = paid_option(DESIGNER_COST_ADJUST, EFFECT_DECK_UPGRADE_PICK);
const ADJ_TWO_BASE: [Effect; 3] = paid_option(DESIGNER_COST_ADJUST, upgrade_random(2));
const CLEAN_REMOVE_BASE: [Effect; 3] = paid_option(DESIGNER_COST_CLEANUP, EFFECT_DECK_PURGE_PICK);
const CLEAN_TRANSFORM_BASE: [Effect; 3] =
    paid_option(DESIGNER_COST_CLEANUP, EFFECT_DECK_TRANSFORM_PICK_TWO);
const FULL_BASE: [Effect; 4] = full_service(DESIGNER_COST_FULL);
const PUNCH_BASE: [Effect; 2] = punch(3);
const ADJ_ONE_A15: [Effect; 3] = paid_option(DESIGNER_COST_ADJUST_A15, EFFECT_DECK_UPGRADE_PICK);
const ADJ_TWO_A15: [Effect; 3] = paid_option(DESIGNER_COST_ADJUST_A15, upgrade_random(2));
const CLEAN_REMOVE_A15: [Effect; 3] =
    paid_option(DESIGNER_COST_CLEANUP_A15, EFFECT_DECK_PURGE_PICK);
const CLEAN_TRANSFORM_A15: [Effect; 3] =
    paid_option(DESIGNER_COST_CLEANUP_A15, EFFECT_DECK_TRANSFORM_PICK_TWO);
const FULL_A15: [Effect; 4] = full_service(DESIGNER_COST_FULL_A15);
const PUNCH_A15: [Effect; 2] = punch(5);

static OPTIONS_BASE: &[Entity] = &[
    make_entity_event_option("[Adjustments] Lose 40 Gold. Upgrade a card.", &ADJ_ONE_BASE),
    make_entity_event_option(
        "[Adjustments] Lose 40 Gold. Upgrade 2 random cards.",
        &ADJ_TWO_BASE,
    ),
    make_entity_event_option(
        "[Clean Up] Lose 60 Gold. Remove a card from your deck.",
        &CLEAN_REMOVE_BASE,
    ),
    make_entity_event_option(
        "[Clean Up] Lose 60 Gold. Transform 2 cards.",
        &CLEAN_TRANSFORM_BASE,
    ),
    make_entity_event_option(
        "[Full Service] Lose 90 Gold. Remove a card, then upgrade a random card.",
        &FULL_BASE,
    ),
    make_entity_event_option("[Punch] Lose 3 HP.", &PUNCH_BASE),
];
static OPTIONS_A15: &[Entity] = &[
    make_entity_event_option("[Adjustments] Lose 50 Gold. Upgrade a card.", &ADJ_ONE_A15),
    make_entity_event_option(
        "[Adjustments] Lose 50 Gold. Upgrade 2 random cards.",
        &ADJ_TWO_A15,
    ),
    make_entity_event_option(
        "[Clean Up] Lose 75 Gold. Remove a card from your deck.",
        &CLEAN_REMOVE_A15,
    ),
    make_entity_event_option(
        "[Clean Up] Lose 75 Gold. Transform 2 cards.",
        &CLEAN_TRANSFORM_A15,
    ),
    make_entity_event_option(
        "[Full Service] Lose 110 Gold. Remove a card, then upgrade a random card.",
        &FULL_A15,
    ),
    make_entity_event_option("[Punch] Lose 5 HP.", &PUNCH_A15),
];

pub fn options(ascension: u8) -> &'static [Entity] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}

pub fn option_available(
    state: &GameState,
    adjust_upgrades_one: bool,
    cleanup_removes: bool,
    idx: usize,
) -> bool {
    let gold = state.entities[state.id_character].character_gold;
    let (adjust_cost, cleanup_cost, full_cost) = if state.ascension < 15 {
        (
            DESIGNER_COST_ADJUST,
            DESIGNER_COST_CLEANUP,
            DESIGNER_COST_FULL,
        )
    } else {
        (
            DESIGNER_COST_ADJUST_A15,
            DESIGNER_COST_CLEANUP_A15,
            DESIGNER_COST_FULL_A15,
        )
    };
    match idx {
        0 => adjust_upgrades_one && gold >= adjust_cost && deck_has_upgradable(state),
        1 => !adjust_upgrades_one && gold >= adjust_cost && deck_has_upgradable(state),
        2 => cleanup_removes && gold >= cleanup_cost && deck_has_purgeable(state),
        3 => {
            !cleanup_removes
                && gold >= cleanup_cost
                && state
                    .id_deck
                    .iter()
                    .filter(|&&id| card_is_transformable(&state.entities[id]))
                    .nth(1)
                    .is_some()
        }
        4 => gold >= full_cost && deck_has_purgeable(state),
        _ => true,
    }
}
