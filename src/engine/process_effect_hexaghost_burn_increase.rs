use crate::cards::get_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::Frame;
use crate::utils::frame_top_mut;

// Upgrade existing Burns in the draw and discard piles, then add `count`
// upgraded Burns to the discard pile
pub fn process_effect_hexaghost_burn_increase(state: &mut GameState, count: u8) {
    let Frame::Combat {
        id_pile_draw,
        id_pile_discard,
        ..
    } = frame_top_mut(&mut state.frame_stack)
    else {
        unreachable!("process_effect_hexaghost_burn_increase outside the Combat frame")
    };
    let burn_upgraded = get_card(CardName::Burn, true);
    for i in 0..id_pile_draw.len() {
        let id_card = id_pile_draw[i];
        if state.entities[id_card].card_name == CardName::Burn
            && !state.entities[id_card].card_upgraded
        {
            state.entities[id_card] = burn_upgraded;
        }
    }
    for i in 0..id_pile_discard.len() {
        let id_card = id_pile_discard[i];
        if state.entities[id_card].card_name == CardName::Burn
            && !state.entities[id_card].card_upgraded
        {
            state.entities[id_card] = burn_upgraded;
        }
    }

    if count > 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Burn,
                pile: CardPile::Discard,
                count: count as u16,
                upgraded: true,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
