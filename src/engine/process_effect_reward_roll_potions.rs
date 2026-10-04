use rand::Rng;

use crate::consts::POTION_DROP_CHANCE_BASE;
use crate::consts::POTION_DROP_CHANCE_MOD_HIT;
use crate::consts::POTION_DROP_CHANCE_MOD_MISS;
use crate::effect::RewardRollTrigger;
use crate::game::GameState;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::potions::get_random_potion_name_uniform;
use crate::types::RelicName;
use crate::types::reward_ensure;
use crate::utils::has_relic;
use crate::utils::potion_roll_policy;
use crate::utils::push_entity;

// Every Potion reward roll; the trigger says who asked, its policy says how
pub fn process_effect_reward_roll_potions(
    state: &mut GameState,
    count: u8,
    trigger: RewardRollTrigger,
) {
    let policy = potion_roll_policy(trigger);

    // Escaped normal fights roll chance 0
    let eligible =
        !(trigger == RewardRollTrigger::CombatMonster && state.combat.this_combat_escaped);

    for _ in 0..count {
        // White Beast Statue guarantees the drop; otherwise it's a drifting chance
        if policy.drop_chance && !has_relic(&state.id_relics, RelicName::WhiteBeastStatue) {
            if !eligible {
                state.potion_drop_mod += POTION_DROP_CHANCE_MOD_MISS;
                continue;
            }
            if !roll_potion_drop(&mut state.rng, &mut state.potion_drop_mod) {
                continue;
            }
        }

        let name = if policy.uniform {
            get_random_potion_name_uniform(&mut state.rng)
        } else {
            get_random_potion_name(&mut state.rng, false)
        };

        // Sozu doesn't stop the roll: the staged Potion adopts to nothing
        if policy.staged {
            let id_potion = push_entity(&mut state.entities, get_potion(name));
            reward_ensure(&mut state.reward);
            state.reward.id_potions.push(id_potion);
        }
    }
}

// +10 on miss, -10 on hit; the drift is unclamped
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
