use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicPick;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_ADVANCE;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventLoot;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::MonsterName;
use crate::types::RelicTier;

const fn monster_spawn(name: MonsterName) -> Effect {
    Effect {
        kind: EffectKind::MonsterSpawn {
            name,
            minion: false,
            cap: None,
        },
        id_source: None,
        target: Target::Direct(None),
    }
}

// First bout: the event stays live beneath the fight and resumes with no reward
const OPTION_FIGHT: &[Effect] = &[
    Effect {
        kind: EffectKind::EventAdvanceState { delta: 1 },
        id_source: None,
        target: Target::Direct(None),
    },
    monster_spawn(MonsterName::SlaverBlue),
    monster_spawn(MonsterName::SlaverRed),
    Effect {
        kind: EffectKind::CombatStart,
        id_source: None,
        target: Target::Direct(None),
    },
];

// Second bout: an elite pair with a rare+uncommon Relic purse and 100 gold
const OPTION_FIGHT_NOBS: &[Effect] = &[
    EFFECT_EVENT_ADVANCE,
    monster_spawn(MonsterName::Taskmaster),
    monster_spawn(MonsterName::GremlinNob),
    Effect {
        kind: EffectKind::CombatStart,
        id_source: None,
        target: Target::Direct(None),
    },
];

// The Nobs purse: 100 gold plus a rare and an uncommon Relic, paid by `fight_loot`
pub const FIGHT_LOOT_NOBS: EventLoot = EventLoot {
    gold: Some(Amount::Absolute(100)),
    relics: [
        Some(RelicPick::Tier(RelicTier::Rare)),
        Some(RelicPick::Tier(RelicTier::Uncommon)),
    ],
};

pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_FIGHT),
    make_event_option_template(OPTION_FIGHT_NOBS),
    make_event_option_template(&[EFFECT_EVENT_CONSUME]),
];

// Stage 0 offers only the first bout; stage 1 the Nobs or the exit
pub fn option_available(state: &GameState, idx: usize) -> bool {
    let stage = state.event.stage;
    match stage {
        0 => idx == 0,
        _ => idx != 0,
    }
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
