use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_card_reward_clear(card_rewards: &mut Vec<EntityId>) -> ProcessEffectResult {
    // Clear card rewards
    card_rewards.clear();

    // Card reward resolution always transitions back to map-node selection.
    ProcessEffectResult::AddAndContinue {
        top: vec![Effect {
            kind: EffectKind::MapNodeSelect,
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::MapNodeNextRow,
                selection: SelectionKind::Input { count: 1 },
            },
        }],
        bot: Vec::new(),
    }
}
