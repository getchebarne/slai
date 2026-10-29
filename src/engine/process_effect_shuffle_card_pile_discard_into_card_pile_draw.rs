use crate::game::GameState;
use crate::types::Combat;
use crate::utils::shuffle;

// A reshuffle's Relic hooks run as ShuffleRelicHooks, which every caller queues ahead of it
pub fn process_effect_shuffle_card_pile_discard_into_card_pile_draw(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_shuffle_card_pile_discard_into_card_pile_draw outside the Combat frame"
    );
    let Combat {
        id_card_pile_draw,
        id_card_pile_discard,
        ..
    } = &mut state.combat;

    // A reshuffled Card costs its combat cost again this turn
    for &id_card in id_card_pile_discard.iter() {
        state.entities[id_card].card_cost_override = None;
    }
    id_card_pile_draw.append(id_card_pile_discard);
    shuffle(&mut id_card_pile_draw[..], &mut state.rng);
}
