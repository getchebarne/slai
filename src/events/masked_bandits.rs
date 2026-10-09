use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ReadAt;
use crate::effect::RelicPick;
use crate::effect::Rounding;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventLoot;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::MonsterName;
use crate::types::RelicName;

const fn monster_spawn(name: MonsterName, slot: usize) -> Effect {
    Effect {
        kind: EffectKind::MonsterSpawn {
            name,
            minion: false,
            slot,
        },
        id_source: None,
        target: Target::Direct(None),
    }
}

// Pay: every last coin
const OPTION_PAY: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Relative {
                numerator: 1,
                denominator: 1,
                rounding: Rounding::Truncate,
                read_at: ReadAt::Now,
            },
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// Fight: the whole gang, with the Red Mask and their pocket gold on the line
const OPTION_FIGHT: &[Effect] = &[
    monster_spawn(MonsterName::BanditPointy, 0),
    monster_spawn(MonsterName::BanditLeader, 1),
    monster_spawn(MonsterName::BanditBear, 2),
    Effect {
        kind: EffectKind::CombatStart { elite: false },
        id_source: None,
        target: Target::Direct(None),
    },
];

// The gang's pocket gold and the Red Mask, paid out by `fight_loot`
pub const FIGHT_LOOT: EventLoot = EventLoot {
    gold: Some(Amount::Range { min: 25, max: 35 }),
    relics: [Some(RelicPick::Name(RelicName::RedMask)), None],
};

pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_PAY),
    make_event_option_template(OPTION_FIGHT),
];

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
