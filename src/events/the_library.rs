use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::make_entity_event_option;
use crate::types::DeltaSign;

// Read: 20 unique rolled cards staged on the Reward context, keep one
const OPTION_READ: &[Effect] = &[
    EVENT_CONSUME_EFFECT,
    Effect {
        kind: EffectKind::RewardRollLibraryCards,
        id_source: None,
        target: Target::Direct(None),
    },
];

// Sleep: heal a third (A15+: a fifth) of max HP
const fn sleep(numerator: u8, denominator: u8) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Relative {
                    numerator,
                    denominator,
                },
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        EVENT_CONSUME_EFFECT,
    ]
}
const OPTION_SLEEP_BASE: [Effect; 2] = sleep(33, 100);
const OPTION_SLEEP_A15: [Effect; 2] = sleep(20, 100);

static OPTIONS_BASE: &[Entity] = &[
    make_entity_event_option("[Read] Choose 1 of 20 random cards.", OPTION_READ),
    make_entity_event_option("[Sleep] Heal 33% of your Max HP.", &OPTION_SLEEP_BASE),
];
static OPTIONS_A15: &[Entity] = &[
    make_entity_event_option("[Read] Choose 1 of 20 random cards.", OPTION_READ),
    make_entity_event_option("[Sleep] Heal 20% of your Max HP.", &OPTION_SLEEP_A15),
];

pub fn options(ascension: u8) -> &'static [Entity] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
