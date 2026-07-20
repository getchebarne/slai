use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::cards::get_card;
use crate::entity::add_card_to_hand_or_discard;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::Mode;
use rand::Rng;

// Random Silent Skill (not Distraction) into hand, free-to-play-once
pub fn process_effect_distraction_add(state: &mut GameState) {
    let Mode::Combat(combat) = &mut state.mode else {
        unreachable!("process_effect_distraction_add outside Combat mode")
    };
    let mut buf = [CardName::Strike; 64];
    let mut n = 0;
    for pool in [
        POOL_COMMON_GREEN_CARD,
        POOL_UNCOMMON_GREEN_CARD,
        POOL_RARE_GREEN_CARD,
    ] {
        for &name in pool {
            if name == CardName::Distraction {
                continue;
            }
            if get_card(name, false).card_kind != CardKind::Skill {
                continue;
            }
            buf[n] = name;
            n += 1;
        }
    }
    if n == 0 {
        return;
    }

    let card_name = buf[state.rng.random_range(0..n)];
    let mut card = get_card(card_name, false);
    card.card_free_to_play_once = true;

    add_card_to_hand_or_discard(
        &mut state.entities,
        &mut combat.id_hand,
        &mut combat.id_pile_discard,
        card,
    );
}
