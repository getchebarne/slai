use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::MonsterName;

const SPAWN_FUNGI: Effect = Effect {
    kind: EffectKind::MonsterSpawn {
        name: MonsterName::FungiBeast,
    },
    id_source: None,
    target: Target::Direct(None),
};

// Stomp: fight 3 Fungi Beasts — the reward roll gives gold and an Odd Mushroom
const OPTION_STOMP: &[Effect] = &[
    EVENT_CONSUME_EFFECT,
    SPAWN_FUNGI,
    SPAWN_FUNGI,
    SPAWN_FUNGI,
    Effect {
        kind: EffectKind::CombatStart,
        id_source: None,
        target: Target::Direct(None),
    },
];

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
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    Effect {
        kind: EffectKind::CardAddToDeck {
            card_name: CardName::Parasite,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

pub const LABELS: &[&str] = &[
    "[Stomp] Fight 3 Fungi Beasts.",
    "[Eat] Heal 25% of your Max HP. Become Cursed - Parasite.",
];

pub fn push_option_effects(buf: &mut Vec<Effect>, idx: usize) {
    buf.extend_from_slice(match idx {
        0 => OPTION_STOMP,
        1 => OPTION_EAT,
        _ => unreachable!("mushrooms option out of range: {idx}"),
    });
}
