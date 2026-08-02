use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RewardKind;
use crate::utils::mode_top_mut;

// Claims hand the staged reward entity to the matching Adopt effect, which owns
// registration and any on-pickup behavior
pub fn process_effect_reward_take(
    id_target: Option<usize>,
    state: &mut GameState,
    kind: RewardKind,
) {
    match kind {
        // Card: taking one consumes its whole bundle; sibling bundles stay claimable
        RewardKind::Card => {
            let id_card = id_target.expect("RewardTake { Card } requires id_target");
            let Mode::Reward {
                reward_id_cards: bundles,
                ..
            } = mode_top_mut(&mut state.mode_stack)
            else {
                unreachable!("RewardTake {{ Card }} outside Reward mode")
            };

            // Remove Card's associated bundle
            let idx = bundles
                .iter()
                .position(|bundle| bundle.contains(&id_card))
                .expect("Taken Card is a staged bundle");
            bundles.remove(idx);

            // Push adoption effect
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardAdopt,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }

        // Relic: unstage the pick; RelicAdopt owns registration and pickup effects
        RewardKind::Relic => {
            let Mode::Reward {
                reward_id_relics, ..
            } = mode_top_mut(&mut state.mode_stack)
            else {
                unreachable!("RewardTake {{ Relic }} outside Reward mode")
            };
            let id_relic = id_target.expect("RewardTake { Relic } requires id_target");

            // Take Relic
            let idx = reward_id_relics
                .iter()
                .position(|&id| id == id_relic)
                .expect("Taken Relic is a staged reward");
            reward_id_relics.remove(idx);

            // Push adoption effect
            state.effect_queue.push_front(Effect {
                kind: EffectKind::RelicAdopt,
                id_source: None,
                target: Target::Direct(Some(id_relic)),
            });
        }

        // Potion: unstage the pick; PotionAdopt owns the belt slot (and the Sozu guard)
        RewardKind::Potion => {
            let Mode::Reward {
                reward_id_potions, ..
            } = mode_top_mut(&mut state.mode_stack)
            else {
                unreachable!("RewardTake {{ Potion }} outside Reward mode")
            };
            let id_potion = id_target.expect("RewardTake { Potion } requires id_target");

            // Take Potion
            let idx = reward_id_potions
                .iter()
                .position(|&id| id == id_potion)
                .expect("Taken Potion is a staged reward");
            reward_id_potions.remove(idx);

            // Push adoption effect
            state.effect_queue.push_front(Effect {
                kind: EffectKind::PotionAdopt,
                id_source: None,
                target: Target::Direct(Some(id_potion)),
            });
        }

        // Gold: routed through GoldDelta so the MAX_GOLD cap and Ectoplasm apply
        RewardKind::Gold => {
            let Mode::Reward { reward_gold, .. } = mode_top_mut(&mut state.mode_stack) else {
                unreachable!("RewardTake {{ Gold }} outside Reward mode")
            };
            if let Some(amount) = reward_gold.take() {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::GoldDelta {
                        sign: DeltaSign::Gain,
                        amount: Amount::Absolute(amount),
                    },
                    id_source: None,
                    target: Target::Direct(None),
                });
            }
        }
    }
}
