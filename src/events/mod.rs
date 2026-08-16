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
use crate::types::EventName;
use crate::types::RelicName;
use crate::utils::card_is_non_basic_non_curse;
use crate::utils::card_is_purgeable;
use crate::utils::card_is_transformable;
use crate::utils::card_is_upgradable;
use crate::utils::push_entity;
use rand::Rng;

pub use beggar::BEGGAR_COST_PURGE;
use knowing_skull::KNOWING_SKULL_COST_START;
pub use knowing_skull::KNOWING_SKULL_GOLD;
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

// The leave option and the three deck-pick effects every shrine-shaped event repeats
pub const OPTION_LEAVE: Entity =
    make_entity_event_option("[Leave] Nothing happens.", &[EVENT_CONSUME_EFFECT]);

#[derive(Debug, Clone, Copy)]
pub enum EventKind {
    BigFish,
    TheCleric,
    Duplicator,
    GoldenShrine,
    WingStatue,
    WorldOfGoop,
    LivingWall,
    Purifier,
    ShiningLight,
    TheSsssserpent,
    Transmogrifier,
    UpgradeShrine,
    TheDivineFountain,
    TheLab,
    TheWomanInBlue,
    WheelOfChange,
    BonfireSpirits,
    OminousForge,
    FaceTrader,
    Mushrooms,
    GoldenIdol {
        stage: u8,
    },
    ScrapOoze {
        attempts: u8,
    },
    WeMeetAgain {
        id_card: Option<usize>,
        id_potion: Option<usize>,
        gold_ask: Option<u16>,
    },
    DeadAdventurer {
        found_gold: bool,
        found_nothing: bool,
        found_relic: bool,
        searches: u8,
    },
    Neow,
    Addict,
    Beggar,
    Ghosts,
    BackToBasics,
    MaskedBandits,
    TheJoust,
    TheLibrary,
    TheMausoleum,
    Vampires,
    Colosseum {
        stage: u8,
    },
    Designer,
    KnowingSkull {
        potion_cost_hp: u8,
        gold_cost_hp: u8,
        card_cost_hp: u8,
    },
    Nest,
    CursedTome {
        stage: u8,
    },
    DrugDealer,
    ForgottenAltar,
    Nloth {
        relic_first: RelicName,
        relic_second: RelicName,
    },
}

// Builds the event in one pass: entry rolls land in the kind, the matching
// option list bakes into the arena. Options encode only spawn-time state;
// mid-event dynamism lives in processors
pub fn spawn_event(state: &mut GameState, name: EventName) -> (EventKind, Vec<usize>) {
    let ascension = state.ascension;
    let (kind, options): (EventKind, &[Entity]) = match name {
        EventName::BigFish => (EventKind::BigFish, big_fish::OPTIONS),
        EventName::TheCleric => (EventKind::TheCleric, the_cleric::options(ascension)),
        EventName::Duplicator => (EventKind::Duplicator, duplicator::OPTIONS),
        EventName::GoldenShrine => (EventKind::GoldenShrine, golden_shrine::options(ascension)),
        EventName::GoldenIdol => (
            EventKind::GoldenIdol { stage: 0 },
            golden_idol::options(ascension),
        ),
        EventName::WingStatue => (EventKind::WingStatue, wing_statue::OPTIONS),
        EventName::WorldOfGoop => (EventKind::WorldOfGoop, world_of_goop::options(ascension)),
        EventName::LivingWall => (EventKind::LivingWall, living_wall::OPTIONS),
        EventName::Purifier => (EventKind::Purifier, purifier::OPTIONS),
        EventName::ScrapOoze => (
            EventKind::ScrapOoze { attempts: 0 },
            scrap_ooze::options(ascension),
        ),
        EventName::ShiningLight => (EventKind::ShiningLight, shining_light::options(ascension)),
        EventName::TheSsssserpent => (
            EventKind::TheSsssserpent,
            the_ssssserpent::options(ascension),
        ),
        EventName::Transmogrifier => (EventKind::Transmogrifier, transmogrifier::OPTIONS),
        EventName::UpgradeShrine => (EventKind::UpgradeShrine, upgrade_shrine::OPTIONS),
        EventName::TheDivineFountain => {
            (EventKind::TheDivineFountain, the_divine_fountain::OPTIONS)
        }
        EventName::TheLab => (EventKind::TheLab, the_lab::options(ascension)),
        EventName::TheWomanInBlue => (
            EventKind::TheWomanInBlue,
            the_woman_in_blue::options(ascension),
        ),
        EventName::WheelOfChange => (EventKind::WheelOfChange, wheel_of_change::OPTIONS),
        EventName::BonfireSpirits => (EventKind::BonfireSpirits, bonfire_spirits::OPTIONS),
        EventName::OminousForge => (EventKind::OminousForge, ominous_forge::OPTIONS),
        EventName::FaceTrader => (EventKind::FaceTrader, face_trader::options(ascension)),
        EventName::Addict => (EventKind::Addict, addict::OPTIONS),
        EventName::Beggar => (EventKind::Beggar, beggar::OPTIONS),
        EventName::Ghosts => (EventKind::Ghosts, ghosts::options(ascension)),
        EventName::BackToBasics => (EventKind::BackToBasics, back_to_basics::OPTIONS),
        EventName::MaskedBandits => (EventKind::MaskedBandits, masked_bandits::OPTIONS),
        EventName::TheJoust => (EventKind::TheJoust, the_joust::OPTIONS),
        EventName::TheLibrary => (EventKind::TheLibrary, the_library::options(ascension)),
        EventName::TheMausoleum => (EventKind::TheMausoleum, the_mausoleum::OPTIONS),
        EventName::Vampires => (EventKind::Vampires, vampires::OPTIONS),
        EventName::Nest => (EventKind::Nest, nest::options(ascension)),
        EventName::CursedTome => (
            EventKind::CursedTome { stage: 0 },
            cursed_tome::options(ascension),
        ),
        EventName::DrugDealer => (EventKind::DrugDealer, drug_dealer::OPTIONS),
        EventName::ForgottenAltar => (
            EventKind::ForgottenAltar,
            forgotten_altar::options(ascension),
        ),
        // N'loth rolls the two offered Relics and bakes them into its options
        EventName::Nloth => return nloth::spawn_event_nloth(state),
        EventName::Colosseum => (EventKind::Colosseum { stage: 0 }, colosseum::OPTIONS),
        EventName::Designer => (
            EventKind::Designer,
            designer::options(
                ascension,
                state.rng.random_bool(0.5),
                state.rng.random_bool(0.5),
            ),
        ),
        EventName::KnowingSkull => (
            EventKind::KnowingSkull {
                potion_cost_hp: KNOWING_SKULL_COST_START,
                gold_cost_hp: KNOWING_SKULL_COST_START,
                card_cost_hp: KNOWING_SKULL_COST_START,
            },
            knowing_skull::OPTIONS,
        ),
        EventName::Mushrooms => (EventKind::Mushrooms, mushrooms::OPTIONS),
        EventName::DeadAdventurer => (
            EventKind::DeadAdventurer {
                found_gold: false,
                found_nothing: false,
                found_relic: false,
                searches: 0,
            },
            dead_adventurer::OPTIONS,
        ),

        // Neow and We Meet Again! roll and bake their own options
        EventName::Neow => return neow::spawn_event_neow(state),
        EventName::WeMeetAgain => return we_meet_again::spawn_event_we_meet_again(state),
    };
    let id_event_options = bake_options(state, options);
    (kind, id_event_options)
}

// One Entity per option, copied into the arena at spawn
fn bake_options(state: &mut GameState, options: &[Entity]) -> Vec<usize> {
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

pub fn fight_loot(kind: EventKind) -> Option<EventLoot> {
    match kind {
        EventKind::Mushrooms => Some(mushrooms::FIGHT_LOOT),
        EventKind::MaskedBandits => Some(masked_bandits::FIGHT_LOOT),
        // Stage counts bouts started: the first pays nothing, the Nobs pay out
        EventKind::Colosseum { stage: 0 | 1 } => None,
        EventKind::Colosseum { .. } => Some(colosseum::FIGHT_LOOT_NOBS),
        EventKind::DeadAdventurer {
            found_gold,
            found_relic,
            ..
        } => {
            let gold_extra = !found_gold as u16 * 30;
            Some(EventLoot {
                gold: Some(Amount::Range {
                    min: 25 + gold_extra,
                    max: 35 + gold_extra,
                }),
                relics: [
                    (!found_relic).then_some(RelicPick::Thresholds {
                        th_common: RELIC_TIER_TH_COMMON,
                        th_uncommon: RELIC_TIER_TH_UNCOMMON,
                    }),
                    None,
                ],
            })
        }
        // Every fight-hosting event must claim an arm; None is reserved for
        // deliberate unpaid bouts (Colosseum's first)
        _ => unreachable!("fight over a non-fight event: {kind:?}"),
    }
}

// Per-event availability checks
pub fn event_option_available(state: &GameState, kind: EventKind, idx: usize) -> bool {
    match kind {
        EventKind::BigFish
        | EventKind::Duplicator
        | EventKind::GoldenShrine
        | EventKind::WorldOfGoop
        | EventKind::TheSsssserpent
        | EventKind::TheDivineFountain
        | EventKind::TheLab
        | EventKind::TheWomanInBlue
        | EventKind::WheelOfChange
        | EventKind::BonfireSpirits
        | EventKind::FaceTrader
        | EventKind::Mushrooms
        | EventKind::Neow
        | EventKind::Ghosts
        | EventKind::MaskedBandits
        | EventKind::TheJoust
        | EventKind::TheLibrary
        | EventKind::TheMausoleum
        | EventKind::KnowingSkull { .. }
        | EventKind::DeadAdventurer { .. }
        | EventKind::Nest
        | EventKind::Nloth { .. } => true,
        EventKind::Addict => addict::option_available(state, idx),
        EventKind::Beggar => beggar::option_available(state, idx),
        EventKind::BackToBasics => back_to_basics::option_available(state, idx),
        EventKind::Vampires => vampires::option_available(state, idx),
        EventKind::Colosseum { stage } => colosseum::option_available(stage, idx),
        EventKind::Designer => designer::option_available(state, idx),
        EventKind::DrugDealer => drug_dealer::option_available(state, idx),
        EventKind::ForgottenAltar => forgotten_altar::option_available(state, idx),
        EventKind::CursedTome { stage } => cursed_tome::option_available(stage, idx),
        EventKind::TheCleric => the_cleric::option_available(state, idx),
        EventKind::WingStatue => wing_statue::option_available(state, idx),
        EventKind::LivingWall => living_wall::option_available(state, idx),
        EventKind::Purifier => purifier::option_available(state, idx),
        EventKind::ShiningLight => shining_light::option_available(state, idx),
        EventKind::Transmogrifier => transmogrifier::option_available(state, idx),
        EventKind::UpgradeShrine => upgrade_shrine::option_available(state, idx),
        EventKind::OminousForge => ominous_forge::option_available(state, idx),
        EventKind::GoldenIdol { stage } => golden_idol::option_available(stage, idx),
        EventKind::ScrapOoze { attempts } => scrap_ooze::option_available(attempts, idx),
        EventKind::WeMeetAgain {
            id_card,
            id_potion,
            gold_ask,
        } => we_meet_again::option_available(state, id_card, id_potion, gold_ask, idx),
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
