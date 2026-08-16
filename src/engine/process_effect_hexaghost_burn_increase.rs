use crate::cards::get_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::Combat;

// Upgrade existing Burns in the draw and discard piles, then add `count`
// upgraded Burns to the discard pile
pub fn process_effect_hexaghost_burn_increase(state: &mut GameState, count: u8) {
    assert!(
        state.combat.active,
        "process_effect_hexaghost_burn_increase outside the Combat frame"
    );
    let Combat {
        id_card_draw,
        id_card_discard,
        ..
    } = &mut state.combat;
    let burn_upgraded = get_card(CardName::Burn, true);
    for idx in 0..id_card_draw.len() {
        let id_card = id_card_draw[idx];
        if state.entities[id_card].card_name == CardName::Burn
            && !state.entities[id_card].card_upgraded
        {
            state.entities[id_card] = burn_upgraded;
        }
    }
    for idx in 0..id_card_discard.len() {
        let id_card = id_card_discard[idx];
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
