use crate::effect::RewardRollTrigger;
use crate::game::GameState;
use crate::types::reward_ensure;
use crate::utils::card_reward_count;
use crate::utils::roll_card_rewards;
use crate::utils::roll_policy;

// Roll `bundles` Card bundles; the trigger sets the rules, whether they are kept and whether the selection closes the Reward
pub fn process_effect_reward_roll_cards(
    state: &mut GameState,
    bundles: u8,
    trigger: RewardRollTrigger,
) {
    let cards_per_bundle = card_reward_count(&state.id_relics);
    let mut id_card_bundles: Vec<Vec<usize>> = Vec::with_capacity(bundles as usize);
    for _ in 0..bundles {
        let mut id_cards: Vec<usize> = Vec::with_capacity(cards_per_bundle);
        roll_card_rewards(
            state.id_character,
            &mut state.entities,
            &mut state.rng,
            &mut id_cards,
            &state.id_relics,
            cards_per_bundle,
            trigger,
            state.act,
            state.ascension,
        );
        id_card_bundles.push(id_cards);
    }

    // A discarded roll still moved the pity offset
    if !roll_policy(trigger).staged {
        return;
    }
    reward_ensure(&mut state.reward);
    state.reward.id_cards.extend(id_card_bundles);

    // Dream Catcher's offer is a selection screen of its own
    if trigger == RewardRollTrigger::DreamCatcher {
        state.reward.closes_on_selection = true;
    }
}
