use crate::cards::get_random_cards_of_kind_and_color;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::engine::push_entity;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardKind;

pub fn process_effect_card_discover_select(
    state: &mut GameState,
    kind: CardKind,
    color: CardColor,
    count: u8,
) {
    let card_picks =
        get_random_cards_of_kind_and_color(kind, color, count as usize, &mut state.rng);
    state.id_pick.clear();
    for card_pick in card_picks {
        let id = push_entity(&mut state.entities, card_pick);
        state.id_pick.push(id);
    }
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardDiscoverPick,
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::IdPick,
            selection: SelectionKind::Input { count: 1 },
        },
    });
}
