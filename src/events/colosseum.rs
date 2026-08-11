use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::EventLoot;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::make_entity_event_option;
use crate::types::MonsterName;
use crate::types::RelicTier;

const fn spawn(name: MonsterName) -> Effect {
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
    spawn(MonsterName::SlaverBlue),
    spawn(MonsterName::SlaverRed),
    Effect {
        kind: EffectKind::CombatStart {
            loot: EventLoot::NONE,
        },
        id_source: None,
        target: Target::Direct(None),
    },
];

// Second bout: an elite pair with a rare+uncommon relic purse and 100 gold
const OPTION_FIGHT_NOBS: &[Effect] = &[
    EVENT_CONSUME_EFFECT,
    spawn(MonsterName::Taskmaster),
    spawn(MonsterName::GremlinNob),
    Effect {
        kind: EffectKind::CombatStart {
            loot: EventLoot {
                gold: Some(Amount::Absolute(100)),
                relic: None,
                relic_roll: false,
                relic_tiers: [Some(RelicTier::Rare), Some(RelicTier::Uncommon)],
            },
        },
        id_source: None,
        target: Target::Direct(None),
    },
];

pub static OPTIONS: &[Entity] = &[
    make_entity_event_option("[Fight] Face the first round.", OPTION_FIGHT),
    make_entity_event_option(
        "[Fight the Nobs] Gain 100 Gold. Obtain a Rare and an Uncommon Relic.",
        OPTION_FIGHT_NOBS,
    ),
    make_entity_event_option("[Flee] Escape the arena.", &[EVENT_CONSUME_EFFECT]),
];

// Stage 0 offers only the first bout; stage 1 the Nobs or the exit
pub fn option_available(stage: u8, idx: usize) -> bool {
    match stage {
        0 => idx == 0,
        _ => idx != 0,
    }
}
