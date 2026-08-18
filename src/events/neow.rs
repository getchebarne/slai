use rand::Rng;

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

// Run-start vitals per ascension tier (silent_health): hp_bonus = max/10, damage = (hp/10)*3
const HP_BONUS_BASE: u16 = 7;
const HP_BONUS_A14: u16 = 6;
const DAMAGE_A0: u16 = 21;
const DAMAGE_A6: u16 = 18;
const DAMAGE_A14: u16 = 15;

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

// Drawbacks, applied before the reward, mirroring the source's activate() order
const fn dw_hp_loss(hp_bonus: u16) -> Effect {
    Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(hp_bonus),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }
}

const fn dw_damage(damage: u16) -> Effect {
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(damage),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }
}

// The gold-loss drawback takes everything; Neow is run start, so all = STARTING_GOLD
const DW_GOLD_LOSS: Effect = effect_gold_delta(DeltaSign::Loss, STARTING_GOLD);

const DW_CURSE: Effect = Effect {
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

// Category 0 (no drawback, tier-free)
const T_CARDS: EventOptionTemplate = make_event_option_template(
    "[Cards] Choose 1 of 3 Cards.",
    &[EVENT_CONSUME_EFFECT, effect_neow_cards(false, false)],
);
const T_RARE_CARD: EventOptionTemplate = make_event_option_template(
    "[Rare Card] Obtain a random Rare Card.",
    &[EVENT_CONSUME_EFFECT, EFFECT_RARE_GREEN_CARD],
);
const T_REMOVE_1: EventOptionTemplate = make_event_option_template(
    "[Remove] Remove a Card from your deck.",
    &[
        EVENT_CONSUME_EFFECT,
        deck_pick(EffectKind::CardPurge, CandidateFilter::Purgeable, 1),
    ],
);
const T_UPGRADE: EventOptionTemplate = make_event_option_template(
    "[Upgrade] Upgrade a Card.",
    &[EVENT_CONSUME_EFFECT, EFFECT_DECK_UPGRADE_PICK_1],
);
const T_TRANSFORM_1: EventOptionTemplate = make_event_option_template(
    "[Transform] Transform a Card.",
    &[
        EVENT_CONSUME_EFFECT,
        deck_pick(
            EffectKind::CardTransform { upgraded: false },
            CandidateFilter::Transformable,
            1,
        ),
    ],
);
const T_COLORLESS: EventOptionTemplate = make_event_option_template(
    "[Colorless] Choose 1 of 3 Uncommon colorless Cards.",
    &[EVENT_CONSUME_EFFECT, effect_neow_cards(true, false)],
);

static CAT_0: [EventOptionTemplate; 6] = [
    T_CARDS,
    T_RARE_CARD,
    T_REMOVE_1,
    T_UPGRADE,
    T_TRANSFORM_1,
    T_COLORLESS,
];

// Category 1 (no drawback; the max-HP entry carries the tier's hp_bonus)
const T_POTIONS: EventOptionTemplate = make_event_option_template(
    "[Potions] Obtain 3 random Potions.",
    &[EVENT_CONSUME_EFFECT, EFFECT_POTIONS],
);
const T_RELIC_COMMON: EventOptionTemplate = make_event_option_template(
    "[Relic] Obtain a random Common Relic.",
    &[EVENT_CONSUME_EFFECT, effect_relic_grant(RelicTier::Common)],
);
const T_HP_10_BASE: EventOptionTemplate = make_event_option_template(
    "[Max HP] Gain 10% Max HP.",
    &[EVENT_CONSUME_EFFECT, effect_max_hp_gain(HP_BONUS_BASE)],
);
const T_HP_10_A14: EventOptionTemplate = make_event_option_template(
    "[Max HP] Gain 10% Max HP.",
    &[EVENT_CONSUME_EFFECT, effect_max_hp_gain(HP_BONUS_A14)],
);
const T_LAMENT: EventOptionTemplate = make_event_option_template(
    "[Neow's Lament] Enemies in your next 3 combats have 1 HP.",
    &[EVENT_CONSUME_EFFECT, EFFECT_LAMENT],
);
const T_GOLD_SMALL: EventOptionTemplate = make_event_option_template(
    "[Gold] Gain 100 gold.",
    &[
        EVENT_CONSUME_EFFECT,
        effect_gold_delta(DeltaSign::Gain, NEOW_GOLD_SMALL),
    ],
);

static CAT_1_BASE: [EventOptionTemplate; 5] = [
    T_POTIONS,
    T_RELIC_COMMON,
    T_HP_10_BASE,
    T_LAMENT,
    T_GOLD_SMALL,
];
static CAT_1_A14: [EventOptionTemplate; 5] = [
    T_POTIONS,
    T_RELIC_COMMON,
    T_HP_10_A14,
    T_LAMENT,
    T_GOLD_SMALL,
];

// Category 2 bonuses (paired with a drawback effect per table below)
const B_COLORLESS_RARE: Effect = effect_neow_cards(true, true);
const B_REMOVE_2: Effect = deck_pick(EffectKind::CardPurge, CandidateFilter::Purgeable, 2);
const B_RELIC_RARE: Effect = effect_relic_grant(RelicTier::Rare);
const B_RARE_CARDS: Effect = effect_neow_cards(false, true);
const B_GOLD_LARGE: Effect = effect_gold_delta(DeltaSign::Gain, NEOW_GOLD_LARGE);
const B_TRANSFORM_2: Effect = deck_pick(
    EffectKind::CardTransform { upgraded: false },
    CandidateFilter::Transformable,
    2,
);
const fn b_hp_20(hp_bonus: u16) -> Effect {
    effect_max_hp_gain(hp_bonus * 2)
}

const L_COLORLESS_RARE: &str = "[Colorless] Choose 1 of 3 Rare colorless Cards.";
const L_REMOVE_2: &str = "[Remove] Remove 2 Cards from your deck.";
const L_RELIC_RARE: &str = "[Relic] Obtain a random Rare Relic.";
const L_RARE_CARDS: &str = "[Cards] Choose 1 of 3 Rare Cards.";
const L_GOLD_LARGE: &str = "[Gold] Gain 250 gold.";
const L_TRANSFORM_2: &str = "[Transform] Transform 2 Cards.";
const L_HP_20: &str = "[Max HP] Gain 20% Max HP.";

// Tables keep the spawn's source order; the thematically-paired bonus is dropped
static CAT_2_HP_LOSS_BASE: [EventOptionTemplate; 6] = [
    make_event_option_template(
        L_COLORLESS_RARE,
        &[
            EVENT_CONSUME_EFFECT,
            dw_hp_loss(HP_BONUS_BASE),
            B_COLORLESS_RARE,
        ],
    ),
    make_event_option_template(
        L_REMOVE_2,
        &[EVENT_CONSUME_EFFECT, dw_hp_loss(HP_BONUS_BASE), B_REMOVE_2],
    ),
    make_event_option_template(
        L_RELIC_RARE,
        &[
            EVENT_CONSUME_EFFECT,
            dw_hp_loss(HP_BONUS_BASE),
            B_RELIC_RARE,
        ],
    ),
    make_event_option_template(
        L_RARE_CARDS,
        &[
            EVENT_CONSUME_EFFECT,
            dw_hp_loss(HP_BONUS_BASE),
            B_RARE_CARDS,
        ],
    ),
    make_event_option_template(
        L_GOLD_LARGE,
        &[
            EVENT_CONSUME_EFFECT,
            dw_hp_loss(HP_BONUS_BASE),
            B_GOLD_LARGE,
        ],
    ),
    make_event_option_template(
        L_TRANSFORM_2,
        &[
            EVENT_CONSUME_EFFECT,
            dw_hp_loss(HP_BONUS_BASE),
            B_TRANSFORM_2,
        ],
    ),
];
static CAT_2_HP_LOSS_A14: [EventOptionTemplate; 6] = [
    make_event_option_template(
        L_COLORLESS_RARE,
        &[
            EVENT_CONSUME_EFFECT,
            dw_hp_loss(HP_BONUS_A14),
            B_COLORLESS_RARE,
        ],
    ),
    make_event_option_template(
        L_REMOVE_2,
        &[EVENT_CONSUME_EFFECT, dw_hp_loss(HP_BONUS_A14), B_REMOVE_2],
    ),
    make_event_option_template(
        L_RELIC_RARE,
        &[EVENT_CONSUME_EFFECT, dw_hp_loss(HP_BONUS_A14), B_RELIC_RARE],
    ),
    make_event_option_template(
        L_RARE_CARDS,
        &[EVENT_CONSUME_EFFECT, dw_hp_loss(HP_BONUS_A14), B_RARE_CARDS],
    ),
    make_event_option_template(
        L_GOLD_LARGE,
        &[EVENT_CONSUME_EFFECT, dw_hp_loss(HP_BONUS_A14), B_GOLD_LARGE],
    ),
    make_event_option_template(
        L_TRANSFORM_2,
        &[
            EVENT_CONSUME_EFFECT,
            dw_hp_loss(HP_BONUS_A14),
            B_TRANSFORM_2,
        ],
    ),
];
static CAT_2_GOLD_LOSS_BASE: [EventOptionTemplate; 6] = [
    make_event_option_template(
        L_COLORLESS_RARE,
        &[EVENT_CONSUME_EFFECT, DW_GOLD_LOSS, B_COLORLESS_RARE],
    ),
    make_event_option_template(
        L_REMOVE_2,
        &[EVENT_CONSUME_EFFECT, DW_GOLD_LOSS, B_REMOVE_2],
    ),
    make_event_option_template(
        L_RELIC_RARE,
        &[EVENT_CONSUME_EFFECT, DW_GOLD_LOSS, B_RELIC_RARE],
    ),
    make_event_option_template(
        L_RARE_CARDS,
        &[EVENT_CONSUME_EFFECT, DW_GOLD_LOSS, B_RARE_CARDS],
    ),
    make_event_option_template(
        L_TRANSFORM_2,
        &[EVENT_CONSUME_EFFECT, DW_GOLD_LOSS, B_TRANSFORM_2],
    ),
    make_event_option_template(
        L_HP_20,
        &[EVENT_CONSUME_EFFECT, DW_GOLD_LOSS, b_hp_20(HP_BONUS_BASE)],
    ),
];
static CAT_2_GOLD_LOSS_A14: [EventOptionTemplate; 6] = [
    make_event_option_template(
        L_COLORLESS_RARE,
        &[EVENT_CONSUME_EFFECT, DW_GOLD_LOSS, B_COLORLESS_RARE],
    ),
    make_event_option_template(
        L_REMOVE_2,
        &[EVENT_CONSUME_EFFECT, DW_GOLD_LOSS, B_REMOVE_2],
    ),
    make_event_option_template(
        L_RELIC_RARE,
        &[EVENT_CONSUME_EFFECT, DW_GOLD_LOSS, B_RELIC_RARE],
    ),
    make_event_option_template(
        L_RARE_CARDS,
        &[EVENT_CONSUME_EFFECT, DW_GOLD_LOSS, B_RARE_CARDS],
    ),
    make_event_option_template(
        L_TRANSFORM_2,
        &[EVENT_CONSUME_EFFECT, DW_GOLD_LOSS, B_TRANSFORM_2],
    ),
    make_event_option_template(
        L_HP_20,
        &[EVENT_CONSUME_EFFECT, DW_GOLD_LOSS, b_hp_20(HP_BONUS_A14)],
    ),
];
static CAT_2_CURSE_BASE: [EventOptionTemplate; 6] = [
    make_event_option_template(
        L_COLORLESS_RARE,
        &[EVENT_CONSUME_EFFECT, DW_CURSE, B_COLORLESS_RARE],
    ),
    make_event_option_template(
        L_RELIC_RARE,
        &[EVENT_CONSUME_EFFECT, DW_CURSE, B_RELIC_RARE],
    ),
    make_event_option_template(
        L_RARE_CARDS,
        &[EVENT_CONSUME_EFFECT, DW_CURSE, B_RARE_CARDS],
    ),
    make_event_option_template(
        L_GOLD_LARGE,
        &[EVENT_CONSUME_EFFECT, DW_CURSE, B_GOLD_LARGE],
    ),
    make_event_option_template(
        L_TRANSFORM_2,
        &[EVENT_CONSUME_EFFECT, DW_CURSE, B_TRANSFORM_2],
    ),
    make_event_option_template(
        L_HP_20,
        &[EVENT_CONSUME_EFFECT, DW_CURSE, b_hp_20(HP_BONUS_BASE)],
    ),
];
static CAT_2_CURSE_A14: [EventOptionTemplate; 6] = [
    make_event_option_template(
        L_COLORLESS_RARE,
        &[EVENT_CONSUME_EFFECT, DW_CURSE, B_COLORLESS_RARE],
    ),
    make_event_option_template(
        L_RELIC_RARE,
        &[EVENT_CONSUME_EFFECT, DW_CURSE, B_RELIC_RARE],
    ),
    make_event_option_template(
        L_RARE_CARDS,
        &[EVENT_CONSUME_EFFECT, DW_CURSE, B_RARE_CARDS],
    ),
    make_event_option_template(
        L_GOLD_LARGE,
        &[EVENT_CONSUME_EFFECT, DW_CURSE, B_GOLD_LARGE],
    ),
    make_event_option_template(
        L_TRANSFORM_2,
        &[EVENT_CONSUME_EFFECT, DW_CURSE, B_TRANSFORM_2],
    ),
    make_event_option_template(
        L_HP_20,
        &[EVENT_CONSUME_EFFECT, DW_CURSE, b_hp_20(HP_BONUS_A14)],
    ),
];
static CAT_2_DAMAGE_A0: [EventOptionTemplate; 7] = [
    make_event_option_template(
        L_COLORLESS_RARE,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A0), B_COLORLESS_RARE],
    ),
    make_event_option_template(
        L_REMOVE_2,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A0), B_REMOVE_2],
    ),
    make_event_option_template(
        L_RELIC_RARE,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A0), B_RELIC_RARE],
    ),
    make_event_option_template(
        L_RARE_CARDS,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A0), B_RARE_CARDS],
    ),
    make_event_option_template(
        L_GOLD_LARGE,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A0), B_GOLD_LARGE],
    ),
    make_event_option_template(
        L_TRANSFORM_2,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A0), B_TRANSFORM_2],
    ),
    make_event_option_template(
        L_HP_20,
        &[
            EVENT_CONSUME_EFFECT,
            dw_damage(DAMAGE_A0),
            b_hp_20(HP_BONUS_BASE),
        ],
    ),
];
static CAT_2_DAMAGE_A6: [EventOptionTemplate; 7] = [
    make_event_option_template(
        L_COLORLESS_RARE,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A6), B_COLORLESS_RARE],
    ),
    make_event_option_template(
        L_REMOVE_2,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A6), B_REMOVE_2],
    ),
    make_event_option_template(
        L_RELIC_RARE,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A6), B_RELIC_RARE],
    ),
    make_event_option_template(
        L_RARE_CARDS,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A6), B_RARE_CARDS],
    ),
    make_event_option_template(
        L_GOLD_LARGE,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A6), B_GOLD_LARGE],
    ),
    make_event_option_template(
        L_TRANSFORM_2,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A6), B_TRANSFORM_2],
    ),
    make_event_option_template(
        L_HP_20,
        &[
            EVENT_CONSUME_EFFECT,
            dw_damage(DAMAGE_A6),
            b_hp_20(HP_BONUS_BASE),
        ],
    ),
];
static CAT_2_DAMAGE_A14: [EventOptionTemplate; 7] = [
    make_event_option_template(
        L_COLORLESS_RARE,
        &[
            EVENT_CONSUME_EFFECT,
            dw_damage(DAMAGE_A14),
            B_COLORLESS_RARE,
        ],
    ),
    make_event_option_template(
        L_REMOVE_2,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A14), B_REMOVE_2],
    ),
    make_event_option_template(
        L_RELIC_RARE,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A14), B_RELIC_RARE],
    ),
    make_event_option_template(
        L_RARE_CARDS,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A14), B_RARE_CARDS],
    ),
    make_event_option_template(
        L_GOLD_LARGE,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A14), B_GOLD_LARGE],
    ),
    make_event_option_template(
        L_TRANSFORM_2,
        &[EVENT_CONSUME_EFFECT, dw_damage(DAMAGE_A14), B_TRANSFORM_2],
    ),
    make_event_option_template(
        L_HP_20,
        &[
            EVENT_CONSUME_EFFECT,
            dw_damage(DAMAGE_A14),
            b_hp_20(HP_BONUS_A14),
        ],
    ),
];

const T_BOSS_SWAP: EventOptionTemplate = make_event_option_template(
    "[Boss Swap] Lose Snake Ring. Obtain a random Boss Relic.",
    &[
        EVENT_CONSUME_EFFECT,
        Effect {
            kind: EffectKind::RelicLose,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::EventRelicPicks,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::Single,
            },
        },
        effect_relic_grant(RelicTier::Boss),
    ],
);

static BOSS: [EventOptionTemplate; 1] = [T_BOSS_SWAP];

// Catalog order: cat-0, cat-1, one table per drawback (DRAWBACKS order), boss
pub fn tables(ascension: u8) -> [&'static [EventOptionTemplate]; 7] {
    let (cat_1, hp_loss, gold_loss, curse): (
        &'static [EventOptionTemplate],
        &'static [EventOptionTemplate],
        &'static [EventOptionTemplate],
        &'static [EventOptionTemplate],
    ) = if ascension < 14 {
        (
            &CAT_1_BASE,
            &CAT_2_HP_LOSS_BASE,
            &CAT_2_GOLD_LOSS_BASE,
            &CAT_2_CURSE_BASE,
        )
    } else {
        (
            &CAT_1_A14,
            &CAT_2_HP_LOSS_A14,
            &CAT_2_GOLD_LOSS_A14,
            &CAT_2_CURSE_A14,
        )
    };
    let damage: &'static [EventOptionTemplate] = if ascension < 6 {
        &CAT_2_DAMAGE_A0
    } else if ascension < 14 {
        &CAT_2_DAMAGE_A6
    } else {
        &CAT_2_DAMAGE_A14
    };
    [&CAT_0, cat_1, hp_loss, gold_loss, curse, damage, &BOSS]
}

pub fn spawn_event_neow(state: &mut GameState) -> Vec<usize> {
    let tables = tables(state.ascension);
    let cat_0 = tables[0][state.rng.random_range(0..tables[0].len())];
    let cat_1 = tables[1][state.rng.random_range(0..tables[1].len())];

    // Drawback first, then its bonus; both option tables keep the source order
    let drawback_tables = &tables[2..6];
    let table_cat_2 = drawback_tables[state.rng.random_range(0..drawback_tables.len())];
    let cat_2 = table_cat_2[state.rng.random_range(0..table_cat_2.len())];

    let options = [cat_0, cat_1, cat_2, tables[6][0]];
    if let Some(id) = state.id_relics[RelicName::SnakeRing as usize] {
        state.event.id_relic_picks.push(id);
    }
    bake_options(state, &options)
}
