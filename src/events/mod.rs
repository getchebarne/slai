mod addict;
mod back_to_basics;
mod beggar;
mod big_fish;
mod bonfire_spirits;
mod colosseum;
mod cursed_tome;
mod dead_adventurer;
mod designer;
mod drug_dealer;
mod duplicator;
mod face_trader;
mod forgotten_altar;
mod ghosts;
mod golden_idol;
mod golden_shrine;
mod knowing_skull;
mod living_wall;
mod masked_bandits;
mod mushrooms;
mod neow;
mod nest;
mod nloth;
mod ominous_forge;
mod purifier;
mod scrap_ooze;
mod shining_light;
mod the_cleric;
mod the_divine_fountain;
mod the_joust;
mod the_lab;
mod the_library;
mod the_mausoleum;
mod the_ssssserpent;
mod the_woman_in_blue;
mod transmogrifier;
mod upgrade_shrine;
mod vampires;
mod we_meet_again;
mod wheel_of_change;
mod wing_statue;
mod world_of_goop;

use crate::consts::MAX_EFFECTS_PER_EVENT_OPTION;
use crate::consts::RELIC_TIER_TH_COMMON;
use crate::consts::RELIC_TIER_TH_UNCOMMON;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::EFFECT_ZERO;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicPick;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::ENTITY_ZERO;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::types::Event;
use crate::types::EventName;
use crate::types::event_reset;
use crate::utils::card_is_non_basic_non_curse;
use crate::utils::card_is_purgeable;
use crate::utils::card_is_transformable;
use crate::utils::card_is_upgradable;
use crate::utils::push_entity;
use rand::Rng;

pub use beggar::BEGGAR_COST_PURGE;
pub use the_joust::JOUST_OWNER_WIN_CHANCE;
pub use the_joust::JOUST_PAYOUT_MURDERER;
pub use the_joust::JOUST_PAYOUT_OWNER;
pub use the_joust::JOUST_STAKE;

pub const EVENT_CONSUME_EFFECT: Effect = Effect {
    kind: EffectKind::EventConsume,
    id_source: None,
    target: Target::Direct(None),
};

pub const EFFECT_DECK_PURGE_PICK_1: Effect = Effect {
    kind: EffectKind::CardPurge,
    id_source: None,
    target: Target::Resolve {
        candidate_pool: CandidatePool::Deck,
        filter: CandidateFilter::Purgeable,
        selection_kind: SelectionKind::Input { count: 1 },
    },
};

pub const EFFECT_DECK_UPGRADE_PICK_1: Effect = Effect {
    kind: EffectKind::CardUpgrade,
    id_source: None,
    target: Target::Resolve {
        candidate_pool: CandidatePool::Deck,
        filter: CandidateFilter::Upgradeable,
        selection_kind: SelectionKind::Input { count: 1 },
    },
};

pub const EFFECT_DECK_TRANSFORM_PICK_1: Effect = Effect {
    kind: EffectKind::CardTransform { upgraded: false },
    id_source: None,
    target: Target::Resolve {
        candidate_pool: CandidatePool::Deck,
        filter: CandidateFilter::Transformable,
        selection_kind: SelectionKind::Input { count: 1 },
    },
};

// Transform two chosen cards (Designer's Clean Up, Drug Dealer)
pub const EFFECT_DECK_TRANSFORM_PICK_2: Effect = Effect {
    kind: EffectKind::CardTransform { upgraded: false },
    id_source: None,
    target: Target::Resolve {
        candidate_pool: CandidatePool::Deck,
        filter: CandidateFilter::Transformable,
        selection_kind: SelectionKind::Input { count: 2 },
    },
};

pub const EVENT_ADVANCE_EFFECT: Effect = Effect {
    kind: EffectKind::EventAdvanceState { delta: 1 },
    id_source: None,
    target: Target::Direct(None),
};

// Immutable option definition; bake instances it into the arena
#[derive(Debug, Clone, Copy)]
pub struct EventOptionTemplate {
    pub label: &'static str,
    pub effects: &'static [Effect],
}

pub const fn make_event_option_template(
    label: &'static str,
    effects: &'static [Effect],
) -> EventOptionTemplate {
    assert!(effects.len() <= MAX_EFFECTS_PER_EVENT_OPTION);
    EventOptionTemplate { label, effects }
}

// The leave option and the three deck-pick effects every shrine-shaped event repeats
pub const OPTION_LEAVE: EventOptionTemplate =
    make_event_option_template("[Leave] Nothing happens.", &[EVENT_CONSUME_EFFECT]);

// Bakes the options into the arena; processors may mutate baked amounts mid-event
pub fn spawn_event(state: &mut GameState, name: EventName) -> Vec<usize> {
    // Clear last visit's staged picks before this spawn stakes its own
    event_reset(&mut state.event);
    let ascension = state.ascension;
    match name {
        // Rolled option table: pick one of the catalog's four variants
        EventName::Designer => {
            let options = designer::options(
                ascension,
                state.rng.random_bool(0.5),
                state.rng.random_bool(0.5),
            );
            return bake_options(state, options);
        }
        // Custom spawns: staged picks and per-visit rolls
        EventName::Vampires => return vampires::spawn_event_vampires(state),
        EventName::ForgottenAltar => return forgotten_altar::spawn_event_forgotten_altar(state),
        EventName::Nloth => return nloth::spawn_event_nloth(state),
        EventName::Neow => return neow::spawn_event_neow(state),
        EventName::WeMeetAgain => return we_meet_again::spawn_event_we_meet_again(state),
        _ => {}
    }
    // Single table source, shared with the FFI catalog
    let tables = options_catalog(name, ascension);
    debug_assert!(
        tables.len() == 1,
        "static-table events have exactly one table"
    );
    bake_options(state, tables[0])
}

// State-free option tables; WMA is excluded (per-visit roll, bounds in consts)
pub fn options_catalog(name: EventName, ascension: u8) -> Vec<&'static [EventOptionTemplate]> {
    match name {
        EventName::BigFish => vec![big_fish::OPTIONS],
        EventName::TheCleric => vec![the_cleric::options(ascension)],
        EventName::Duplicator => vec![duplicator::OPTIONS],
        EventName::GoldenShrine => vec![golden_shrine::options(ascension)],
        EventName::GoldenIdol => vec![golden_idol::options(ascension)],
        EventName::WingStatue => vec![wing_statue::OPTIONS],
        EventName::WorldOfGoop => vec![world_of_goop::options(ascension)],
        EventName::LivingWall => vec![living_wall::OPTIONS],
        EventName::Purifier => vec![purifier::OPTIONS],
        EventName::ScrapOoze => vec![scrap_ooze::options(ascension)],
        EventName::ShiningLight => vec![shining_light::options(ascension)],
        EventName::TheSsssserpent => vec![the_ssssserpent::options(ascension)],
        EventName::Transmogrifier => vec![transmogrifier::OPTIONS],
        EventName::UpgradeShrine => vec![upgrade_shrine::OPTIONS],
        EventName::TheDivineFountain => vec![the_divine_fountain::OPTIONS],
        EventName::TheLab => vec![the_lab::options(ascension)],
        EventName::TheWomanInBlue => vec![the_woman_in_blue::options(ascension)],
        EventName::WheelOfChange => vec![wheel_of_change::OPTIONS],
        EventName::BonfireSpirits => vec![bonfire_spirits::OPTIONS],
        EventName::OminousForge => vec![ominous_forge::OPTIONS],
        EventName::FaceTrader => vec![face_trader::options(ascension)],
        EventName::Addict => vec![addict::OPTIONS],
        EventName::Beggar => vec![beggar::OPTIONS],
        EventName::Ghosts => vec![ghosts::options(ascension)],
        EventName::BackToBasics => vec![back_to_basics::OPTIONS],
        EventName::MaskedBandits => vec![masked_bandits::OPTIONS],
        EventName::TheJoust => vec![the_joust::OPTIONS],
        EventName::TheLibrary => vec![the_library::options(ascension)],
        EventName::TheMausoleum => vec![the_mausoleum::OPTIONS],
        EventName::Vampires => vec![vampires::OPTIONS],
        EventName::Nest => vec![nest::options(ascension)],
        EventName::CursedTome => vec![cursed_tome::options(ascension)],
        EventName::DrugDealer => vec![drug_dealer::OPTIONS],
        EventName::ForgottenAltar => vec![forgotten_altar::options(ascension)],
        EventName::Nloth => vec![nloth::OPTIONS],
        EventName::Colosseum => vec![colosseum::OPTIONS],
        EventName::Designer => vec![
            designer::options(ascension, false, false),
            designer::options(ascension, false, true),
            designer::options(ascension, true, false),
            designer::options(ascension, true, true),
        ],
        EventName::KnowingSkull => vec![knowing_skull::OPTIONS],
        EventName::Mushrooms => vec![mushrooms::OPTIONS],
        EventName::DeadAdventurer => vec![dead_adventurer::OPTIONS],
        // Neow's menus vary per ascension tier; all 37 templates enumerate here
        EventName::Neow => neow::tables(ascension).to_vec(),
        EventName::WeMeetAgain => vec![],
    }
}

// One Entity per option, instanced into the arena at spawn
fn bake_options(state: &mut GameState, templates: &[EventOptionTemplate]) -> Vec<usize> {
    let mut id_event_options = Vec::with_capacity(templates.len());
    for template in templates {
        let option = make_entity_event_option(template.label, template.effects);
        id_event_options.push(push_entity(&mut state.entities, option));
    }
    id_event_options
}

// We Meet Again builds its options from per-visit rolls; it bakes Entities directly
fn bake_option_entities(state: &mut GameState, options: &[Entity]) -> Vec<usize> {
    let mut id_event_options = Vec::with_capacity(options.len());
    for &option in options {
        id_event_options.push(push_entity(&mut state.entities, option));
    }
    id_event_options
}

// Loot an event stakes on the fight it hosts, translated into reward effects by
// the combat's end; None resumes the event unpaid (Colosseum's first bout)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventLoot {
    pub gold: Option<Amount>,
    pub relics: [Option<RelicPick>; 2],
}

pub fn fight_loot(event: &Event) -> Option<EventLoot> {
    match event.name {
        EventName::Mushrooms => Some(mushrooms::FIGHT_LOOT),
        EventName::MaskedBandits => Some(masked_bandits::FIGHT_LOOT),
        EventName::Colosseum => match event.stage {
            0 | 1 => None,
            _ => Some(colosseum::FIGHT_LOOT_NOBS),
        },
        EventName::DeadAdventurer => {
            let gold_extra = !event.found_gold as u16 * 30;
            Some(EventLoot {
                gold: Some(Amount::Range {
                    min: 25 + gold_extra,
                    max: 35 + gold_extra,
                }),
                relics: [
                    (!event.found_relic).then_some(RelicPick::Thresholds {
                        th_common: RELIC_TIER_TH_COMMON,
                        th_uncommon: RELIC_TIER_TH_UNCOMMON,
                    }),
                    None,
                ],
            })
        }
        // Every fight-hosting event must claim an arm; None is reserved for
        // deliberate unpaid bouts (Colosseum's first)
        name => unreachable!("fight over a non-fight event: {name:?}"),
    }
}

// Per-event availability checks
pub fn event_option_available(state: &GameState, idx: usize) -> bool {
    match state.event.name {
        EventName::BigFish
        | EventName::Duplicator
        | EventName::GoldenShrine
        | EventName::WorldOfGoop
        | EventName::TheSsssserpent
        | EventName::TheDivineFountain
        | EventName::TheLab
        | EventName::TheWomanInBlue
        | EventName::WheelOfChange
        | EventName::BonfireSpirits
        | EventName::FaceTrader
        | EventName::Mushrooms
        | EventName::Neow
        | EventName::Ghosts
        | EventName::MaskedBandits
        | EventName::TheJoust
        | EventName::TheLibrary
        | EventName::TheMausoleum
        | EventName::KnowingSkull
        | EventName::DeadAdventurer
        | EventName::Nest
        | EventName::Nloth => true,
        EventName::Addict => addict::option_available(state, idx),
        EventName::Beggar => beggar::option_available(state, idx),
        EventName::BackToBasics => back_to_basics::option_available(state, idx),
        EventName::Vampires => vampires::option_available(state, idx),
        EventName::Colosseum => colosseum::option_available(state.event.stage, idx),
        EventName::Designer => designer::option_available(state, idx),
        EventName::DrugDealer => drug_dealer::option_available(state, idx),
        EventName::ForgottenAltar => forgotten_altar::option_available(state, idx),
        EventName::CursedTome => cursed_tome::option_available(state.event.stage, idx),
        EventName::TheCleric => the_cleric::option_available(state, idx),
        EventName::WingStatue => wing_statue::option_available(state, idx),
        EventName::LivingWall => living_wall::option_available(state, idx),
        EventName::Purifier => purifier::option_available(state, idx),
        EventName::ShiningLight => shining_light::option_available(state, idx),
        EventName::Transmogrifier => transmogrifier::option_available(state, idx),
        EventName::UpgradeShrine => upgrade_shrine::option_available(state, idx),
        EventName::OminousForge => ominous_forge::option_available(state, idx),
        EventName::GoldenIdol => golden_idol::option_available(state.event.stage, idx),
        EventName::ScrapOoze => scrap_ooze::option_available(state.event.stage, idx),
        EventName::WeMeetAgain => we_meet_again::option_available(state, idx),
    }
}

pub fn deck_has_two_transformable(state: &GameState) -> bool {
    state
        .id_card_deck
        .iter()
        .filter(|&&id| card_is_transformable(&state.entities[id]))
        .nth(1)
        .is_some()
}

pub fn deck_has_upgradable(state: &GameState) -> bool {
    state
        .id_card_deck
        .iter()
        .any(|&id| card_is_upgradable(&state.entities[id]))
}

pub fn deck_has_purgeable(state: &GameState) -> bool {
    state
        .id_card_deck
        .iter()
        .any(|&id| card_is_purgeable(&state.entities[id]))
}

pub fn deck_has_non_basic_non_curse(state: &GameState) -> bool {
    state
        .id_card_deck
        .iter()
        .any(|&id| card_is_non_basic_non_curse(&state.entities[id]))
}

pub fn deck_has_damage_card(state: &GameState, min_base: u16) -> bool {
    state
        .id_card_deck
        .iter()
        .any(|&id| card_has_damage_at_least(&state.entities[id], min_base))
}

fn card_has_damage_at_least(entity: &Entity, min_base: u16) -> bool {
    if entity.kind != EntityKind::Card {
        return false;
    }
    for effect in entity.card_effects[..entity.card_effects_len as usize].iter() {
        let amount = match effect.kind {
            EffectKind::DamagePhysical { amount, .. } => amount,
            EffectKind::DamagePhysicalIfPoisoned { amount } => amount,
            _ => 0,
        };
        if amount >= min_base {
            return true;
        }
    }
    false
}

// Mushrooms and Dead Adventurer are draw-gated in `draw_event` (floor 7+)
pub const POOL_EVENT_ACT1: &[EventName] = &[
    EventName::BigFish,
    EventName::Mushrooms,
    EventName::DeadAdventurer,
    EventName::TheCleric,
    EventName::GoldenIdol,
    EventName::WingStatue,
    EventName::WorldOfGoop,
    EventName::LivingWall,
    EventName::ScrapOoze,
    EventName::ShiningLight,
    EventName::TheSsssserpent,
];

// Shrines and one-time specials roll together at EVENT_SPECIAL_CHANCE
pub const POOL_EVENT_ACT1_SPECIAL: &[EventName] = &[
    EventName::GoldenShrine,
    EventName::Purifier,
    EventName::Transmogrifier,
    EventName::UpgradeShrine,
    EventName::TheDivineFountain,
    EventName::TheLab,
    EventName::TheWomanInBlue,
    EventName::WheelOfChange,
    EventName::BonfireSpirits,
    EventName::OminousForge,
    EventName::FaceTrader,
    EventName::WeMeetAgain,
];

// Beggar is draw-gated in `draw_event` (gold 75+)
pub const POOL_EVENT_ACT2: &[EventName] = &[
    EventName::Addict,
    EventName::BackToBasics,
    EventName::Beggar,
    EventName::Colosseum,
    EventName::CursedTome,
    EventName::DrugDealer,
    EventName::ForgottenAltar,
    EventName::Ghosts,
    EventName::MaskedBandits,
    EventName::Nest,
    EventName::TheLibrary,
    EventName::TheMausoleum,
    EventName::Vampires,
];

// Per-act shrines: dropped from the special pool and re-added fresh each Act
pub const POOL_EVENT_SHRINES: &[EventName] = &[
    EventName::GoldenShrine,
    EventName::Purifier,
    EventName::Transmogrifier,
    EventName::UpgradeShrine,
    EventName::WheelOfChange,
];

// One-time specials first reachable in act 2
pub const POOL_EVENT_ACT2_SPECIAL: &[EventName] = &[
    EventName::Designer,
    EventName::Duplicator,
    EventName::KnowingSkull,
    EventName::Nloth,
    EventName::TheJoust,
];

// Every per-act shrine must sit in the Act-1 special pool: the Act transition's
// retain-then-re-add link depends on it
const _: () = {
    let mut idx = 0;
    while idx < POOL_EVENT_SHRINES.len() {
        let mut found = false;
        let mut jdx = 0;
        while jdx < POOL_EVENT_ACT1_SPECIAL.len() {
            if POOL_EVENT_SHRINES[idx] as u8 == POOL_EVENT_ACT1_SPECIAL[jdx] as u8 {
                found = true;
            }
            jdx += 1;
        }
        assert!(found, "shrine missing from POOL_EVENT_ACT1_SPECIAL");
        idx += 1;
    }
};

// Per-act event pools: (act list, special-pool additions)
pub fn pools_for_act(act: u8) -> (&'static [EventName], &'static [EventName]) {
    match act {
        1 => (POOL_EVENT_ACT1, POOL_EVENT_ACT1_SPECIAL),
        2 => (POOL_EVENT_ACT2, POOL_EVENT_ACT2_SPECIAL),
        _ => unreachable!("no event pools for act {act}"),
    }
}

pub const fn make_entity_event_option(label: &'static str, effects: &[Effect]) -> Entity {
    assert!(effects.len() <= MAX_EFFECTS_PER_EVENT_OPTION);

    // Push Effects
    let mut effects_owned = [EFFECT_ZERO; MAX_EFFECTS_PER_EVENT_OPTION];
    let mut idx = 0;
    while idx < effects.len() {
        effects_owned[idx] = effects[idx];
        idx += 1;
    }

    // Make Entity
    Entity {
        kind: EntityKind::EventOption,
        event_option_label: label,
        event_option_effects: effects_owned,
        event_option_effects_len: effects.len() as u8,
        ..ENTITY_ZERO
    }
}
