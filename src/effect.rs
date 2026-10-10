use crate::game::Location;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::CostScope;
use crate::types::DeltaSign;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::RewardKind;
use crate::types::ShopSlot;

// EffectKind: the shared "what happens" enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectKind {
    AccuracyResync,
    ActTransition,
    AdventurerSearch,
    BlockGain {
        amount: u16,
    },
    BlockSet {
        amount: u16,
    },
    BonfireOffer,
    CardAdd {
        card_name: CardName,
        card_pile: CardPile,
        count: u16,
        upgraded: bool,
    },
    CardAddRandom {
        color: CardColor,
        kind: Option<CardKind>,
        card_pile: CardPile,
        count: u16,
        cost_zero: Option<CostScope>,
        upgraded: bool,
        rarity: Option<CardRarity>,
    },
    CardAdopt {
        landing: bool, // The second pass, which CardAdopt queues for itself
    },
    CardBottle,
    CardCostMinusDiscards,
    CardDiscard {
        source: DiscardSource,
    },
    CardDiscoverPick {
        cost_zero: Option<CostScope>,
        card_pile: CardPile,
        copies: u8,
    },
    CardDiscoverRoll {
        kind: Option<CardKind>,
        color: CardColor,
        exclude: &'static [CardName],
        count: u8,
    },
    CardDraw {
        count: u16,
    },
    CardDrawIfNoAttacks {
        count: u16,
    },
    CardDrawUpTo {
        amount: u8,
    },
    CardDuplicate {
        card_pile: CardPile,
    },
    CardExhaust,
    CardFreePlaySpend,
    CardMove {
        card_pile: CardPile,
        cost_zero: Option<CostScope>,
    },
    CardNightmarePick,
    CardNightmareSpawn,
    CardPlace {
        pile: CardPile,
    },
    BombArm {
        turns: u8,
        damage: u16,
    },
    BombTick {
        seq: u32, // The Bomb's stamp
    },
    LifestealHeal,
    CardPlayRelocate,
    CardPlayFromCardPileDrawTop,
    CardPurge,
    CardRemove,
    CardRetain,
    CardSetupPick {
        free: bool,
        bottom: bool,
    },
    CardStasisReturn {
        hand_full: bool, // Fixed as the holder leaves; a full hand sends the Card to the discard pile
    },
    CardStasisSteal,
    CardTransform {
        upgraded: bool,
    },
    CardUpgrade,
    ChestOpen,
    CombatEnd {
        escaped_character: bool,
    },
    CombatStart {
        elite: bool,
    },
    DamageDeal {
        amount: u16,
        lifesteal: bool, // Life Suck
    },
    DamageFinisher {
        damage: u16,
    },
    DamageFlechettes {
        damage: u16,
    },
    DamageMindBlast, // Mind Blast's hit; its play turns it into a DamagePhysical for the draw pile's size
    DamagePhysical {
        amount: u16,
        instances: u16, // Hits on one target, resolved one at a time; an X-cost Card's X can pass 255
        lifesteal: bool, // Life Suck
    },
    DamagePhysicalIfPoisoned {
        amount: u16,
    },
    Death {
        with_leader: bool, // A Minion its leader's death takes down
    },
    DebuffsClear,
    DefensiveMode,
    DistractionAdd,
    DuVuDollRecount,
    EnergyDelta {
        sign: DeltaSign,
        amount: u16,
    },
    EscapePlanCheck {
        block: u16,
    },
    EventAdvanceState {
        delta: i8,
    },
    EventConsume,
    Gamble {
        choose_discards: bool,
    },
    GambleDraw {
        discards_before: u16, // This turn's discards as the Gamble began
    },
    GiryaLift,
    GlassKnifeDecay {
        delta: i16,
    },
    GoldDelta {
        sign: DeltaSign,
        amount: Amount,
    },
    GoldSteal {
        amount: u8,
    },
    GremlinSummon,
    HandOfGreedProc {
        gold: u16,
    },
    HealthDelta {
        sign: DeltaSign,
        amount: Amount,
    },
    HealthLowerTo {
        amount: Amount,
    },
    HeelHookProc,
    HexaghostBurnIncrease {
        count: u8,
    },
    JoustBet {
        on_owner: bool,
    },
    KnowingSkullCostBump,
    LagavulinWake,
    MatchFlipSeen,
    MatchFlipUnseen,
    MausoleumOpen,
    MaxHealthDelta {
        sign: DeltaSign,
        amount: Amount,
    },
    MayhemProc,
    ModifierDelta {
        kind: ModifierKind,
        stacks: i16,
    },
    ModifierMultiply {
        kind: ModifierKind,
        factor: u8,
    },
    ModifierRemove {
        kind: ModifierKind,
    },
    ModifierTick,
    MonsterEscape,
    MonsterRemove,
    MonsterSpawn {
        name: MonsterName,
        minion: bool, // Gremlin Leader's summons
        slot: usize,
    },
    MonsterSplit {
        name: MonsterName,
        slot_offset: isize,
    },
    MoveExecute,
    MoveUpdate {
        move_override: Option<usize>,
    },
    NoOp,
    PoisonTick {
        amount: u16,
    },
    PotionAddRandom {
        limited: bool,
        uniform: bool,
    },
    PotionAdopt,
    PotionDiscard,
    PotionUse {
        at_pick: bool, // Drunk while a pick waits
    },
    RelicAdopt,
    RelicGrantPool {
        pool: &'static [RelicName],
    },
    RelicGrantRandom {
        tier: Option<RelicTier>,
        exclusion: RelicExclusion,
    },
    RelicGrantSpecific {
        name: RelicName,
    },
    RelicLose,
    RestDig,
    RestSiteConsume,
    RestSmith,
    RestToke,
    RitualDaggerProc {
        bump: u16,
    },
    RewardRollCards {
        bundles: u8,
        trigger: RewardRollTrigger,
    },
    RewardRollGold {
        amount: Amount,
    },
    RewardRollPotions {
        count: u8,
        trigger: RewardRollTrigger,
    },
    RelicRewardRemoveOne,
    RewardRollRelic {
        pick: RelicPick,
        exclusion: RelicExclusion,
    },
    RewardTake {
        kind: RewardKind,
    },
    RoomEnter {
        location: Location,
        landing: bool, // The second pass, which RoomEnter queues behind the entry Relics' gold
    },
    RoomExit,
    RoomSelect,
    ScrapOozeReach {
        chance: u8,
        advance_on_miss: bool,
    },
    SetCostOverride {
        amount: u8,
        only_reduce: bool,
        random: bool,
        scope: CostScope,
    },
    ShopBuild,
    ShopBuy {
        slot: ShopSlot,
    },
    ShopPurge,
    ShuffleCardPileDiscardIntoCardPileDraw,
    SingingBowlProc {
        idx_bundle: u8,
    },
    SneakyStrikeProc {
        energy: u8,
    },
    StormOfSteelProc {
        upgraded: bool,
    },
    StrengthLoseTemp {
        stacks: i16,
    },
    TurnEndCharacter {
        landing: bool, // The second pass, which TurnEndCharacter queues for itself
    },
    TurnEndMonster,
    TurnMonsters {
        stage: TurnMonstersStage,
    },
    TurnStartCharacter,
    TurnStartMonster,
    UnloadDiscard,
    WheelSpin,
}

// How far the staged Relic is already resolved; each variant rolls only what remains
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelicPick {
    Thresholds { th_common: u8, th_uncommon: u8 },
    Tier(RelicTier),
    Pool(&'static [RelicName]), // Uniform over the unowned entries; Circlet when all are owned
    Name(RelicName),
}

// Redraw loops around the relic draw
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelicExclusion {
    Unfiltered,
    Screenless,  // Skip Bottles and Whetstone
    NonCampfire, // Skip Girya, Shovel, Peace Pipe
}

// Who is asking for a reward roll
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RewardRollTrigger {
    CombatMonster,
    CombatElite,
    CombatBoss,
    EventFight,
    EventFightUnpaid,
    SmokeBomb,
    DreamCatcher,
    Orrery,
    Library,
    Neow,
    NeowRare,
    NeowColorless,
    NeowColorlessRare,
    NeowPotions,
    Cauldron,
    WomanInBlue,
    Lab,
    TinyHouse,
    CallingBell,
}

// Origin tag the CardDiscard handler branches on
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiscardSource {
    Explicit,
    ExplicitRelicsFirst, // Unload and the Gamble pick: the discard Relics fire before the Card's own discard effects
    EndOfTurn,
}

// Origin tag the CardPlay handler branches on; Replay is a Burst / Duplication / Necronomicon replay
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaySource {
    Hand,
    DrawTop,
    Replay,
    TurnEnd { hand_size: u16 }, // A Card playing itself out of hand; Regret loses the hand size as the turn ended
}

// The Monsters' half of the round, each stage waiting for everything queued before it
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurnMonstersStage {
    TurnStarts, // Every Monster's turn starts, its Poison tick with it
    Moves,      // The Monsters still standing act one at a time
    RoundEnd,   // Every Monster's turn ends, then the Character's next turn starts
}

// A Card play waiting in the card play queue; it starts once every queued effect has resolved
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CardPlay {
    pub id_card: usize,
    pub id_target: Option<usize>,
    pub play_source: PlaySource,
    pub energy: u16,         // Fixed when queued; X-cost Cards read it as X
    pub draw_pile_size: u16, // Fixed when queued; Mind Blast hits for it
}

// Which value a Relative amount's fraction reads: the current one, or the one the event opened with
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReadAt {
    Now,
    EventOpen,
}

// How a Relative amount rounds its fraction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rounding {
    Truncate,       // Rounded down
    TruncateMinOne, // Rounded down, then raised to at least 1
    HalfUp,         // Rounded half-up
    Ceil,           // Rounded up
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Amount {
    Absolute(u16),
    Relative {
        numerator: u8,
        denominator: u8,
        rounding: Rounding,
        read_at: ReadAt,
    },
    Range {
        min: u16,
        max: u16,
    },
}

// Source pool for a Resolve effect
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CandidatePool {
    CardPileHand,
    Character,
    Monsters,
    MonsterPicked,
    Source,
    CardPileDiscover,
    CardPileDeck,
    CardPileDraw,
    CardPileDiscard,
    CardPileExhaust,
    CardPileEventRoll,
    RelicEventRoll,
    PotionEventRoll,
}

// Only Card pools are ever multi-pick
pub const fn pool_is_cards(pool: CandidatePool) -> bool {
    matches!(
        pool,
        CandidatePool::CardPileHand
            | CandidatePool::CardPileDiscover
            | CandidatePool::CardPileDeck
            | CandidatePool::CardPileDraw
            | CandidatePool::CardPileDiscard
            | CandidatePool::CardPileExhaust
            | CandidatePool::CardPileEventRoll
    )
}

// Input picks above one stage before applying; every other kind resolves at once
pub const fn is_multi_pick(selection_kind: SelectionKind) -> bool {
    match selection_kind {
        SelectionKind::Input { count } | SelectionKind::InputUpTo { count } => count > 1,
        _ => false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CandidateFilter {
    // Compare against `Entity` fields
    Upgradeable,
    KindAttack,
    KindSkill,
    KindPower,
    KindCurse,
    Costed,
    NotBottled,
    NotBoundCurse,

    // Compare against the `Target::Resolve` context
    NotSource,
    NotSourceUnlessAlone,
    NotMinion,

    // Starter-Card predicates (Vampires, Back to Basics)
    Starter, // Strike or Defend
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionKind {
    All,
    Single,
    Random { count: u8 },
    Input { count: u16 },
    InputUpTo { count: u16 },
}

// Target known at queue time (Direct) or resolved against live state at dequeue (Resolve)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Target {
    // Known target, or `None` for targetless effects (CardDraw, EnergyGain).
    Direct(Option<usize>),

    // Resolved against live state at dequeue via `resolve_selection_kind`.
    Resolve {
        candidate_pool: CandidatePool,
        // Left to right, Costed and NotSourceUnlessAlone last; an empty list keeps every candidate
        filters: &'static [CandidateFilter],
        selection_kind: SelectionKind,
    },
}

// A unit of work in the queue; static defs use `Resolve`, runtime uses `Direct`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Effect {
    pub kind: EffectKind,
    pub id_source: Option<usize>,
    pub target: Target,
}

// Filler for slots past `card_effects_play_len` in Entity.card_effects_play
pub const EFFECT_ZERO: Effect = Effect {
    kind: EffectKind::NoOp,
    id_source: None,
    target: Target::Direct(None),
};

// The Resolve shapes static defs use almost everywhere
pub const TARGET_CHARACTER: Target = Target::Resolve {
    candidate_pool: CandidatePool::Character,
    filters: &[],
    selection_kind: SelectionKind::Single,
};

// Accuracy's sweep of one pool: every Shiv in it resets to its printed damage plus Accuracy
pub const fn effect_accuracy_resync(candidate_pool: CandidatePool) -> Effect {
    Effect {
        kind: EffectKind::AccuracyResync,
        id_source: None,
        target: Target::Resolve {
            candidate_pool,
            filters: &[],
            selection_kind: SelectionKind::All,
        },
    }
}

pub const EFFECT_ACCURACY_RESYNC_HAND: Effect = effect_accuracy_resync(CandidatePool::CardPileHand);

// Du-Vu Doll's recount, queued by its pickup and by every Card landing in or purged from the deck
pub const EFFECT_DU_VU_DOLL_RECOUNT: Effect = Effect {
    kind: EffectKind::DuVuDollRecount,
    id_source: None,
    target: Target::Direct(None),
};

// Discover pick: choose 1 of the rolled Cards, or none if skippable; cost break, destination and copies vary by caller
pub const fn effect_discover_pick(
    cost_zero: Option<CostScope>,
    pile: CardPile,
    copies: u8,
    skippable: bool,
) -> Effect {
    let selection_kind = if skippable {
        SelectionKind::InputUpTo { count: 1 }
    } else {
        SelectionKind::Input { count: 1 }
    };
    Effect {
        kind: EffectKind::CardDiscoverPick {
            cost_zero,
            card_pile: pile,
            copies,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::CardPileDiscover,
            filters: &[],
            selection_kind,
        },
    }
}

// The pick outlives the roster slot, so a lethal hit still resolves a target
pub const TARGET_MONSTER_PICKED: Target = Target::Resolve {
    candidate_pool: CandidatePool::MonsterPicked,
    filters: &[],
    selection_kind: SelectionKind::Single,
};

pub const TARGET_SOURCE: Target = Target::Resolve {
    candidate_pool: CandidatePool::Source,
    filters: &[],
    selection_kind: SelectionKind::Single,
};

pub const TARGET_MONSTERS_ALL: Target = Target::Resolve {
    candidate_pool: CandidatePool::Monsters,
    filters: &[],
    selection_kind: SelectionKind::All,
};
