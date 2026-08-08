use rand::Rng;

use crate::consts::NEOW_GOLD_LARGE;
use crate::consts::NEOW_GOLD_SMALL;
use crate::consts::NEOW_POTION_COUNT;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RewardSource;
use crate::effect::SelectionKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EFFECT_DECK_PURGE_PICK;
use crate::events::EFFECT_DECK_TRANSFORM_PICK;
use crate::events::EFFECT_DECK_UPGRADE_PICK;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventKind;
use crate::events::bake_options;
use crate::events::make_entity_event_option;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// Rolled option tags; identity lives in the baked effect lists, so these stay file-local
#[derive(Debug, Clone, Copy, PartialEq)]
enum NeowBonus {
    ThreeCards,
    OneRandomRareCard,
    RemoveCard,
    UpgradeCard,
    TransformCard,
    RandomColorless,
    ThreeSmallPotions,
    RandomCommonRelic,
    TenPercentHpBonus,
    ThreeEnemyKill,
    HundredGold,
    RandomColorlessRare,
    RemoveTwo,
    OneRareRelic,
    ThreeRareCards,
    TwoFiftyGold,
    TransformTwoCards,
    TwentyPercentHpBonus,
    BossRelic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NeowDrawback {
    TenPercentHpLoss,
    NoGold,
    Curse,
    PercentDamage,
}

// Category tables in source order
const CAT0: [NeowBonus; 6] = [
    NeowBonus::ThreeCards,
    NeowBonus::OneRandomRareCard,
    NeowBonus::RemoveCard,
    NeowBonus::UpgradeCard,
    NeowBonus::TransformCard,
    NeowBonus::RandomColorless,
];
const CAT1: [NeowBonus; 5] = [
    NeowBonus::ThreeSmallPotions,
    NeowBonus::RandomCommonRelic,
    NeowBonus::TenPercentHpBonus,
    NeowBonus::ThreeEnemyKill,
    NeowBonus::HundredGold,
];
const DRAWBACKS: [NeowDrawback; 4] = [
    NeowDrawback::TenPercentHpLoss,
    NeowDrawback::NoGold,
    NeowDrawback::Curse,
    NeowDrawback::PercentDamage,
];

// The 4-option full blessing; per-run amounts bake into the option lists at spawn
pub fn spawn_event_neow(state: &mut GameState) -> (EventKind, Vec<usize>) {
    let vitals = state.entities[state.id_character].vitals;
    let gold = state.entities[state.id_character].character_gold;

    // Source truncation: hp_bonus once at roll time (Twenty = 2x it), damage = (hp/10)*3
    let hp_bonus = vitals.health_max / 10;
    let damage = (vitals.health / 10) * 3;

    let bonus_cat0 = CAT0[state.rng.random_range(0..CAT0.len())];
    let bonus_cat1 = CAT1[state.rng.random_range(0..CAT1.len())];

    // Drawback rolls first; its thematic pairing drops out of the reward list (source order)
    let drawback = DRAWBACKS[state.rng.random_range(0..DRAWBACKS.len())];
    let mut rewards_cat2: Vec<NeowBonus> = Vec::with_capacity(7);
    rewards_cat2.push(NeowBonus::RandomColorlessRare);
    if drawback != NeowDrawback::Curse {
        rewards_cat2.push(NeowBonus::RemoveTwo);
    }
    rewards_cat2.push(NeowBonus::OneRareRelic);
    rewards_cat2.push(NeowBonus::ThreeRareCards);
    if drawback != NeowDrawback::NoGold {
        rewards_cat2.push(NeowBonus::TwoFiftyGold);
    }
    rewards_cat2.push(NeowBonus::TransformTwoCards);
    if drawback != NeowDrawback::TenPercentHpLoss {
        rewards_cat2.push(NeowBonus::TwentyPercentHpBonus);
    }
    let bonus_cat2 = rewards_cat2[state.rng.random_range(0..rewards_cat2.len())];

    let options = [
        option_for(bonus_cat0, None, hp_bonus, damage, gold),
        option_for(bonus_cat1, None, hp_bonus, damage, gold),
        option_for(bonus_cat2, Some(drawback), hp_bonus, damage, gold),
        option_for(NeowBonus::BossRelic, None, hp_bonus, damage, gold),
    ];
    let id_options = bake_options(state, &options);
    (EventKind::Neow, id_options)
}

// EVENT_CONSUME leads every list: relic adoption can stage a Reward frame (Tiny House),
// and the consume processor demands the Event frame on top; RewardRolls stay last
fn option_for(
    bonus: NeowBonus,
    drawback: Option<NeowDrawback>,
    hp_bonus: u16,
    damage: u16,
    gold: u16,
) -> Entity {
    let mut effects: Vec<Effect> = vec![EVENT_CONSUME_EFFECT];
    if let Some(drawback) = drawback {
        effects.push(drawback_effect(drawback, hp_bonus, damage, gold));
    }
    let label = match bonus {
        NeowBonus::ThreeCards => {
            effects.push(neow_cards(false, false));
            "[Cards] Choose 1 of 3 Cards."
        }
        NeowBonus::OneRandomRareCard => {
            effects.push(Effect {
                kind: EffectKind::CardAddRandom {
                    color: CardColor::Green,
                    kind: None,
                    pile: CardPile::Deck,
                    count: 1,
                    cost_zero: None,
                    upgraded: false,
                    rarity: Some(CardRarity::Rare),
                },
                id_source: None,
                target: Target::Direct(None),
            });
            "[Rare Card] Obtain a random Rare Card."
        }
        NeowBonus::RemoveCard => {
            effects.push(EFFECT_DECK_PURGE_PICK);
            "[Remove] Remove a Card from your deck."
        }
        NeowBonus::UpgradeCard => {
            effects.push(EFFECT_DECK_UPGRADE_PICK);
            "[Upgrade] Upgrade a Card."
        }
        NeowBonus::TransformCard => {
            effects.push(EFFECT_DECK_TRANSFORM_PICK);
            "[Transform] Transform a Card."
        }
        NeowBonus::RandomColorless => {
            effects.push(neow_cards(true, false));
            "[Colorless] Choose 1 of 3 Uncommon colorless Cards."
        }
        NeowBonus::ThreeSmallPotions => {
            effects.push(Effect {
                kind: EffectKind::RewardRoll {
                    source: RewardSource::Potions {
                        count: NEOW_POTION_COUNT,
                        uniform: true,
                    },
                },
                id_source: None,
                target: Target::Direct(None),
            });
            "[Potions] Obtain 3 random Potions."
        }
        NeowBonus::RandomCommonRelic => {
            effects.push(relic_grant(RelicTier::Common));
            "[Relic] Obtain a random Common Relic."
        }
        NeowBonus::TenPercentHpBonus => {
            effects.extend(max_hp_gain(hp_bonus));
            "[Max HP] Gain 10% Max HP."
        }
        NeowBonus::ThreeEnemyKill => {
            effects.push(Effect {
                kind: EffectKind::RelicGrantSpecific {
                    name: RelicName::NeowsLament,
                    fallback_circlet: false,
                },
                id_source: None,
                target: Target::Direct(None),
            });
            "[Neow's Lament] Enemies in your next 3 combats have 1 HP."
        }
        NeowBonus::HundredGold => {
            effects.push(gold_delta(DeltaSign::Gain, NEOW_GOLD_SMALL));
            "[Gold] Gain 100 gold."
        }
        NeowBonus::RandomColorlessRare => {
            effects.push(neow_cards(true, true));
            "[Colorless] Choose 1 of 3 Rare colorless Cards."
        }
        NeowBonus::RemoveTwo => {
            effects.push(deck_pick(
                EffectKind::CardPurge,
                CandidateFilter::Purgeable,
                2,
            ));
            "[Remove] Remove 2 Cards from your deck."
        }
        NeowBonus::OneRareRelic => {
            effects.push(relic_grant(RelicTier::Rare));
            "[Relic] Obtain a random Rare Relic."
        }
        NeowBonus::ThreeRareCards => {
            effects.push(neow_cards(false, true));
            "[Cards] Choose 1 of 3 Rare Cards."
        }
        NeowBonus::TwoFiftyGold => {
            effects.push(gold_delta(DeltaSign::Gain, NEOW_GOLD_LARGE));
            "[Gold] Gain 250 gold."
        }
        NeowBonus::TransformTwoCards => {
            effects.push(deck_pick(
                EffectKind::CardTransform { upgraded: false },
                CandidateFilter::Transformable,
                2,
            ));
            "[Transform] Transform 2 Cards."
        }
        NeowBonus::TwentyPercentHpBonus => {
            effects.extend(max_hp_gain(hp_bonus * 2));
            "[Max HP] Gain 20% Max HP."
        }
        NeowBonus::BossRelic => {
            effects.push(Effect {
                kind: EffectKind::RelicLose {
                    name: RelicName::SnakeRing,
                },
                id_source: None,
                target: Target::Direct(None),
            });
            effects.push(relic_grant(RelicTier::Boss));
            "[Boss Swap] Lose Snake Ring. Obtain a random Boss Relic."
        }
    };
    make_entity_event_option(label, &effects)
}

// Applied before the reward, mirroring the source's activate() order
fn drawback_effect(drawback: NeowDrawback, hp_bonus: u16, damage: u16, gold: u16) -> Effect {
    match drawback {
        NeowDrawback::TenPercentHpLoss => Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(hp_bonus),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        NeowDrawback::NoGold => gold_delta(DeltaSign::Loss, gold),
        NeowDrawback::Curse => Effect {
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
        },
        NeowDrawback::PercentDamage => Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(damage),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    }
}

fn neow_cards(colorless: bool, rare_only: bool) -> Effect {
    Effect {
        kind: EffectKind::RewardRoll {
            source: RewardSource::NeowCards {
                colorless,
                rare_only,
            },
        },
        id_source: None,
        target: Target::Direct(None),
    }
}

fn relic_grant(tier: RelicTier) -> Effect {
    Effect {
        kind: EffectKind::RelicGrantRandom { tier: Some(tier) },
        id_source: None,
        target: Target::Direct(None),
    }
}

fn gold_delta(sign: DeltaSign, amount: u16) -> Effect {
    Effect {
        kind: EffectKind::GoldDelta {
            sign,
            amount: Amount::Absolute(amount),
        },
        id_source: None,
        target: Target::Direct(None),
    }
}

// Max first, then the matching heal — the source's increaseMaxHp does both
fn max_hp_gain(amount: u16) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(amount),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(amount),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ]
}

fn deck_pick(kind: EffectKind, filter: CandidateFilter, count: u16) -> Effect {
    Effect {
        kind,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter,
            selection_kind: SelectionKind::Input { count },
        },
    }
}
