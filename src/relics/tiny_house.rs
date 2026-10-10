use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RewardRollTrigger;
use crate::effect::SelectionKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup: upgrade 1 random Card, +5 max HP (healed), 50 gold, 1 random Potion, 1 Card
// See:
//    - `process_effect_relic_adopt.rs`
pub static TINY_HOUSE: RelicTemplate = RelicTemplate {
    name: RelicName::TinyHouse,
    tier: RelicTier::Boss,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[
        Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::CardPileDeck,
                filters: &[CandidateFilter::Upgradeable],
                selection_kind: SelectionKind::Random { count: 1 },
            },
        },
        Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(5),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::RewardRollGold {
                amount: Amount::Absolute(50),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::RewardRollPotions {
                count: 1,
                trigger: RewardRollTrigger::TinyHouse,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::RewardRollCards {
                bundles: 1,
                trigger: RewardRollTrigger::CombatMonster,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    effects_rest: &[],
    effects_counter: &[],
};
