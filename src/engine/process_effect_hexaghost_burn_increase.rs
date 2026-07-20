use crate::cards::get_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::Mode;

// Upgrade existing Burns in draw+discard, then add `count` upgraded Burns to discard
pub fn process_effect_hexaghost_burn_increase(state: &mut GameState, count: u8) {
    let Mode::Combat(combat) = &mut state.mode else {
        unreachable!("process_effect_hexaghost_burn_increase outside Combat mode")
    };
    let burn_upgraded = get_card(CardName::Burn, true);
    for i in 0..combat.id_pile_draw.len() {
        let id_card = combat.id_pile_draw[i];
        if state.entities[id_card].card_name == CardName::Burn
            && !state.entities[id_card].card_upgraded
        {
            state.entities[id_card] = burn_upgraded;
        }
    }
    for i in 0..combat.id_pile_discard.len() {
        let id_card = combat.id_pile_discard[i];
        if state.entities[id_card].card_name == CardName::Burn
            && !state.entities[id_card].card_upgraded
        {
            state.entities[id_card] = burn_upgraded;
        }
    }

    if count > 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardAddToDiscard {
                card_name: CardName::Burn,
                count,
                upgraded: true,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
