use crate::consts::MAX_SIZE_DECK;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::iter_owned_relics;
use crate::utils::push_entity;
use crate::utils::shuffle;

pub fn process_effect_combat_start(state: &mut GameState) {
    state.this_combat_damage_instances_taken = 0;
    state.this_combat_escaped = false;

    // Innate cards sit on top of the draw pile, ahead of the shuffled rest
    let mut other_ids: [usize; MAX_SIZE_DECK] = [0; MAX_SIZE_DECK];
    let mut other_n: usize = 0;
    let mut innate_ids: [usize; MAX_SIZE_DECK] = [0; MAX_SIZE_DECK];
    let mut innate_n: usize = 0;

    for i in 0..state.id_deck.len() {
        let id_card_src = state.id_deck[i];
        let card = state.entities[id_card_src];
        let id_card = push_entity(&mut state.entities, card);
        if card.card_innate {
            innate_ids[innate_n] = id_card;
            innate_n += 1;
        } else {
            other_ids[other_n] = id_card;
            other_n += 1;
        }
    }

    shuffle(&mut other_ids[..other_n], &mut state.rng);

    state.id_pile_draw.clear();
    for &id in &other_ids[..other_n] {
        state.id_pile_draw.push(id);
    }
    for &id in &innate_ids[..innate_n] {
        state.id_pile_draw.push(id);
    }

    state.id_picked_monster = None;

    // Monster MoveUpdates already queued at MonsterSpawn; queue character TurnStart
    state.effect_queue.push_front(Effect {
        kind: EffectKind::TurnStart,
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    });

    for (_name, id_relic) in iter_owned_relics(&state.id_relics) {
        for &eff in state.entities[id_relic].relic_effects_on_combat_start {
            state.effect_queue.push_back(eff);
        }
    }
}
