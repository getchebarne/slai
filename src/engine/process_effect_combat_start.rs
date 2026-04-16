use std::collections::VecDeque;

use rand::Rng;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::utils::shuffle;

pub fn process_effect_combat_start(
    character: usize,
    deck: &[usize],
    entities: &mut Vec<Entity>,
    draw_pile: &mut Vec<usize>,
    hand: &mut Vec<usize>,
    discard_pile: &mut Vec<usize>,
    exhaust_pile: &mut Vec<usize>,
    card_target: &mut Option<usize>,
    monsters: &[usize],
    monster_count: u8,
    rng: &mut impl Rng,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    // Clone deck cards into combat copies, separating innate from non-innate.
    // These small local Vecs could become stack buffers, but deck size is
    // unbounded at design level (decks grow across a run), so heap is the
    // right call here.
    let mut innate_ids: Vec<usize> = Vec::new();
    let mut other_ids: Vec<usize> = Vec::new();

    for &deck_id in deck {
        let card = entities[deck_id];
        let id = entities.len();
        entities.push(card);
        if card.card_innate {
            innate_ids.push(id);
        } else {
            other_ids.push(id);
        }
    }

    shuffle(&mut other_ids, rng);
    *draw_pile = other_ids;
    draw_pile.extend(innate_ids);

    hand.clear();
    discard_pile.clear();
    exhaust_pile.clear();
    *card_target = None;

    // Queue initial effects: MoveUpdate for each monster (original order), then TurnStart.
    // Push TurnStart first (it runs last), then monsters in reverse.
    queue.push_front(Effect {
        kind: EffectKind::TurnStart,
        source: None,
        target: Target::Direct(Some(character)),
    });
    for &id in monsters[..monster_count as usize].iter().rev() {
        queue.push_front(Effect {
            kind: EffectKind::MoveUpdate,
            source: None,
            target: Target::Direct(Some(id)),
        });
    }

    DispatchResult::Continue
}
