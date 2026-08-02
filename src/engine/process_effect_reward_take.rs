use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RewardKind;

// Claims hand the staged reward entity to the matching Adopt effect, which owns
// registration and any on-pickup behavior
pub fn process_effect_reward_take(
    id_target: Option<usize>,
    state: &mut GameState,
    kind: RewardKind,
) {
    match kind {
        RewardKind::Card => {
            let id_card = id_target.expect("RewardTake { Card } requires id_target");
            let Some(Mode::Reward {
                reward_id_cards: bundles,
                ..
            }) = state.mode_stack.last_mut()
            else {
                unreachable!("RewardTake {{ Card }} outside Reward mode")
            };

            // Taking a card consumes its whole bundle; the others stay claimable
            let pos = bundles
                .iter()
                .position(|bundle| bundle.contains(&id_card))
                .expect("taken card is a staged bundle");
            bundles.remove(pos);

            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardAdopt,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
        RewardKind::Relic => {
            let Some(Mode::Reward {
                reward_id_relics, ..
            }) = state.mode_stack.last_mut()
            else {
                unreachable!("RewardTake {{ Relic }} outside Reward mode")
            };
            let id_relic = id_target.expect("RewardTake { Relic } requires id_target");
            let pos = reward_id_relics
                .iter()
                .position(|&id| id == id_relic)
                .expect("taken relic is a staged reward");
            reward_id_relics.remove(pos);
            state.effect_queue.push_front(Effect {
                kind: EffectKind::RelicAdopt,
                id_source: None,
                target: Target::Direct(Some(id_relic)),
            });
        }
        RewardKind::Potion => {
            let Some(Mode::Reward {
                reward_id_potions, ..
            }) = state.mode_stack.last_mut()
            else {
                unreachable!("RewardTake {{ Potion }} outside Reward mode")
            };
            let id_potion = id_target.expect("RewardTake { Potion } requires id_target");
            let pos = reward_id_potions
                .iter()
                .position(|&id| id == id_potion)
                .expect("taken potion is a staged reward");
            reward_id_potions.remove(pos);
            state.effect_queue.push_front(Effect {
                kind: EffectKind::PotionAdopt,
                id_source: None,
                target: Target::Direct(Some(id_potion)),
            });
        }
        RewardKind::Gold => {
            let Some(Mode::Reward { reward_gold, .. }) = state.mode_stack.last_mut() else {
                unreachable!("RewardTake {{ Gold }} outside Reward mode")
            };
            // Routed through GoldDelta so the MAX_GOLD cap and Ectoplasm apply
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
