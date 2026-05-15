use rand::Rng;
use strum::EnumCount;

use crate::entity::Entity;
use crate::potions::get_potion;
use crate::potions::get_random_potion;
use crate::types::Phase;
use crate::types::RelicName;
use crate::utils::add_relic_reward_for_roll;
use crate::utils::roll_card_rewards;

pub fn process_effect_reward_roll_combat(
    id_character: usize,
    gold_range: Option<(u16, u16)>,
    relic_thresholds: Option<(u8, u8)>,
    potion_drop: bool,
    id_relics: &[Option<usize>; RelicName::COUNT],
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> Option<Phase> {
    let id_cards = roll_card_rewards(id_character, entities, rng);
    let gold = gold_range.map(|(min, max)| rng.random_range(min..=max));
    let id_relic = relic_thresholds.map(|(th_c, th_u)| {
        let roll = rng.random_range(0..100) as u8;
        add_relic_reward_for_roll(roll, th_c, th_u, id_relics, entities, rng)
    });
    let id_potion = potion_drop.then(|| {
        let name = get_random_potion(rng, false);
        let id = entities.len();
        entities.push(get_potion(name));
        id
    });
    Some(Phase::Reward { id_cards, id_relic, id_potion, gold })
}
