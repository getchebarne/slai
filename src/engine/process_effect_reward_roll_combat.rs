use rand::Rng;

use crate::engine::DispatchResult;
use crate::engine::enter_reward;
use crate::entity::Entity;
use crate::potions::get_potion;
use crate::potions::get_random_potion;
use crate::types::Phase;
use crate::utils::add_relic_reward_for_roll;
use crate::utils::roll_card_rewards;

use strum::EnumCount;
use crate::types::RelicName;

pub fn process_effect_reward_roll_combat(
    id_character: usize,
    gold_range: Option<(u16, u16)>,
    relic_thresholds: Option<(u8, u8)>,
    potion_drop: bool,
    id_relics: &[Option<usize>; RelicName::COUNT],
    phase: &mut Phase,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> DispatchResult {
    let rolled_cards = roll_card_rewards(id_character, entities, rng);
    let rolled_gold = gold_range.map(|(min, max)| rng.random_range(min..=max));
    let rolled_relic = relic_thresholds.map(|(th_c, th_u)| {
        let roll = rng.random_range(0..100) as u8;
        add_relic_reward_for_roll(roll, th_c, th_u, id_relics, entities, rng)
    });
    let rolled_potion = if potion_drop {
        let name = get_random_potion(rng, false);
        let id = entities.len();
        entities.push(get_potion(name));
        Some(id)
    } else {
        None
    };

    let Phase::Reward { id_cards, id_relic, id_potion, gold } = enter_reward(phase) else {
        unreachable!()
    };
    id_cards.extend(rolled_cards);
    if rolled_relic.is_some() {
        *id_relic = rolled_relic;
    }
    if rolled_potion.is_some() {
        *id_potion = rolled_potion;
    }
    if rolled_gold.is_some() {
        *gold = rolled_gold;
    }

    DispatchResult::Continue
}
