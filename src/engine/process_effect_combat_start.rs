use std::collections::VecDeque;

use rand::Rng;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::utils::shuffle;

pub fn process_effect_combat_start(
    id_character: usize,
    id_deck: &[usize],
    entities: &mut Vec<Entity>,
    id_pile_draw: &mut Vec<usize>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    id_pile_exhaust: &mut Vec<usize>,
    id_card_target: &mut Option<usize>,
    id_monsters: &[usize],
    monster_count: u8,
    this_combat_damage_instances_taken: &mut u8,
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    *this_combat_damage_instances_taken = 0;
    // Clone deck cards into combat copies, separating innate from non-innate
    // These small local Vecs could become stack buffers, but deck size is
    // unbounded at design level (decks grow across a run), so heap is the
    // right call here
    let mut innate_ids: Vec<usize> = Vec::new();
    let mut other_ids: Vec<usize> = Vec::new();

    for &id_card_src in id_deck {
        let card = entities[id_card_src];
        let id_card = entities.len();
        entities.push(card);
        if card.card_innate {
            innate_ids.push(id_card);
        } else {
            other_ids.push(id_card);
        }
    }

    shuffle(&mut other_ids, rng);
    *id_pile_draw = other_ids;
    id_pile_draw.extend(innate_ids);

    id_hand.clear();
    id_pile_discard.clear();
    id_pile_exhaust.clear();
    *id_card_target = None;

    // Queue initial effects: MoveUpdate for each monster (original order), then TurnStart
    // Push TurnStart first (it runs last), then monsters in reverse
    effect_queue.push_front(Effect {
        kind: EffectKind::TurnStart,
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });
    for &id_monster in id_monsters[..monster_count as usize].iter().rev() {
        effect_queue.push_front(Effect {
            kind: EffectKind::MoveUpdate,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }

    DispatchResult::Continue
}
