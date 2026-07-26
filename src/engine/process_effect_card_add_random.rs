use rand::Rng;

use crate::cards::ALL_CARDS;
use crate::cards::get_card;
use crate::entity::CostOverride;
use crate::entity::Entity;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::CostScope;
use crate::utils::place_card;
use crate::utils::push_entity;

// Rolls like StS returnTrulyRandom*InCombat: reward rarities only, independent rolls (dupes allowed)
pub fn process_effect_card_add_random(
    state: &mut GameState,
    color: CardColor,
    kind: Option<CardKind>,
    pile: CardPile,
    count: u8,
    cost_zero: Option<CostScope>,
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
        match cost_zero {
            // StS writes the base cost only when it is positive (Chrysalis/Metamorphosis)
            Some(CostScope::Combat) => {
                if card.card_cost > 0 {
                    card.card_cost = 0;
                }
            }
            Some(scope) => card.card_cost_override = Some(CostOverride { amount: 0, scope }),
            None => {}
        }
        let id_card = push_entity(&mut state.entities, card);
        place_card(state, id_card, pile);
    }
}
