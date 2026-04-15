use rand::Rng;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::entities::Entity;
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
) -> ProcessEffectResult {
    // Clone deck cards into combat copies, separating innate from non-innate
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

    // Build draw pile: shuffled non-innate on bottom, innate on top
    shuffle(&mut other_ids, rng);
    *draw_pile = other_ids;
    draw_pile.extend(innate_ids);

    // Reset combat piles
    hand.clear();
    discard_pile.clear();
    exhaust_pile.clear();
    *card_target = None;

    // Queue initial monster moves and character's first turn
    let mut effects: Vec<Effect> = Vec::new();
    for &id in &monsters[..monster_count as usize] {
        effects.push(Effect {
            kind: EffectKind::MoveUpdate,
            source: None,
            target: Target::Direct(Some(id)),
        });
    }
    effects.push(Effect {
        kind: EffectKind::TurnStart,
        source: None,
        target: Target::Direct(Some(character)),
    });

    // Add and continue
    ProcessEffectResult::Continue {
        top: effects,
        bot: Vec::new(),
    }
}
