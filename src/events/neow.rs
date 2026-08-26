use rand::Rng;

use crate::character::silent_health;
use crate::consts::ASCENSION_HP_MAX_CUT_LEVEL;
use crate::consts::ASCENSION_HP_START_CUT_LEVEL;
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
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

#[derive(Debug, Clone, Copy, PartialEq)]
enum NeowBonus {
    Cards { colorless: bool, rare_only: bool },
    OneRandomRareCard,
    Remove { count: u16 },
    UpgradeCard,
    Transform { count: u16 },
    ThreeSmallPotions,
    Relic { tier: RelicTier },
    HealthBonus { mult: u16 },
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
const BONUS_CAT_0: [NeowBonus; 6] = [
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
const BONUS_CAT_1: [NeowBonus; 5] = [
    NeowBonus::ThreeSmallPotions,
    NeowBonus::Relic {
        tier: RelicTier::Common,
    },
    NeowBonus::HealthBonus { mult: 1 },
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

const fn effect_relic_grant_random(tier: RelicTier) -> Effect {
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

const fn effect_bonus(bonus: NeowBonus, health_bonus: u16) -> Effect {
    match bonus {
        NeowBonus::Cards {
            colorless,
            rare_only,
        } => Effect {
            kind: EffectKind::RewardRollNeowCards {
                colorless,
                rare_only,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        NeowBonus::OneRandomRareCard => Effect {
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
        },
        NeowBonus::Remove { count } => Effect {
            kind: EffectKind::CardPurge,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck,
                filter: CandidateFilter::Purgeable,
                selection_kind: SelectionKind::Input { count },
            },
        },
        NeowBonus::UpgradeCard => EFFECT_DECK_UPGRADE_PICK_1,
        NeowBonus::Transform { count } => Effect {
            kind: EffectKind::CardTransform { upgraded: false },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck,
                filter: CandidateFilter::Transformable,
                selection_kind: SelectionKind::Input { count },
            },
        },
        NeowBonus::ThreeSmallPotions => Effect {
            kind: EffectKind::RewardRollPotions {
                count: NEOW_POTION_COUNT,
                uniform: true,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        NeowBonus::Relic { tier } => effect_relic_grant_random(tier),
        NeowBonus::HealthBonus { mult } => Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(health_bonus * mult),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        NeowBonus::ThreeEnemyKill => Effect {
            kind: EffectKind::RelicGrantSpecific {
                name: RelicName::NeowsLament,
                fallback_circlet: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        NeowBonus::GoldGain { amount } => effect_gold_delta(DeltaSign::Gain, amount),
    }
}

const fn effect_drawback(drawback: NeowDrawback, health_bonus: u16, damage: u16) -> Effect {
    match drawback {
        NeowDrawback::TenPercentHpLoss => Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(health_bonus),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        NeowDrawback::GoldLoss => effect_gold_delta(DeltaSign::Loss, STARTING_GOLD),
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

// Category-2 bonuses per drawback
const BONUS_CAT_2_HP_LOSS: [NeowBonus; 6] = [
    NeowBonus::Cards {
        colorless: true,
        rare_only: true,
    },
    NeowBonus::Remove { count: 2 },
    NeowBonus::Relic {
        tier: RelicTier::Rare,
    },
    NeowBonus::Cards {
        colorless: false,
        rare_only: true,
    },
    NeowBonus::GoldGain {
        amount: NEOW_GOLD_LARGE,
    },
    NeowBonus::Transform { count: 2 },
];
const BONUS_CAT_2_GOLD_LOSS: [NeowBonus; 6] = [
    NeowBonus::Cards {
        colorless: true,
        rare_only: true,
    },
    NeowBonus::Remove { count: 2 },
    NeowBonus::Relic {
        tier: RelicTier::Rare,
    },
    NeowBonus::Cards {
        colorless: false,
        rare_only: true,
    },
    NeowBonus::Transform { count: 2 },
    NeowBonus::HealthBonus { mult: 2 },
];
const BONUS_CAT_2_CURSE: [NeowBonus; 6] = [
    NeowBonus::Cards {
        colorless: true,
        rare_only: true,
    },
    NeowBonus::Relic {
        tier: RelicTier::Rare,
    },
    NeowBonus::Cards {
        colorless: false,
        rare_only: true,
    },
    NeowBonus::GoldGain {
        amount: NEOW_GOLD_LARGE,
    },
    NeowBonus::Transform { count: 2 },
    NeowBonus::HealthBonus { mult: 2 },
];
const BONUS_CAT_2_DAMAGE: [NeowBonus; 7] = [
    NeowBonus::Cards {
        colorless: true,
        rare_only: true,
    },
    NeowBonus::Remove { count: 2 },
    NeowBonus::Relic {
        tier: RelicTier::Rare,
    },
    NeowBonus::Cards {
        colorless: false,
        rare_only: true,
    },
    NeowBonus::GoldGain {
        amount: NEOW_GOLD_LARGE,
    },
    NeowBonus::Transform { count: 2 },
    NeowBonus::HealthBonus { mult: 2 },
];

const fn bonus_for_drawback_cat2(drawback: NeowDrawback) -> &'static [NeowBonus] {
    match drawback {
        NeowDrawback::TenPercentHpLoss => &BONUS_CAT_2_HP_LOSS,
        NeowDrawback::GoldLoss => &BONUS_CAT_2_GOLD_LOSS,
        NeowDrawback::Curse => &BONUS_CAT_2_CURSE,
        NeowDrawback::PercentDamage => &BONUS_CAT_2_DAMAGE,
    }
}

// Catalog layout: Cat-0 | Cat-1 | Cat-2 grouped by drawback | Boss swap
const IDX_CAT_1: usize = BONUS_CAT_0.len();
const IDX_CAT_2: usize = IDX_CAT_1 + BONUS_CAT_1.len();
const IDX_BOSS: usize = IDX_CAT_2
    + BONUS_CAT_2_HP_LOSS.len()
    + BONUS_CAT_2_GOLD_LOSS.len()
    + BONUS_CAT_2_CURSE.len()
    + BONUS_CAT_2_DAMAGE.len();
const CATALOG_LEN: usize = IDX_BOSS + 1;

// Where each drawback's category-2 block starts within the drawback `EventOptionTemplate`s (EOTs)
const BONUS_CAT_2_OFFSETS: [usize; DRAWBACKS.len()] = {
    let mut offsets = [0usize; DRAWBACKS.len()];
    let mut idx = 1;
    while idx < DRAWBACKS.len() {
        offsets[idx] = offsets[idx - 1] + bonus_for_drawback_cat2(DRAWBACKS[idx - 1]).len();
        idx += 1;
    }
    offsets
};
const _: () = assert!(
    IDX_CAT_2
        + BONUS_CAT_2_OFFSETS[DRAWBACKS.len() - 1]
        + bonus_for_drawback_cat2(DRAWBACKS[DRAWBACKS.len() - 1]).len()
        == IDX_BOSS
);

const fn eots_for_asc(ascension: u8) -> [EventOptionTemplate; CATALOG_LEN] {
    // Run-start vitals drive every amount Neow bakes
    let (health, health_max) = silent_health(ascension);
    let damage = (health / 10) * 3;
    let health_bonus = health_max / 10;

    // Initialize catalog
    let mut eots = [make_event_option_template(&[
        EFFECT_EVENT_CONSUME,
        Effect {
            kind: EffectKind::RelicLose,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::EventRollRelic,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::Single,
            },
        },
        effect_relic_grant_random(RelicTier::Boss),
    ]); CATALOG_LEN];

    // The two drawback-free categories sit back to back, so one cursor covers both
    let mut idx = 0;
    while idx < IDX_CAT_2 {
        let bonus = if idx < IDX_CAT_1 {
            BONUS_CAT_0[idx]
        } else {
            BONUS_CAT_1[idx - IDX_CAT_1]
        };
        let eot = [EFFECT_EVENT_CONSUME, effect_bonus(bonus, health_bonus)];
        eots[idx] = make_event_option_template(&eot);
        idx += 1;
    }

    // `idx` is already at IDX_CAT_2; the drawback EOTs continue the same cursor
    let mut ddx = 0;
    while ddx < DRAWBACKS.len() {
        let drawback = DRAWBACKS[ddx];

        // Fill all possible `NeowBonus`es for this `NewoDrawback`
        let bonuses = bonus_for_drawback_cat2(drawback);
        let mut bdx = 0;
        while bdx < bonuses.len() {
            let eot = [
                EFFECT_EVENT_CONSUME,
                effect_drawback(drawback, health_bonus, damage),
                effect_bonus(bonuses[bdx], health_bonus),
            ];
            eots[idx] = make_event_option_template(&eot);

            // Increment indexes
            idx += 1;
            bdx += 1;
        }
        ddx += 1;
    }
    eots
}

// One table per distinct run-start vitals tier
static EOTS_A0: [EventOptionTemplate; CATALOG_LEN] = eots_for_asc(0);
static EOTS_A6: [EventOptionTemplate; CATALOG_LEN] = eots_for_asc(ASCENSION_HP_START_CUT_LEVEL);
static EOTS_A14: [EventOptionTemplate; CATALOG_LEN] = eots_for_asc(ASCENSION_HP_MAX_CUT_LEVEL);

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < ASCENSION_HP_START_CUT_LEVEL {
        &EOTS_A0
    } else if ascension < ASCENSION_HP_MAX_CUT_LEVEL {
        &EOTS_A6
    } else {
        &EOTS_A14
    }
}

// Rolls index into the catalog, so a spawned option is a catalog EOT by construction
pub fn spawn(state: &mut GameState) -> Vec<usize> {
    let eots = catalog(state.ascension);
    let idx_cat_0 = state.rng.random_range(0..BONUS_CAT_0.len());
    let idx_cat_1 = state.rng.random_range(0..BONUS_CAT_1.len());

    // Drawback rolls first; its thematically-paired bonus drops out (source order)
    let idx_drawback = state.rng.random_range(0..DRAWBACKS.len());
    let idx_cat_2 = state
        .rng
        .random_range(0..bonus_for_drawback_cat2(DRAWBACKS[idx_drawback]).len());

    // The boss swap consumes the staked starter Relic
    let id_ring_of_the_snake = state.id_relics[RelicName::RingOfTheSnake as usize]
        .expect("Neow spawns at run start with the starter relic");
    state.event.id_roll_relic.push(id_ring_of_the_snake);

    let options = [
        eots[idx_cat_0],
        eots[IDX_CAT_1 + idx_cat_1],
        eots[IDX_CAT_2 + BONUS_CAT_2_OFFSETS[idx_drawback] + idx_cat_2],
        eots[IDX_BOSS],
    ];
    bake_options(state, &options)
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
