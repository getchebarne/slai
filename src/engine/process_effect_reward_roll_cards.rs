use crate::consts::MAX_COMBAT_CARD_REWARD;
use crate::game::GameState;
use crate::types::reward_ensure;
use crate::utils::card_reward_count;
use crate::utils::roll_card_rewards;

// Stage `bundles` combat-style Card bundles (Busted Crown and Question Card apply)
pub fn process_effect_reward_roll_cards(state: &mut GameState, bundles: u8, rare_only: bool) {
    let cards_per_bundle = card_reward_count(&state.id_relics);
    let mut id_card_bundles: Vec<Vec<usize>> = Vec::with_capacity(bundles as usize);
    for _ in 0..bundles {
        let mut id_cards: Vec<usize> = Vec::with_capacity(MAX_COMBAT_CARD_REWARD);
        roll_card_rewards(
            state.id_character,
            &mut state.entities,
            &mut state.rng,
            &mut id_cards,
            &state.id_relics,
            cards_per_bundle,
            rare_only,
        );
        id_card_bundles.push(id_cards);
    }

    reward_ensure(&mut state.reward);
    state.reward.id_cards.extend(id_card_bundles);
}
