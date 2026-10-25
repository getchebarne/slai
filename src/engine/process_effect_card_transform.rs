use rand::Rng;

use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_CURSE_CARD;
use crate::cards::POOL_RARE_COLORLESS_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_COLORLESS_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardName;
use crate::types::CardPile;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_card_transform(id_targets: &[usize], state: &mut GameState, upgraded: bool) {
    assert!(!id_targets.is_empty(), "CardTransform requires id_target");

    // Each Card rolls its replacement in the list's order, then each is purged and its replacement added, in the same order
    state.effect_buf.clear();
    for &id_card in id_targets {
        let source_name = state.entities[id_card].card_name;

        // The target keeps the source's color — Curses only become other Curses, etc.
        // Curse results are never upgradable, so Astrolabe's upgrade skips them
        let (pools, upgradable): (&[&[CardName]], bool) = match state.entities[id_card].card_color {
            // Pools already exclude Basic, Special, and AscendersBane
            CardColor::Green => (
                &[
                    POOL_COMMON_GREEN_CARD,
                    POOL_UNCOMMON_GREEN_CARD,
                    POOL_RARE_GREEN_CARD,
                ],
                true,
            ),
            CardColor::Colorless => (
                &[POOL_UNCOMMON_COLORLESS_CARD, POOL_RARE_COLORLESS_CARD],
                true,
            ),
            CardColor::Curse => (&[POOL_CURSE_CARD], false),
        };

        // Uniform across rarities (unlike Card rewards), never into the same Card
        let candidates: Vec<CardName> = pools
            .iter()
            .flat_map(|pool| pool.iter().copied())
            .filter(|&name| name != source_name)
            .collect();
        let card_name = candidates[state.rng.random_range(0..candidates.len())];
        state.effect_buf.push(Effect {
            kind: EffectKind::CardPurge,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
        state.effect_buf.push(Effect {
            kind: EffectKind::CardAdd {
                card_name,
                card_pile: CardPile::Deck,
                count: 1,
                upgraded: upgraded && upgradable,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
    flush_effects_from_buf_to_queue_front(state);
}
