use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::KnowingSkullWish;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::make_entity_event_option;
use crate::types::DeltaSign;

pub const KNOWING_SKULL_COST_START: u8 = 6;
pub const KNOWING_SKULL_GOLD: u16 = 90;
pub const KNOWING_SKULL_COST_LEAVE: u16 = 6;

const fn ask(wish: KnowingSkullWish) -> [Effect; 1] {
    [Effect {
        kind: EffectKind::KnowingSkullAsk { wish },
        id_source: None,
        target: Target::Direct(None),
    }]
}
const OPTION_POTION: [Effect; 1] = ask(KnowingSkullWish::Potion);
const OPTION_GOLD: [Effect; 1] = ask(KnowingSkullWish::Gold);
const OPTION_CARD: [Effect; 1] = ask(KnowingSkullWish::Card);

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
    EVENT_CONSUME_EFFECT,
];

// The asks repeat, each escalating its own HP cost (tracked in the payload)
pub static OPTIONS: &[Entity] = &[
    make_entity_event_option("[A Pretty Potion!] Obtain a random potion.", &OPTION_POTION),
    make_entity_event_option("[Riches!] Obtain 90 Gold.", &OPTION_GOLD),
    make_entity_event_option(
        "[A Great Discovery!] Obtain a random Uncommon colorless card.",
        &OPTION_CARD,
    ),
    make_entity_event_option("[Leave] Lose 6 HP.", OPTION_LEAVE_PAID),
];
