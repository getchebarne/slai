use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Mode;
use crate::types::RewardKind;

// Claims hand the staged reward entity to the matching Adopt effect, which owns
// registration and any on-pickup behavior
pub fn process_effect_reward_take(
    id_target: Option<usize>,
    state: &mut GameState,
    kind: RewardKind,
) {
    let Mode::Reward(rewards) = &mut state.mode else {
        unreachable!("RewardTake outside Reward mode")
    };
    match kind {
        RewardKind::Card => {
            let id_card = id_target.expect("RewardTake { Card } requires id_target");

            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardAdopt,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });

            // Clear the rest of the cards
            rewards.id_cards.clear();
        }
        RewardKind::Relic => {
            if let Some(id) = rewards.id_relic.take() {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::RelicAdopt,
                    id_source: None,
                    target: Target::Direct(Some(id)),
                });
            }
        }
        RewardKind::Potion => {
            let id_potion = id_target.expect("RewardTake { Potion } requires id_target");
            let pos = rewards
                .id_potions
                .iter()
                .position(|&id| id == id_potion)
                .expect("taken potion is a staged reward");
            rewards.id_potions.remove(pos);
            state.effect_queue.push_front(Effect {
                kind: EffectKind::PotionAdopt,
                id_source: None,
                target: Target::Direct(Some(id_potion)),
            });
        }
        RewardKind::Gold => {
            if let Some(amount) = rewards.gold.take() {
                let character = &mut state.entities[state.id_character];
                character.character_gold = character.character_gold.saturating_add(amount);
            }
        }
    }
}
