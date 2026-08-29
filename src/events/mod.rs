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
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::ENTITY_ZERO;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Event;
use crate::types::EventName;
use crate::types::event_reset;
use crate::utils::card_is_non_basic_non_curse;
use crate::utils::card_is_purgeable;
use crate::utils::card_is_transformable;
use crate::utils::card_is_upgradable;
use crate::utils::push_entity;

pub use beggar::BEGGAR_COST_PURGE;
pub use the_joust::JOUST_OWNER_WIN_CHANCE;
pub use the_joust::JOUST_PAYOUT_MURDERER;
pub use the_joust::JOUST_PAYOUT_OWNER;
pub use the_joust::JOUST_STAKE;

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

// Transform two chosen Cards (Designer's Clean Up, Drug Dealer)
pub const EFFECT_DECK_TRANSFORM_PICK_2: Effect = Effect {
    kind: EffectKind::CardTransform { upgraded: false },
    id_source: None,
    target: Target::Resolve {
        candidate_pool: CandidatePool::Deck,
        filter: CandidateFilter::Transformable,
        selection_kind: SelectionKind::Input { count: 2 },
    },
};

pub const EFFECT_EVENT_ADVANCE: Effect = Effect {
    kind: EffectKind::EventAdvanceState { delta: 1 },
    id_source: None,
    target: Target::Direct(None),
};

pub const EFFECT_EVENT_CONSUME: Effect = Effect {
    kind: EffectKind::EventConsume,
    id_source: None,
    target: Target::Direct(None),
};

// Character HP loss, the standard Event Option cost
pub const fn health_delta(amount: u16) -> Effect {
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(amount),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventOptionTemplate {
    pub effects: [Effect; MAX_EFFECTS_PER_EVENT_OPTION],
    pub effects_len: u8,
}

pub const fn make_event_option_template(effects: &[Effect]) -> EventOptionTemplate {
    let mut owned = [EFFECT_ZERO; MAX_EFFECTS_PER_EVENT_OPTION];
    let mut idx = 0;
    while idx < effects.len() {
        owned[idx] = effects[idx];
        idx += 1;
    }
    EventOptionTemplate {
        effects: owned,
        effects_len: effects.len() as u8,
    }
}

// The leave option and the three deck-pick effects every shrine-shaped event repeats
pub const EOT_LEAVE: EventOptionTemplate = make_event_option_template(&[EFFECT_EVENT_CONSUME]);

// Bakes the options into the arena; processors may mutate baked amounts mid-event
pub fn spawn_event(state: &mut GameState, name: EventName) -> Vec<usize> {
    // Clear last visit's staged picks before this spawn stakes its own
    event_reset(&mut state.event);

    match name {
        EventName::BigFish => big_fish::spawn(state),
        EventName::TheCleric => the_cleric::spawn(state),
        EventName::Duplicator => duplicator::spawn(state),
        EventName::GoldenShrine => golden_shrine::spawn(state),
        EventName::WingStatue => wing_statue::spawn(state),
        EventName::WorldOfGoop => world_of_goop::spawn(state),
        EventName::LivingWall => living_wall::spawn(state),
        EventName::Purifier => purifier::spawn(state),
        EventName::ShiningLight => shining_light::spawn(state),
        EventName::TheSsssserpent => the_ssssserpent::spawn(state),
        EventName::Transmogrifier => transmogrifier::spawn(state),
        EventName::UpgradeShrine => upgrade_shrine::spawn(state),
        EventName::TheDivineFountain => the_divine_fountain::spawn(state),
        EventName::TheLab => the_lab::spawn(state),
        EventName::TheWomanInBlue => the_woman_in_blue::spawn(state),
        EventName::WheelOfChange => wheel_of_change::spawn(state),
        EventName::BonfireSpirits => bonfire_spirits::spawn(state),
        EventName::OminousForge => ominous_forge::spawn(state),
        EventName::FaceTrader => face_trader::spawn(state),
        EventName::Mushrooms => mushrooms::spawn(state),
        EventName::GoldenIdol => golden_idol::spawn(state),
        EventName::ScrapOoze => scrap_ooze::spawn(state),
        EventName::WeMeetAgain => we_meet_again::spawn(state),
        EventName::DeadAdventurer => dead_adventurer::spawn(state),
        EventName::Neow => neow::spawn(state),
        EventName::Addict => addict::spawn(state),
        EventName::Beggar => beggar::spawn(state),
        EventName::Ghosts => ghosts::spawn(state),
        EventName::BackToBasics => back_to_basics::spawn(state),
        EventName::MaskedBandits => masked_bandits::spawn(state),
        EventName::TheJoust => the_joust::spawn(state),
        EventName::TheLibrary => the_library::spawn(state),
        EventName::TheMausoleum => the_mausoleum::spawn(state),
        EventName::Vampires => vampires::spawn(state),
        EventName::Colosseum => colosseum::spawn(state),
        EventName::Designer => designer::spawn(state),
        EventName::KnowingSkull => knowing_skull::spawn(state),
        EventName::Nest => nest::spawn(state),
        EventName::CursedTome => cursed_tome::spawn(state),
        EventName::DrugDealer => drug_dealer::spawn(state),
        EventName::ForgottenAltar => forgotten_altar::spawn(state),
        EventName::Nloth => nloth::spawn(state),
    }
}

// Every event's reachable option EOTs, state-free. A spawn selects from these
pub fn options_catalog(name: EventName, ascension: u8) -> &'static [EventOptionTemplate] {
    match name {
        EventName::BigFish => big_fish::catalog(ascension),
        EventName::TheCleric => the_cleric::catalog(ascension),
        EventName::Duplicator => duplicator::catalog(ascension),
        EventName::GoldenShrine => golden_shrine::catalog(ascension),
        EventName::WingStatue => wing_statue::catalog(ascension),
        EventName::WorldOfGoop => world_of_goop::catalog(ascension),
        EventName::LivingWall => living_wall::catalog(ascension),
        EventName::Purifier => purifier::catalog(ascension),
        EventName::ShiningLight => shining_light::catalog(ascension),
        EventName::TheSsssserpent => the_ssssserpent::catalog(ascension),
        EventName::Transmogrifier => transmogrifier::catalog(ascension),
        EventName::UpgradeShrine => upgrade_shrine::catalog(ascension),
        EventName::TheDivineFountain => the_divine_fountain::catalog(ascension),
        EventName::TheLab => the_lab::catalog(ascension),
        EventName::TheWomanInBlue => the_woman_in_blue::catalog(ascension),
        EventName::WheelOfChange => wheel_of_change::catalog(ascension),
        EventName::BonfireSpirits => bonfire_spirits::catalog(ascension),
        EventName::OminousForge => ominous_forge::catalog(ascension),
        EventName::FaceTrader => face_trader::catalog(ascension),
        EventName::Mushrooms => mushrooms::catalog(ascension),
        EventName::GoldenIdol => golden_idol::catalog(ascension),
        EventName::ScrapOoze => scrap_ooze::catalog(ascension),
        EventName::WeMeetAgain => we_meet_again::catalog(ascension),
        EventName::DeadAdventurer => dead_adventurer::catalog(ascension),
        EventName::Neow => neow::catalog(ascension),
        EventName::Addict => addict::catalog(ascension),
        EventName::Beggar => beggar::catalog(ascension),
        EventName::Ghosts => ghosts::catalog(ascension),
        EventName::BackToBasics => back_to_basics::catalog(ascension),
        EventName::MaskedBandits => masked_bandits::catalog(ascension),
        EventName::TheJoust => the_joust::catalog(ascension),
        EventName::TheLibrary => the_library::catalog(ascension),
        EventName::TheMausoleum => the_mausoleum::catalog(ascension),
        EventName::Vampires => vampires::catalog(ascension),
        EventName::Colosseum => colosseum::catalog(ascension),
        EventName::Designer => designer::catalog(ascension),
        EventName::KnowingSkull => knowing_skull::catalog(ascension),
        EventName::Nest => nest::catalog(ascension),
        EventName::CursedTome => cursed_tome::catalog(ascension),
        EventName::DrugDealer => drug_dealer::catalog(ascension),
        EventName::ForgottenAltar => forgotten_altar::catalog(ascension),
        EventName::Nloth => nloth::catalog(ascension),
    }
}

// One Entity per option, instanced into the arena at spawn
fn bake_options(state: &mut GameState, options: &[EventOptionTemplate]) -> Vec<usize> {
    let mut id_event_options = Vec::with_capacity(options.len());
    for template in options {
        let option = instance_event_option(template);
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
        EventName::BigFish => big_fish::option_available(state, idx),
        EventName::TheCleric => the_cleric::option_available(state, idx),
        EventName::Duplicator => duplicator::option_available(state, idx),
        EventName::GoldenShrine => golden_shrine::option_available(state, idx),
        EventName::WingStatue => wing_statue::option_available(state, idx),
        EventName::WorldOfGoop => world_of_goop::option_available(state, idx),
        EventName::LivingWall => living_wall::option_available(state, idx),
        EventName::Purifier => purifier::option_available(state, idx),
        EventName::ShiningLight => shining_light::option_available(state, idx),
        EventName::TheSsssserpent => the_ssssserpent::option_available(state, idx),
        EventName::Transmogrifier => transmogrifier::option_available(state, idx),
        EventName::UpgradeShrine => upgrade_shrine::option_available(state, idx),
        EventName::TheDivineFountain => the_divine_fountain::option_available(state, idx),
        EventName::TheLab => the_lab::option_available(state, idx),
        EventName::TheWomanInBlue => the_woman_in_blue::option_available(state, idx),
        EventName::WheelOfChange => wheel_of_change::option_available(state, idx),
        EventName::BonfireSpirits => bonfire_spirits::option_available(state, idx),
        EventName::OminousForge => ominous_forge::option_available(state, idx),
        EventName::FaceTrader => face_trader::option_available(state, idx),
        EventName::Mushrooms => mushrooms::option_available(state, idx),
        EventName::GoldenIdol => golden_idol::option_available(state, idx),
        EventName::ScrapOoze => scrap_ooze::option_available(state, idx),
        EventName::WeMeetAgain => we_meet_again::option_available(state, idx),
        EventName::DeadAdventurer => dead_adventurer::option_available(state, idx),
        EventName::Neow => neow::option_available(state, idx),
        EventName::Addict => addict::option_available(state, idx),
        EventName::Beggar => beggar::option_available(state, idx),
        EventName::Ghosts => ghosts::option_available(state, idx),
        EventName::BackToBasics => back_to_basics::option_available(state, idx),
        EventName::MaskedBandits => masked_bandits::option_available(state, idx),
        EventName::TheJoust => the_joust::option_available(state, idx),
        EventName::TheLibrary => the_library::option_available(state, idx),
        EventName::TheMausoleum => the_mausoleum::option_available(state, idx),
        EventName::Vampires => vampires::option_available(state, idx),
        EventName::Colosseum => colosseum::option_available(state, idx),
        EventName::Designer => designer::option_available(state, idx),
        EventName::KnowingSkull => knowing_skull::option_available(state, idx),
        EventName::Nest => nest::option_available(state, idx),
        EventName::CursedTome => cursed_tome::option_available(state, idx),
        EventName::DrugDealer => drug_dealer::option_available(state, idx),
        EventName::ForgottenAltar => forgotten_altar::option_available(state, idx),
        EventName::Nloth => nloth::option_available(state, idx),
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

const fn instance_event_option(template: &EventOptionTemplate) -> Entity {
    Entity {
        kind: EntityKind::EventOption,
        event_option_effects: template.effects,
        event_option_effects_len: template.effects_len,
        ..ENTITY_ZERO
    }
}
