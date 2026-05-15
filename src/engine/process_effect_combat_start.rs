use std::collections::VecDeque;

use rand::Rng;
use strum::EnumCount;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::relics::iter_owned_relics;
use crate::types::RelicName;
use crate::utils::shuffle;
use crate::types::Phase;

pub fn process_effect_combat_start(
    id_character: usize,
    id_deck: &[usize],
    id_relics: &[Option<usize>; RelicName::COUNT],
    entities: &mut Vec<Entity>,
    id_pile_draw: &mut Vec<usize>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    id_pile_exhaust: &mut Vec<usize>,
    id_card_target: &mut Option<usize>,
    this_combat_damage_instances_taken: &mut u8,
    escaped_this_combat: &mut bool,
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    *this_combat_damage_instances_taken = 0;
    *escaped_this_combat = false;
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

    // Monsters already had MoveUpdate queued at MonsterSpawn time, so we only
    // need to queue TurnStart for the character here
    effect_queue.push_front(Effect {
        kind: EffectKind::TurnStart,
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });

    // Each owned relic's combat-start effects run after TurnStart
    for (_name, id_relic) in iter_owned_relics(id_relics) {
        for &eff in entities[id_relic].relic_effects_on_combat_start {
            effect_queue.push_back(eff);
        }
    }

    None
}
