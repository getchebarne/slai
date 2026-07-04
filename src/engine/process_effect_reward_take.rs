use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::GoldDeltaKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::potions::find_free_slot;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RewardKind;

pub fn process_effect_reward_take(
    id_target: Option<usize>,
    state: &mut GameState,
    kind: RewardKind,
) {
    match kind {
        RewardKind::Card => {
            let id_card = id_target.expect("RewardTake { Card } requires id_target");
            state.id_deck.push(id_card);
            state.reward_id_cards.clear();
            // Ceramic Fish: this deck-add bypasses CardAddToDeck, so pay here too
            if state.id_relics[RelicName::CeramicFish as usize].is_some() {
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::GoldDelta {
                        sign: DeltaSign::Gain,
                        kind: GoldDeltaKind::Fixed(9),
                    },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }
        }
        RewardKind::Relic => {
            // The staged reward entity is orphaned; RelicGrantSpecific spawns the
            // owned copy and fires on-pickup, keeping acquisition a single effect
            if let Some(id) = state.reward_id_relic.take() {
                let name = state.entities[id].relic_name;
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::RelicGrantSpecific {
                        name,
                        fallback_circlet: false,
                    },
                    id_source: None,
                    target: Target::Direct(None),
                });
            }
        }
        RewardKind::Potion => {
            if let Some(id) = state.reward_id_potion.take() {
                let slot = find_free_slot(&state.id_potions, state.potion_slots_max)
                    .expect("RewardTake { Potion }: belt full (action handler should have rejected)");
                state.id_potions[slot] = Some(id);
            }
        }
        RewardKind::Gold => {
            if let Some(amount) = state.reward_gold.take() {
                let character = &mut state.entities[state.id_character];
                character.character_gold = character.character_gold.saturating_add(amount);
            }
        }
    }
}
