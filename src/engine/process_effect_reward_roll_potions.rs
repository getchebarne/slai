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
        if policy.drop_chance {
            // White Beast Statue pins the chance at 100; the roll and its drift still happen
            let statue = has_relic(&state.id_relics, RelicName::WhiteBeastStatue);
            if !eligible && !statue {
                state.potion_drop_mod += POTION_DROP_CHANCE_MOD_MISS;
                continue;
            }
            let chance = if statue {
                100
            } else {
                potion_drop_chance(state.potion_drop_mod)
            };
            if !roll_potion_drop(&mut state.rng, &mut state.potion_drop_mod, chance) {
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
            let sacred_bark = has_relic(&state.id_relics, RelicName::SacredBark);
            let id_potion = push_entity(&mut state.entities, get_potion(name, sacred_bark));
            reward_ensure(&mut state.reward);
            state.reward.id_potions.push(id_potion);
        }
    }
}

// The drift shifts the base chance; only the chance clamps to 0..=100
fn potion_drop_chance(potion_drop_mod: i32) -> u8 {
    (POTION_DROP_CHANCE_BASE + potion_drop_mod).clamp(0, 100) as u8
}

// +10 on miss, -10 on hit; the drift is unclamped
fn roll_potion_drop(rng: &mut impl Rng, potion_drop_mod: &mut i32, chance: u8) -> bool {
    let roll = rng.random_range(0..100) as u8;

    if roll < chance {
        *potion_drop_mod += POTION_DROP_CHANCE_MOD_HIT;
        true
    } else {
        *potion_drop_mod += POTION_DROP_CHANCE_MOD_MISS;
        false
    }
}
