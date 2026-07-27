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
    let Mode::Reward {
        reward_id_cards,
        reward_id_relic,
        reward_id_potions,
        reward_gold,
        ..
    } = &mut state.mode
    else {
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
            reward_id_cards.clear();
        }
        RewardKind::Relic => {
            if let Some(id) = reward_id_relic.take() {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::RelicAdopt,
                    id_source: None,
                    target: Target::Direct(Some(id)),
                });
            }
        }
        RewardKind::Potion => {
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
