use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EFFECT_DECK_PURGE_PICK_1;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::DeltaSign;

// The draw gate in `draw_event` requires this much gold before the event can spawn
pub const BEGGAR_COST_PURGE: u16 = 75;

// Give: 75 gold buys a Card purge; consume after the cost, so the pick lands over the spent event
const OPTION_GIVE: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(BEGGAR_COST_PURGE),
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
    EFFECT_DECK_PURGE_PICK_1,
];

pub static EOTS_BASE: &[EventOptionTemplate] =
    &[make_event_option_template(OPTION_GIVE), EOT_LEAVE];

// Give needs only the gold; with nothing to purge its pick resolves to nothing
pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => state.entities[state.id_character].character_gold >= BEGGAR_COST_PURGE,
        _ => true,
    }
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
