use rand::Rng;

use crate::character::silent_health;
use crate::consts::NEOW_GOLD_LARGE;
use crate::consts::NEOW_GOLD_SMALL;
use crate::consts::NEOW_POTION_COUNT;
use crate::consts::STARTING_GOLD;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_DECK_UPGRADE_PICK_1;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::bake_options;
use crate::events::make_event_option_template;
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
    Cards { colorless: bool, rare_only: bool },
    OneRandomRareCard,
    Remove { count: u16 },
    UpgradeCard,
    Transform { count: u16 },
    ThreeSmallPotions,
    Relic { tier: RelicTier },
    HpBonus { mult: u16 },
    ThreeEnemyKill,
    GoldGain { amount: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NeowDrawback {
    TenPercentHpLoss,
    GoldLoss,
    Curse,
    PercentDamage,
}

// Category tables in source order
const BONUSES_CAT_0: [NeowBonus; 6] = [
    NeowBonus::Cards {
        colorless: false,
        rare_only: false,
    },
    NeowBonus::OneRandomRareCard,
    NeowBonus::Remove { count: 1 },
    NeowBonus::UpgradeCard,
    NeowBonus::Transform { count: 1 },
    NeowBonus::Cards {
        colorless: true,
        rare_only: false,
    },
];
const BONUSES_CAT_1: [NeowBonus; 5] = [
    NeowBonus::ThreeSmallPotions,
    NeowBonus::Relic {
        tier: RelicTier::Common,
    },
    NeowBonus::HpBonus { mult: 1 },
    NeowBonus::ThreeEnemyKill,
    NeowBonus::GoldGain {
        amount: NEOW_GOLD_SMALL,
    },
];
const DRAWBACKS: [NeowDrawback; 4] = [
    NeowDrawback::TenPercentHpLoss,
    NeowDrawback::GoldLoss,
    NeowDrawback::Curse,
    NeowDrawback::PercentDamage,
];

const fn effect_neow_cards(colorless: bool, rare_only: bool) -> Effect {
    Effect {
        kind: EffectKind::RewardRollNeowCards {
            colorless,
            rare_only,
        },
        id_source: None,
        target: Target::Direct(None),
    }
}

const fn effect_relic_grant(tier: RelicTier) -> Effect {
    Effect {
        kind: EffectKind::RelicGrantRandom { tier: Some(tier) },
        id_source: None,
        target: Target::Direct(None),
    }
}

const fn effect_gold_delta(sign: DeltaSign, amount: u16) -> Effect {
    Effect {
        kind: EffectKind::GoldDelta {
            sign,
            amount: Amount::Absolute(amount),
        },
        id_source: None,
        target: Target::Direct(None),
    }
}

const fn effect_max_hp_gain(amount: u16) -> Effect {
    Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(amount),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }
}

const fn deck_pick(kind: EffectKind, filter: CandidateFilter, count: u16) -> Effect {
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

const EFFECT_RARE_GREEN_CARD: Effect = Effect {
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
};

const EFFECT_POTIONS: Effect = Effect {
    kind: EffectKind::RewardRollPotions {
        count: NEOW_POTION_COUNT,
        uniform: true,
    },
    id_source: None,
    target: Target::Direct(None),
};

const EFFECT_LAMENT: Effect = Effect {
    kind: EffectKind::RelicGrantSpecific {
        name: RelicName::NeowsLament,
        fallback_circlet: false,
    },
    id_source: None,
    target: Target::Direct(None),
};

const EFFECT_CURSE: Effect = Effect {
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
};

// Category-2 bonuses for a rolled drawback: source order, thematic pairing dropped
fn bonuses_cat_2(drawback: NeowDrawback) -> Vec<NeowBonus> {
    let mut bonuses: Vec<NeowBonus> = Vec::with_capacity(7);
    bonuses.push(NeowBonus::Cards {
        colorless: true,
        rare_only: true,
    });
    if drawback != NeowDrawback::Curse {
        bonuses.push(NeowBonus::Remove { count: 2 });
    }
    bonuses.push(NeowBonus::Relic {
        tier: RelicTier::Rare,
    });
    bonuses.push(NeowBonus::Cards {
        colorless: false,
        rare_only: true,
    });
    if drawback != NeowDrawback::GoldLoss {
        bonuses.push(NeowBonus::GoldGain {
            amount: NEOW_GOLD_LARGE,
        });
    }
    bonuses.push(NeowBonus::Transform { count: 2 });
    if drawback != NeowDrawback::TenPercentHpLoss {
        bonuses.push(NeowBonus::HpBonus { mult: 2 });
    }
    bonuses
}

const LABEL_BOSS_SWAP: &str = "[Boss Swap] Lose Snake Ring. Obtain a random Boss Relic.";

const OPTION_BOSS_SWAP: [Effect; 3] = [
    EVENT_CONSUME_EFFECT,
    Effect {
        kind: EffectKind::RelicLose,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::EventRollRelic,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    },
    effect_relic_grant(RelicTier::Boss),
];

// Catalog product: cat-0, cat-1, drawback x its bonus list (DRAWBACKS order), boss.
// Run-start vitals derive from ascension, so rows equal the live spawn's values
pub fn catalog(ascension: u8) -> Vec<Vec<Effect>> {
    let (health, health_max) = silent_health(ascension);
    let hp_bonus = health_max / 10;
    let damage = (health / 10) * 3;
    let gold = STARTING_GOLD;

    let mut rows = Vec::with_capacity(37);
    for &bonus in BONUSES_CAT_0.iter() {
        rows.push(option_for(bonus, None, hp_bonus, damage, gold).1);
    }
    for &bonus in BONUSES_CAT_1.iter() {
        rows.push(option_for(bonus, None, hp_bonus, damage, gold).1);
    }
    for &drawback in DRAWBACKS.iter() {
        for bonus in bonuses_cat_2(drawback) {
            rows.push(option_for(bonus, Some(drawback), hp_bonus, damage, gold).1);
        }
    }
    rows.push(OPTION_BOSS_SWAP.to_vec());
    rows
}

pub fn spawn_event_neow(state: &mut GameState) -> Vec<usize> {
    let bonus_cat_0 = BONUSES_CAT_0[state.rng.random_range(0..BONUSES_CAT_0.len())];
    let bonus_cat_1 = BONUSES_CAT_1[state.rng.random_range(0..BONUSES_CAT_1.len())];

    // Drawback rolls first; its thematically-paired bonus drops out (source order)
    let drawback = DRAWBACKS[state.rng.random_range(0..DRAWBACKS.len())];
    let bonuses = bonuses_cat_2(drawback);
    let bonus_cat_2 = bonuses[state.rng.random_range(0..bonuses.len())];

    // Run-start vitals: hp_bonus = max/10, damage = (hp/10)*3 (reference NeowReward)
    let character = &state.entities[state.id_character];
    let gold = character.character_gold;
    let hp_bonus = character.vitals.health_max / 10;
    let damage = (character.vitals.health / 10) * 3;

    let built = [
        option_for(bonus_cat_0, None, hp_bonus, damage, gold),
        option_for(bonus_cat_1, None, hp_bonus, damage, gold),
        option_for(bonus_cat_2, Some(drawback), hp_bonus, damage, gold),
    ];

    // The boss swap consumes the staked starter relic
    let id_snake_ring = state.id_relics[RelicName::SnakeRing as usize]
        .expect("Neow spawns at run start with the starter relic");
    state.event.id_roll_relic.push(id_snake_ring);

    let options = [
        make_event_option_template(built[0].0, &built[0].1),
        make_event_option_template(built[1].0, &built[1].1),
        make_event_option_template(built[2].0, &built[2].1),
        make_event_option_template(LABEL_BOSS_SWAP, &OPTION_BOSS_SWAP),
    ];
    bake_options(state, &options)
}

// EVENT_CONSUME leads every list (relic adoption can replace the context with a
// staged reward, e.g. Calling Bell); the reward effects stay last
fn option_for(
    bonus: NeowBonus,
    drawback: Option<NeowDrawback>,
    hp_bonus: u16,
    damage: u16,
    gold: u16,
) -> (&'static str, Vec<Effect>) {
    let mut effects: Vec<Effect> = vec![EVENT_CONSUME_EFFECT];

    // Push drawback
    if let Some(drawback) = drawback {
        effects.push(effect_drawback(drawback, hp_bonus, damage, gold));
    }

    // Push bonus and pick its label
    let label = match bonus {
        NeowBonus::Cards {
            colorless,
            rare_only,
        } => {
            effects.push(effect_neow_cards(colorless, rare_only));
            match (colorless, rare_only) {
                (false, false) => "[Cards] Choose 1 of 3 Cards.",
                (false, true) => "[Cards] Choose 1 of 3 Rare Cards.",
                (true, false) => "[Colorless] Choose 1 of 3 Uncommon colorless Cards.",
                (true, true) => "[Colorless] Choose 1 of 3 Rare colorless Cards.",
            }
        }
        NeowBonus::OneRandomRareCard => {
            effects.push(EFFECT_RARE_GREEN_CARD);
            "[Rare Card] Obtain a random Rare Card."
        }
        NeowBonus::Remove { count } => {
            effects.push(deck_pick(
                EffectKind::CardPurge,
                CandidateFilter::Purgeable,
                count,
            ));
            if count == 1 {
                "[Remove] Remove a Card from your deck."
            } else {
                "[Remove] Remove 2 Cards from your deck."
            }
        }
        NeowBonus::UpgradeCard => {
            effects.push(EFFECT_DECK_UPGRADE_PICK_1);
            "[Upgrade] Upgrade a Card."
        }
        NeowBonus::Transform { count } => {
            effects.push(deck_pick(
                EffectKind::CardTransform { upgraded: false },
                CandidateFilter::Transformable,
                count,
            ));
            if count == 1 {
                "[Transform] Transform a Card."
            } else {
                "[Transform] Transform 2 Cards."
            }
        }
        NeowBonus::ThreeSmallPotions => {
            effects.push(EFFECT_POTIONS);
            "[Potions] Obtain 3 random Potions."
        }
        NeowBonus::Relic { tier } => {
            effects.push(effect_relic_grant(tier));
            if tier == RelicTier::Common {
                "[Relic] Obtain a random Common Relic."
            } else {
                "[Relic] Obtain a random Rare Relic."
            }
        }
        NeowBonus::HpBonus { mult } => {
            effects.push(effect_max_hp_gain(hp_bonus * mult));
            if mult == 1 {
                "[Max HP] Gain 10% Max HP."
            } else {
                "[Max HP] Gain 20% Max HP."
            }
        }
        NeowBonus::ThreeEnemyKill => {
            effects.push(EFFECT_LAMENT);
            "[Neow's Lament] Enemies in your next 3 combats have 1 HP."
        }
        NeowBonus::GoldGain { amount } => {
            effects.push(effect_gold_delta(DeltaSign::Gain, amount));
            if amount == NEOW_GOLD_SMALL {
                "[Gold] Gain 100 gold."
            } else {
                "[Gold] Gain 250 gold."
            }
        }
    };
    (label, effects)
}

// Applied before the reward, mirroring the source's activate() order
fn effect_drawback(drawback: NeowDrawback, hp_bonus: u16, damage: u16, gold: u16) -> Effect {
    match drawback {
        NeowDrawback::TenPercentHpLoss => Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(hp_bonus),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        NeowDrawback::GoldLoss => effect_gold_delta(DeltaSign::Loss, gold),
        NeowDrawback::Curse => EFFECT_CURSE,
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
