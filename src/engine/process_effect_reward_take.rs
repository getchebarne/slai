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
    let Mode::Reward {
        reward_id_cards: bundles,
        reward_id_relics,
        reward_id_potions,
        reward_gold,
    } = mode_top_mut(&mut state.mode_stack)
    else {
        unreachable!("RewardTake outside Reward mode")
    };

    // Each `expect` stays inside its arm: Gold legitimately carries no id_target
    let (id_taken, kind_adopt) = match kind {
        // Card: taking one consumes its whole bundle; sibling bundles stay claimable
        RewardKind::Card => {
            let id_card = id_target.expect("RewardTake { Card } requires id_target");
            let idx = bundles
                .iter()
                .position(|bundle| bundle.contains(&id_card))
                .expect("Taken Card is a staged bundle");
            bundles.remove(idx);
            (id_card, EffectKind::CardAddToDeck)
        }

        // Relic: unstage the pick; RelicAdopt owns registration and pickup effects
        RewardKind::Relic => {
            let id_relic = id_target.expect("RewardTake { Relic } requires id_target");
            let idx = reward_id_relics
                .iter()
                .position(|&id| id == id_relic)
                .expect("Taken Relic is a staged reward");
            reward_id_relics.remove(idx);
            (id_relic, EffectKind::RelicAdopt)
        }

        // Potion: unstage the pick; PotionAdopt owns the belt slot (and the Sozu guard)
        RewardKind::Potion => {
            let id_potion = id_target.expect("RewardTake { Potion } requires id_target");
            let idx = reward_id_potions
                .iter()
                .position(|&id| id == id_potion)
                .expect("Taken Potion is a staged reward");
            reward_id_potions.remove(idx);
            (id_potion, EffectKind::PotionAdopt)
        }

        // Gold: routed through GoldDelta so the MAX_GOLD cap and Ectoplasm apply
        RewardKind::Gold => {
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
            return;
        }
    };

    state.effect_queue.push_front(Effect {
        kind: kind_adopt,
        id_source: None,
        target: Target::Direct(Some(id_taken)),
    });
}
