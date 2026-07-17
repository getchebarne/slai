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

pub fn process_effect_card_transform(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardTransform requires id_target");
    let source_name = state.entities[id_card].card_name;

    // The target keeps the source's color — Curses only become other Curses, etc.
    let pools: &[&[CardName]] = match state.entities[id_card].card_color {
        // Pools already exclude Basic, Special, and AscendersBane
        CardColor::Green => &[
            POOL_COMMON_GREEN_CARD,
            POOL_UNCOMMON_GREEN_CARD,
            POOL_RARE_GREEN_CARD,
        ],
        CardColor::Colorless => &[POOL_UNCOMMON_COLORLESS_CARD, POOL_RARE_COLORLESS_CARD],
        CardColor::Curse => &[POOL_CURSE_CARD],
    };

    // Uniform across rarities (unlike card rewards), never into the same card
    let candidates: Vec<CardName> = pools
        .iter()
        .flat_map(|pool| pool.iter().copied())
        .filter(|&name| name != source_name)
        .collect();
    let card_name = candidates[state.rng.random_range(0..candidates.len())];

    // CardPurge pops first, then CardAddToDeck
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardAddToDeck {
            card_name,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    });
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}
