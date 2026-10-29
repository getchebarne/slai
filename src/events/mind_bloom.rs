use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ReadAt;
use crate::effect::RelicRoll;
use crate::effect::Rounding;
use crate::effect::SelectionKind;
use crate::effect::Summon;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventLoot;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::utils::run_floor;

// I am War's boss: an Act 1 boss, rolled when chosen, not at the open, in slot 2 as in its Boss Room
static SUMMON_WAR: Summon = Summon {
    pool: &[
        MonsterName::TheGuardian,
        MonsterName::Hexaghost,
        MonsterName::SlimeBoss,
    ],
    posts: &[2],
    minion: false,
};

// I am War: fight that boss
const OPTION_WAR: &[Effect] = &[
    Effect {
        kind: EffectKind::MonsterSummon {
            summon: &SUMMON_WAR,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::CombatStart { elite: false },
        id_source: None,
        target: Target::Direct(None),
    },
];

// The War purse: 50 gold and a Rare Relic, paid by `fight_loot`
pub const FIGHT_LOOT_BASE: EventLoot = EventLoot {
    gold: Some(Amount::Absolute(50)),
    relics: [Some(RelicRoll::Tier(RelicTier::Rare)), None],
};

// The War purse at A13+: 25 gold
pub const FIGHT_LOOT_A13: EventLoot = EventLoot {
    gold: Some(Amount::Absolute(25)),
    ..FIGHT_LOOT_BASE
};

// I am Awake: every upgradable deck Card upgrades, then Mark of the Bloom
const OPTION_AWAKE: &[Effect] = &[
    Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::CardPileDeck,
            filters: &[CandidateFilter::Upgradeable],
            selection_kind: SelectionKind::All,
        },
    },
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::MarkOfTheBloom,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// I am Rich: 999 gold, then two Normality
const OPTION_RICH: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(999),
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Normality,
            card_pile: CardPile::Deck,
            count: 2,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// I am Healthy: heal to full, then a Doubt
const OPTION_HEALTHY: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Relative {
                numerator: 1,
                denominator: 1,
                rounding: Rounding::Truncate,
                read_at: ReadAt::Now,
            },
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Doubt,
            card_pile: CardPile::Deck,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// Rich and Healthy share the third slot; the spawn bakes one of them
static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_WAR),
    make_event_option_template(OPTION_AWAKE),
    make_event_option_template(OPTION_RICH),
    make_event_option_template(OPTION_HEALTHY),
];

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

// The run floor the event opens on picks the third option: Rich through floor 40, Healthy past it
pub fn spawn(state: &mut GameState) -> Vec<usize> {
    let eots = catalog(state.ascension);
    let third = if run_floor(state.act, state.location) <= 40 {
        eots[2]
    } else {
        eots[3]
    };
    let options = [eots[0], eots[1], third];
    bake_options(state, &options)
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
