use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EVENT_ADVANCE_EFFECT;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventOptionTemplate;
use crate::events::OPTION_LEAVE;
use crate::events::make_event_option_template;
use crate::types::DeltaSign;
use crate::types::RelicName;

// The three book Relics; the take-option rolls uniformly among the unowned ones
pub const BOOK_POOL: &[RelicName] = &[
    RelicName::Necronomicon,
    RelicName::Enchiridion,
    RelicName::NilrysCodex,
];

const fn hp_loss(amount: u16) -> Effect {
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(amount),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }
}

const OPTION_READ: [Effect; 1] = [EVENT_ADVANCE_EFFECT];
const OPTION_PAGE_1: [Effect; 2] = [hp_loss(1), EVENT_ADVANCE_EFFECT];
const OPTION_PAGE_2: [Effect; 2] = [hp_loss(2), EVENT_ADVANCE_EFFECT];
const OPTION_PAGE_3: [Effect; 2] = [hp_loss(3), EVENT_ADVANCE_EFFECT];

const fn take_book(final_dmg: u16) -> [Effect; 3] {
    [
        hp_loss(final_dmg),
        Effect {
            kind: EffectKind::RelicGrantPool { pool: BOOK_POOL },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_CONSUME_EFFECT,
    ]
}

const OPTION_TAKE_BASE: [Effect; 3] = take_book(10);
const OPTION_TAKE_A15: [Effect; 3] = take_book(15);
const OPTION_STOP: [Effect; 2] = [hp_loss(3), EVENT_CONSUME_EFFECT];

// Only the take-option varies with ascension
const fn options_for(
    take_label: &'static str,
    take: &'static [Effect],
) -> [EventOptionTemplate<'static>; 7] {
    [
        make_event_option_template("[Read] Begin reading.", &OPTION_READ),
        make_event_option_template("[Continue] Lose 1 HP.", &OPTION_PAGE_1),
        make_event_option_template("[Continue] Lose 2 HP.", &OPTION_PAGE_2),
        make_event_option_template("[Continue] Lose 3 HP.", &OPTION_PAGE_3),
        make_event_option_template(take_label, take),
        make_event_option_template("[Stop Reading] Lose 3 HP.", &OPTION_STOP),
        OPTION_LEAVE,
    ]
}

static OPTIONS_BASE: &[EventOptionTemplate] = &options_for(
    "[Take the Book] Lose 10 HP. Obtain a book Relic.",
    &OPTION_TAKE_BASE,
);
static OPTIONS_A15: &[EventOptionTemplate] = &options_for(
    "[Take the Book] Lose 15 HP. Obtain a book Relic.",
    &OPTION_TAKE_A15,
);

pub fn options(ascension: u8) -> &'static [EventOptionTemplate<'static>] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}

// Stages: 0 intro, 1-3 pages, 4 the final choice
pub fn option_available(stage: u8, idx: usize) -> bool {
    match stage {
        0 => idx == 0 || idx == 6,
        1..=3 => idx == stage as usize,
        _ => idx == 4 || idx == 5,
    }
}
