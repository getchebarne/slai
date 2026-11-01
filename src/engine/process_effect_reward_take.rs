use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Key;
use crate::types::Reward;
use crate::types::RewardKind;

// Claims hand the staged reward entity to the matching Adopt effect, which owns registration and any on-pickup behavior; the Sapphire Key obtains the key and drops its linked Relic
pub fn process_effect_reward_take(
    id_target: Option<usize>,
    state: &mut GameState,
    kind: RewardKind,
) {
    assert!(state.reward.active, "RewardTake outside the Reward context");
    let Reward {
        id_cards: bundles,
        id_relics,
        id_relic_linked,
        id_potions,
        gold,
        gold_stolen,
        ..
    } = &mut state.reward;

    // Each `expect` stays inside its arm: the gold takes and the Sapphire Key carry no id_target
    let (id_taken, kind_adopt) = match kind {
        // Card: taking one consumes its whole bundle; sibling bundles stay claimable
        RewardKind::Card => {
            let id_card = id_target.expect("RewardTake { Card } requires id_target");
            let idx = bundles
                .iter()
                .position(|bundle| bundle.contains(&id_card))
                .expect("Taken Card is a staged bundle");
            bundles.remove(idx);
            (id_card, EffectKind::CardAdopt { landing: false })
        }

        // Relic: unstage the selection; RelicAdopt owns registration and pickup effects
        RewardKind::Relic => {
            let id_relic = id_target.expect("RewardTake { Relic } requires id_target");
            let idx = id_relics
                .iter()
                .position(|&id| id == id_relic)
                .expect("Taken Relic is a staged reward");
            id_relics.remove(idx);

            // Taking the linked Relic withdraws its Sapphire Key, ahead of any pick its pickup raises
            if *id_relic_linked == Some(id_relic) {
                *id_relic_linked = None;
            }
            (id_relic, EffectKind::RelicAdopt)
        }

        // Sapphire Key: its linked Relic leaves unobtained; the rest of the Reward stays on offer
        RewardKind::SapphireKey => {
            // Take the linked Relic from its dedicated slot
            let id_relic = id_relic_linked
                .take()
                .expect("RewardTake { SapphireKey } requires the key on offer");

            // Drop the linked Relic from the staged rewards
            id_relics.retain(|&id| id != id_relic);

            // Push Effect to obtain the Sapphire Key
            state.effect_queue.push_front(Effect {
                kind: EffectKind::KeyObtain { key: Key::Sapphire },
                id_source: None,
                target: Target::Direct(None),
            });
            return;
        }

        // Potion: unstage the selection; PotionAdopt owns the belt slot (and the Sozu guard)
        RewardKind::Potion => {
            let id_potion = id_target.expect("RewardTake { Potion } requires id_target");
            let idx = id_potions
                .iter()
                .position(|&id| id == id_potion)
                .expect("Taken Potion is a staged reward");
            id_potions.remove(idx);
            (id_potion, EffectKind::PotionAdopt)
        }

        // Gold or the thieves' purse: routed through GoldDelta so the MAX_GOLD cap and Ectoplasm apply
        RewardKind::Gold | RewardKind::GoldStolen => {
            let offer = if kind == RewardKind::Gold {
                gold
            } else {
                gold_stolen
            };
            if let Some(amount) = offer.take() {
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

    // A selection screen closes on its selection
    if state.reward.closes_on_selection {
        state.reward.active = false;
    }

    state.effect_queue.push_front(Effect {
        kind: kind_adopt,
        id_source: None,
        target: Target::Direct(Some(id_taken)),
    });
}
