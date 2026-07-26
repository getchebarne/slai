use rand::Rng;

use crate::cards::ALL_CARDS;
use crate::cards::get_card;
use crate::entity::Entity;
use crate::entity::add_card_to_hand_or_discard;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardRarity;
use crate::types::Mode;
use crate::utils::push_entity;

// Rolls like StS returnTrulyRandom*InCombat: reward rarities only, independent rolls (dupes allowed)
pub fn process_effect_card_add_random(
    state: &mut GameState,
    color: CardColor,
    kind: Option<CardKind>,
    count: u8,
    into_draw: bool,
    cost_zero_turn: bool,
    cost_zero_combat: bool,
    upgraded: bool,
) {
    let pool: Vec<&Entity> = ALL_CARDS
        .iter()
        .filter(|c| c.card_color == color)
        .filter(|c| kind.is_none_or(|k| c.card_kind == k))
        .filter(|c| {
            matches!(
                c.card_rarity,
                CardRarity::Common | CardRarity::Uncommon | CardRarity::Rare
            )
        })
        .map(|c| &**c)
        .collect();
    if pool.is_empty() {
        return;
    }

    for _ in 0..count {
        let name = pool[state.rng.random_range(0..pool.len())].card_name;
        let mut card = get_card(name, upgraded);
        if cost_zero_combat && card.card_cost > 0 {
            card.card_cost = 0;
        }
        if cost_zero_turn {
            card.card_cost_override = Some(0);
        }
        let Mode::Combat {
            id_hand,
            id_pile_draw,
            id_pile_discard,
            ..
        } = &mut state.mode
        else {
            unreachable!("process_effect_card_add_random outside Combat mode")
        };
        if into_draw {
            let id = push_entity(&mut state.entities, card);
            let idx = state.rng.random_range(0..=id_pile_draw.len());
            id_pile_draw.insert(idx, id);
        } else {
            add_card_to_hand_or_discard(
                &mut state.entities,
                &mut *id_hand,
                &mut *id_pile_discard,
                card,
            );
        }
    }
}
