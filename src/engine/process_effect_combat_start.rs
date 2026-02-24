use rand::Rng;

use crate::cards::Card;
use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::types::EntityId;
use crate::utils::shuffle;

pub fn process_effect_combat_start(
    deck: &[Card],
    combat_cards: &mut Vec<Card>,
    draw_pile: &mut Vec<usize>,
    hand: &mut Vec<usize>,
    discard_pile: &mut Vec<usize>,
    exhaust_pile: &mut Vec<usize>,
    card_active: &mut Option<usize>,
    card_target: &mut Option<EntityId>,
    monster_ids: &[EntityId],
    character_id: EntityId,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    *combat_cards = deck.to_vec();
    let n = combat_cards.len();

    let mut innate_indices: Vec<usize> = Vec::new();
    let mut other_indices: Vec<usize> = Vec::new();
    for i in 0..n {
        if combat_cards[i].innate {
            innate_indices.push(i);
        } else {
            other_indices.push(i);
        }
    }

    shuffle(&mut other_indices, rng);

    *draw_pile = innate_indices;
    draw_pile.extend(other_indices);

    hand.clear();
    discard_pile.clear();
    exhaust_pile.clear();
    *card_active = None;
    *card_target = None;

    let mut effects: Vec<Effect> = Vec::new();
    for &id in monster_ids {
        effects.push(Effect::MoveUpdate { monster: id });
    }
    effects.push(Effect::TurnStart { actor: character_id });

    ProcessEffectResult::Continue {
        top: effects,
        bot: Vec::new(),
    }
}
