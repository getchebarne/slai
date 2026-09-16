use rand::Rng;

use crate::consts::POTION_DROP_CHANCE_BASE;
use crate::consts::POTION_DROP_CHANCE_MOD_HIT;
use crate::consts::POTION_DROP_CHANCE_MOD_MISS;
use crate::game::GameState;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::types::RelicName;
use crate::types::reward_ensure;
use crate::utils::has_relic;
use crate::utils::push_entity;

// The drifting end-of-combat Potion drop; `staged: false` rolls and drifts but keeps nothing
// (Smoke Bomb, Colosseum's unpaid bout)
pub fn process_effect_reward_roll_potion(state: &mut GameState, eligible: bool, staged: bool) {
    // White Beast Statue: guaranteed drop, bypassing the drifting chance roll
    let has_white_beast_statue = has_relic(&state.id_relics, RelicName::WhiteBeastStatue);

    // Sozu doesn't stop the roll: the staged Potion adopts to nothing
    if has_white_beast_statue
        || (eligible && roll_potion_drop(&mut state.rng, &mut state.potion_drop_mod))
    {
        let name = get_random_potion_name(&mut state.rng, false);
        if staged {
            let id = push_entity(&mut state.entities, get_potion(name));
            reward_ensure(&mut state.reward);
            state.reward.id_potions.push(id);
        }
    } else if !eligible {
        // Escaped normal fights roll chance 0 in the source: no Potion, but the miss drift lands
        state.potion_drop_mod += POTION_DROP_CHANCE_MOD_MISS;
    }
}

// +10 on miss, -10 on hit; blizzardPotionMod has no clamp
fn roll_potion_drop(rng: &mut impl Rng, potion_drop_mod: &mut i8) -> bool {
    let roll = rng.random_range(0..100) as u8;
    let chance = (POTION_DROP_CHANCE_BASE as i16 + *potion_drop_mod as i16).clamp(0, 100) as u8;

    if roll < chance {
        *potion_drop_mod += POTION_DROP_CHANCE_MOD_HIT;
        true
    } else {
        *potion_drop_mod += POTION_DROP_CHANCE_MOD_MISS;
        false
    }
}
