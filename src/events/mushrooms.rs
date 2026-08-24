use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicPick;
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

const SPAWN_FUNGI: Effect = Effect {
    kind: EffectKind::MonsterSpawn {
        name: MonsterName::FungiBeast,
        minion: false,
        cap: None,
    },
    id_source: None,
    target: Target::Direct(None),
};

// Stomp: fight 3 Fungi Beasts — the reward roll gives gold and an Odd Mushroom
const OPTION_STOMP: &[Effect] = &[
    SPAWN_FUNGI,
    SPAWN_FUNGI,
    SPAWN_FUNGI,
    Effect {
        kind: EffectKind::CombatStart,
        id_source: None,
        target: Target::Direct(None),
    },
];

// Staked on the Stomp fight; paid out by `fight_loot` when the combat ends
pub const FIGHT_LOOT: EventLoot = EventLoot {
    gold: Some(Amount::Range { min: 20, max: 30 }),
    relics: [Some(RelicPick::Name(RelicName::OddMushroom)), None],
};

// Eat: heal 25% max HP and become Cursed w/ Parasite
const OPTION_EAT: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Relative {
                numerator: 1,
                denominator: 4,
            },
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Parasite,
            pile: CardPile::Deck,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_STOMP),
    make_event_option_template(OPTION_EAT),
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
