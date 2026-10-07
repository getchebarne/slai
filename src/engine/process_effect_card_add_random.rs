use rand::Rng;

use crate::cards::ALL_CARDS;
use crate::cards::CardTemplate;
use crate::cards::get_card;
use crate::effect::EFFECT_ACCURACY_RESYNC_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::CostScope;
use crate::utils::card_name_bound_curse;
use crate::utils::card_name_healing;
use crate::utils::place_card;
use crate::utils::push_entity;

#[allow(clippy::too_many_arguments)]
pub fn process_effect_card_add_random(
    state: &mut GameState,
    color: CardColor,
    kind: Option<CardKind>,
    pile: CardPile,
    count: u16,
    cost_zero: Option<CostScope>,
    upgraded: bool,
    rarity: Option<CardRarity>,
) {
    let pool: Vec<&CardTemplate> = ALL_CARDS
        .iter()
        .filter(|card| card.color == color)
        .filter(|card| kind.is_none_or(|card_kind| card.kind == card_kind))
        .filter(|card| {
            rarity.map_or(
                matches!(
                    card.rarity,
                    CardRarity::Common | CardRarity::Uncommon | CardRarity::Rare
                ),
                |card_rarity| card.rarity == card_rarity,
            )
        })
        .filter(|card| !card_name_bound_curse(card.name))
        .filter(|card| card.kind != CardKind::Status)
        .filter(|card| !state.combat.active || !card_name_healing(card.name))
        .map(|card| &**card)
        .collect();

    let mut landed_in_hand = false;
    for _ in 0..count {
        let name = pool[state.rng.random_range(0..pool.len())].name;
        let id_card = push_entity(&mut state.entities, get_card(name, upgraded));

        // Deck additions route through the obtain hook
        if pile == CardPile::Deck {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardAdopt { landing: false },
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        } else {
            let placed = place_card(state, id_card, pile);
            landed_in_hand |= placed && pile == CardPile::Hand;
        }

        if let Some(scope) = cost_zero {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::SetCostOverride {
                    amount: 0,
                    only_reduce: false,
                    random: false,
                    scope,
                },
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
    }

    // Accuracy: a Card reaching the hand resets every Shiv in it
    if landed_in_hand
        && has_modifier(
            &state.entities[state.id_character].modifiers,
            ModifierKind::Accuracy,
        )
    {
        state.effect_queue.push_front(EFFECT_ACCURACY_RESYNC_HAND);
    }
}
