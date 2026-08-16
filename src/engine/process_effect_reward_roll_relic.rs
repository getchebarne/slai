use rand::Rng;

use crate::effect::RelicPick;
use crate::game::GameState;
use crate::relics::get_relic;
use crate::types::RelicName;
use crate::types::reward_ensure;
use crate::utils::has_relic;
use crate::utils::pick_relic_by_roll;
use crate::utils::pick_relic_by_tier;
use crate::utils::push_entity;

pub fn process_effect_reward_roll_relic(state: &mut GameState, pick: RelicPick) {
    reward_ensure(&mut state.reward);

    // Exclude owned and already-staged Relics, so repeat picks never duplicate
    // (the boss triple, Black Star's second roll, Matryoshka before the chest's own)
    let mut id_relics_aux = state.id_relics;
    for &id in &state.reward.id_relics {
        id_relics_aux[state.entities[id].relic_name as usize] = Some(id);
    }

    let name = match pick {
        RelicPick::Thresholds {
            th_common,
            th_uncommon,
        } => pick_relic_by_roll(
            state.rng.random_range(0..100) as u8,
            th_common,
            th_uncommon,
            &id_relics_aux,
            &mut state.rng,
        ),
        RelicPick::Tier(tier) => pick_relic_by_tier(tier, &id_relics_aux, &mut state.rng),
        // Circlet substitutes when the named Relic is already owned
        RelicPick::Name(name) => {
            if has_relic(&state.id_relics, name) {
                RelicName::Circlet
            } else {
                name
            }
        }
    };

    let id = push_entity(&mut state.entities, get_relic(name));
    state.reward.id_relics.push(id);
}
