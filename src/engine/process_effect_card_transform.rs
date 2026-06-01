use rand::Rng;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::utils::roll_card_reward_pool_green;

pub fn process_effect_card_transform(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardTransform requires id_target");

    // Weighted rarity roll — can't collapse into SelectionKind::Random (single-stage uniform)
    let roll = state.rng.random_range(0..99);
    let (pool, _) = roll_card_reward_pool_green(roll);
    let card_name = pool[state.rng.random_range(0..pool.len())];

    // push_front in reverse: CardPurge pops first, then CardAddToDeck
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardAddToDeck {
            card_name,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    });
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}
