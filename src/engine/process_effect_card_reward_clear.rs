use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_card_reward_clear(card_rewards: &mut Vec<EntityId>) -> ProcessEffectResult {
    // Clear card rewards
    card_rewards.clear();

    // Card reward resolution always transitions back to map-node selection.
    ProcessEffectResult::AddAndContinue {
            top: vec![crate::effect::Effect {
                kind: crate::effect::EffectKind::SelectMapNode,
                source: None,
                target: crate::effect::Target::Resolve {
                    candidates: crate::effect::CandidatePool::MapNodeNextRow,
                    selection: crate::effect::SelectionKind::Input { count: 1 },
                },
            }],
            bot: Vec::new(),
        }
}
