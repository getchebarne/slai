use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicPick;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventLoot;
use crate::events::make_entity_event_option;
use crate::types::DeltaSign;
use crate::types::MonsterName;
use crate::types::RelicName;

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

// Pay: every last coin
const OPTION_PAY: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Relative {
                numerator: 1,
                denominator: 1,
            },
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Fight: the whole gang, with the Red Mask and their pocket gold on the line
const OPTION_FIGHT: &[Effect] = &[
    spawn(MonsterName::BanditPointy),
    spawn(MonsterName::BanditLeader),
    spawn(MonsterName::BanditBear),
    Effect {
        kind: EffectKind::CombatStart,
        id_source: None,
        target: Target::Direct(None),
    },
];

// The gang's pocket gold and the Red Mask, paid out by `fight_loot`
pub const FIGHT_LOOT: EventLoot = EventLoot {
    gold: Some(Amount::Range { min: 25, max: 35 }),
    relics: [Some(RelicPick::Name(RelicName::RedMask)), None],
};

pub static OPTIONS: &[Entity] = &[
    make_entity_event_option("[Pay] Lose ALL your Gold.", OPTION_PAY),
    make_entity_event_option(
        "[Fight] Gain 25-35 Gold. Obtain the Red Mask.",
        OPTION_FIGHT,
    ),
];
