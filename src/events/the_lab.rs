use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;

// Search: the rolled Potions land on the Reward context, where the belt is
// interactive (discard-to-swap), matching the source's combatRewardScreen
const fn search(count: u8) -> [Effect; 2] {
    [
        // Consume first: the staged Reward overlays this frame until RoomExit
        EFFECT_EVENT_CONSUME,
        Effect {
            kind: EffectKind::RewardRollPotions {
                count,
                uniform: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ]
}

// Search: 3 random Potions
const OPTION_SEARCH_BASE: [Effect; 2] = search(3);

// Search at A15+: only 2
const OPTION_SEARCH_A15: [Effect; 2] = search(2);

// The source game offers no way to decline
static EOTS_BASE: &[EventOptionTemplate] = &[make_event_option_template(&OPTION_SEARCH_BASE)];
static EOTS_A15: &[EventOptionTemplate] = &[make_event_option_template(&OPTION_SEARCH_A15)];

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 { EOTS_BASE } else { EOTS_A15 }
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
