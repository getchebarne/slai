use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event_option;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::types::DeltaSign;

const fn buy(cost: u16, count: u8) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(cost),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        // Consume first: the potion roll replaces this event with Mode::Reward.
        // Rolled potions land on the reward screen, where the belt is interactive
        // (discard-to-swap), matching the source's combatRewardScreen
        EVENT_CONSUME_EFFECT,
        Effect {
            kind: EffectKind::RewardRollPotions { count },
            id_source: None,
            target: Target::Direct(None),
        },
    ]
}
const OPTION_BUY_1: [Effect; 3] = buy(20, 1);
const OPTION_BUY_2: [Effect; 3] = buy(30, 2);
const OPTION_BUY_3: [Effect; 3] = buy(40, 3);

// Leave: free below A15; costs ceil(5% max HP) at A15+
const OPTION_LEAVE_BASE: &[Effect] = &[EVENT_CONSUME_EFFECT];
const OPTION_LEAVE_A15: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::RelativeCeil {
                numerator: 1,
                denominator: 20,
            },
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    EVENT_CONSUME_EFFECT,
];

// The event only spawns with >= 50 gold, which covers every price
static OPTIONS_BASE: &[Entity] = &[
    make_entity_event_option("[Buy 1 Potion] Lose 20 Gold.", &OPTION_BUY_1),
    make_entity_event_option("[Buy 2 Potions] Lose 30 Gold.", &OPTION_BUY_2),
    make_entity_event_option("[Buy 3 Potions] Lose 40 Gold.", &OPTION_BUY_3),
    make_entity_event_option("[Leave] Nothing happens.", OPTION_LEAVE_BASE),
];
static OPTIONS_A15: &[Entity] = &[
    make_entity_event_option("[Buy 1 Potion] Lose 20 Gold.", &OPTION_BUY_1),
    make_entity_event_option("[Buy 2 Potions] Lose 30 Gold.", &OPTION_BUY_2),
    make_entity_event_option("[Buy 3 Potions] Lose 40 Gold.", &OPTION_BUY_3),
    make_entity_event_option("[Leave] Lose 5% of your Max HP.", OPTION_LEAVE_A15),
];

pub fn options(ascension: u8) -> &'static [Entity] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
