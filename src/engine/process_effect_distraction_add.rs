use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::cards::get_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CostScope;
use crate::utils::place_card;
use crate::utils::push_entity;
use rand::Rng;

// Random Silent Skill (not Distraction) into hand, free-to-play-once
pub fn process_effect_distraction_add(state: &mut GameState) {
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
    let id_card = push_entity(&mut state.entities, get_card(card_name, false));
    place_card(state, id_card, CardPile::Hand);

    // Costs 0 this turn
    state.effect_queue.push_front(Effect {
        kind: EffectKind::SetCostOverride {
            amount: 0,
            only_reduce: false,
            random: false,
            scope: CostScope::Turn,
        },
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}
