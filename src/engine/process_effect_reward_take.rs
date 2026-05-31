use crate::game::GameState;
use crate::potions::find_free_slot;
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
        }
        RewardKind::Relic => {
            if let Some(id) = state.reward_id_relic.take() {
                let name = state.entities[id].relic_name;
                state.id_relics[name as usize] = Some(id);
            }
        }
        RewardKind::Potion => {
            if let Some(id) = state.reward_id_potion.take() {
                let character = &mut state.entities[state.id_character];
                let slot = find_free_slot(
                    &character.character_potion_slots,
                    character.character_potion_slots_max,
                )
                .expect("RewardTake { Potion }: belt full (action handler should have rejected)");
                character.character_potion_slots[slot] = Some(id);
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
