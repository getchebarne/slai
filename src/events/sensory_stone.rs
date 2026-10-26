use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RewardRollTrigger;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::health_delta;
use crate::events::make_event_option_template;
use crate::game::GameState;

// Colorless Card rewards, rolled when the option is chosen
const fn colorless_rewards(bundles: u8) -> Effect {
    Effect {
        kind: EffectKind::RewardRollCards {
            bundles,
            trigger: RewardRollTrigger::SensoryStone,
        },
        id_source: None,
        target: Target::Direct(None),
    }
}

// Recall: one colorless Card reward
const OPTION_RECALL_1: [Effect; 2] = [EFFECT_EVENT_CONSUME, colorless_rewards(1)];

// Recall: two colorless Card rewards, then lose 5 HP
const OPTION_RECALL_2: [Effect; 3] = [EFFECT_EVENT_CONSUME, colorless_rewards(2), health_delta(5)];

// Recall: three colorless Card rewards, then lose 10 HP
const OPTION_RECALL_3: [Effect; 3] = [EFFECT_EVENT_CONSUME, colorless_rewards(3), health_delta(10)];

static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_RECALL_1),
    make_event_option_template(&OPTION_RECALL_2),
    make_event_option_template(&OPTION_RECALL_3),
];

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

// A lethal HP cost stays selectable
pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
