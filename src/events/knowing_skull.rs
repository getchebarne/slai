use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::DeltaSign;

const KNOWING_SKULL_COST_START: u16 = 6;
const KNOWING_SKULL_GOLD: u16 = 90;
const KNOWING_SKULL_COST_LEAVE: u16 = 6;

const fn wish(effect_kind_reward: EffectKind) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(KNOWING_SKULL_COST_START),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: effect_kind_reward,
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::KnowingSkullCostBump,
            id_source: None,
            target: Target::Direct(None),
        },
    ]
}

// Potion: a random one, at a rising HP cost
const OPTION_POTION: [Effect; 3] = wish(EffectKind::PotionAddRandom { limited: false });

// Gold: 90, at a rising HP cost
const OPTION_GOLD: [Effect; 3] = wish(EffectKind::GoldDelta {
    sign: DeltaSign::Gain,
    amount: Amount::Absolute(KNOWING_SKULL_GOLD),
});

// Card: a random colorless one, at a rising HP cost
const OPTION_CARD: [Effect; 3] = wish(EffectKind::CardAddRandom {
    color: CardColor::Colorless,
    kind: None,
    pile: CardPile::Deck,
    count: 1,
    cost_zero: None,
    upgraded: false,
    rarity: Some(CardRarity::Uncommon),
});

// Leaving costs a flat 6 HP
const OPTION_LEAVE_PAID: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(KNOWING_SKULL_COST_LEAVE),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    EFFECT_EVENT_CONSUME,
];

pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_POTION),
    make_event_option_template(&OPTION_GOLD),
    make_event_option_template(&OPTION_CARD),
    make_event_option_template(OPTION_LEAVE_PAID),
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
