use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;

pub const JOUST_STAKE: u16 = 50;
pub const JOUST_OWNER_WIN_CHANCE: f64 = 0.3;
pub const JOUST_PAYOUT_MURDERER: u16 = 100;
pub const JOUST_PAYOUT_OWNER: u16 = 250;

const fn bet(on_owner: bool) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::JoustBet { on_owner },
            id_source: None,
            target: Target::Direct(None),
        },
        EFFECT_EVENT_CONSUME,
    ]
}

// Bet on the Murderer: the long odds
const OPTION_MURDERER: [Effect; 2] = bet(false);

// Bet on the Owner: the favourite
const OPTION_OWNER: [Effect; 2] = bet(true);

pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_MURDERER),
    make_event_option_template(&OPTION_OWNER),
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
