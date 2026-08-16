use rand::Rng;

use crate::consts::CHEST_LARGE_GOLD_BASE;
use crate::consts::CHEST_LARGE_GOLD_CHANCE;
use crate::consts::CHEST_LARGE_TH_COMMON;
use crate::consts::CHEST_LARGE_TH_UNCOMMON;
use crate::consts::CHEST_MEDIUM_GOLD_BASE;
use crate::consts::CHEST_MEDIUM_GOLD_CHANCE;
use crate::consts::CHEST_MEDIUM_TH_COMMON;
use crate::consts::CHEST_MEDIUM_TH_UNCOMMON;
use crate::consts::CHEST_SMALL_GOLD_BASE;
use crate::consts::CHEST_SMALL_GOLD_CHANCE;
use crate::consts::CHEST_SMALL_TH_COMMON;
use crate::consts::CHEST_SMALL_TH_UNCOMMON;
use crate::consts::MATRYOSHKA_TH_COMMON;
use crate::consts::MATRYOSHKA_TH_UNCOMMON;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicPick;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::ChestKind;
use crate::types::Focus;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::reward_reset;
use crate::utils::context_focus;
use crate::utils::has_relic;
use crate::utils::queue_effect_untargeted;

#[derive(Debug, Clone, Copy)]
struct ChestParams {
    gold_chance: u8,
    gold_base: u16,
    th_common: u8,
    th_uncommon: u8,
}

pub fn process_effect_chest_open(state: &mut GameState) {
    assert!(
        context_focus(state) == Focus::Chest,
        "ChestOpen outside the Chest context"
    );
    let chest_kind = state.chest.chest_kind;
    state.chest.chest_opened = true;

    // Cursed Key: opening a chest adds a random Curse to the deck
    if has_relic(&state.id_relics, RelicName::CursedKey) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardAddRandom {
                color: CardColor::Curse,
                kind: None,
                pile: CardPile::Deck,
                count: 1,
                cost_zero: None,
                upgraded: false,
                rarity: Some(CardRarity::Curse),
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // N'loth's Hungry Face: the next chest opened is empty (one-shot)
    if let Some(id) = state.id_relics[RelicName::NlothsHungryFace as usize]
        && !state.entities[id].relic_used_up
    {
        state.entities[id].relic_used_up = true;
        return;
    }

    let chest_params = match chest_kind {
        ChestKind::Small => ChestParams {
            gold_chance: CHEST_SMALL_GOLD_CHANCE,
            gold_base: CHEST_SMALL_GOLD_BASE,
            th_common: CHEST_SMALL_TH_COMMON,
            th_uncommon: CHEST_SMALL_TH_UNCOMMON,
        },
        ChestKind::Medium => ChestParams {
            gold_chance: CHEST_MEDIUM_GOLD_CHANCE,
            gold_base: CHEST_MEDIUM_GOLD_BASE,
            th_common: CHEST_MEDIUM_TH_COMMON,
            th_uncommon: CHEST_MEDIUM_TH_UNCOMMON,
        },
        ChestKind::Large => ChestParams {
            gold_chance: CHEST_LARGE_GOLD_CHANCE,
            gold_base: CHEST_LARGE_GOLD_BASE,
            th_common: CHEST_LARGE_TH_COMMON,
            th_uncommon: CHEST_LARGE_TH_UNCOMMON,
        },
    };

    // One d100 serves the gold chance and the Relic tier, keeping them correlated
    let roll = state.rng.random_range(0..100) as u8;
    let gold =
        (roll < chest_params.gold_chance).then(|| roll_gold_amount(&mut state.rng, chest_params));

    // Chest gold skips Golden Idol (a combat-gold rule), so it rides the reset
    reward_reset(&mut state.reward);
    state.reward.gold = gold;
    state.reward.active = true;

    // Matryoshka: the next 2 chests hold an extra Relic (75% Common / 25% Uncommon)
    if let Some(id) = state.id_relics[RelicName::Matryoshka as usize]
        && state.entities[id].relic_counter > 0
    {
        // Grab mutable Matryoshka reference
        let relic = &mut state.entities[id];

        // Decrease counter, set `relic_used_up` if appropriate
        relic.relic_counter -= 1;
        relic.relic_used_up = relic.relic_counter == 0;

        queue_effect_untargeted(
            state,
            EffectKind::RewardRollRelic {
                pick: RelicPick::Thresholds {
                    th_common: MATRYOSHKA_TH_COMMON,
                    th_uncommon: MATRYOSHKA_TH_UNCOMMON,
                },
            },
        );
    }

    // The chest's own Relic, at the tier the shared roll already decided
    let tier = if roll < chest_params.th_common {
        RelicTier::Common
    } else if roll < chest_params.th_uncommon {
        RelicTier::Uncommon
    } else {
        RelicTier::Rare
    };
    queue_effect_untargeted(
        state,
        EffectKind::RewardRollRelic {
            pick: RelicPick::Tier(tier),
        },
    );
}

fn roll_gold_amount(rng: &mut impl Rng, chest_params: ChestParams) -> u16 {
    let base = chest_params.gold_base as f32;
    let factor = rng.random_range(0.9..=1.1);
    (base * factor).round() as u16
}
