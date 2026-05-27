from enum import IntEnum
from typing import Iterator, NamedTuple, Optional, Union

class CardKind(IntEnum):
    Attack: int
    Skill: int
    Power: int
    Curse: int
    Status: int

class CardColor(IntEnum):
    Green: int
    Colorless: int
    Curse: int

class CardRarity(IntEnum):
    Basic: int
    Common: int
    Uncommon: int
    Rare: int
    Special: int
    Curse: int

class RoomKind(IntEnum):
    CombatMonster: int
    CombatElite: int
    CombatBoss: int
    RestSite: int

class ModifierKind(IntEnum):
    Accuracy: int
    AfterImage: int
    Angry: int
    Artifact: int
    Asleep: int
    Blur: int
    Burst: int
    Choke: int
    CorpseExplosion: int
    CurlUp: int
    Dexterity: int
    DoubleDamage: int
    DrawCardNextTurn: int
    Enrage: int
    Entangled: int
    Envenom: int
    Frail: int
    InfiniteBlades: int
    Intangible: int
    Metallicize: int
    ModeShift: int
    NextTurnBlock: int
    NextTurnEnergy: int
    NoDraw: int
    NoxiousFumes: int
    Phantasmal: int
    PlatedArmor: int
    Poison: int
    Retain: int
    Ritual: int
    Shackled: int
    SharpHide: int
    Splittable: int
    SporeCloud: int
    Strength: int
    Thievery: int
    Thorns: int
    ThousandCuts: int
    ToolsOfTheTrade: int
    Vigor: int
    Vulnerable: int
    Weak: int
    WraithForm: int

class IntentKind(IntEnum):
    Attack: int
    AttackBlock: int
    AttackBuff: int
    AttackDebuff: int
    Block: int
    BlockBuff: int
    Buff: int
    Debuff: int
    DebuffPowerful: int
    Escape: int
    Sleep: int
    Stunned: int
    Unknown: int

class CandidatePool(IntEnum):
    Hand: int
    Character: int
    Monsters: int
    Source: int
    NextRowRooms: int
    Discover: int
    Deck: int

class CandidatePoolMonstersFilter(IntEnum):
    All: int
    Other: int
    Picked: int

class RelicName(IntEnum):
    SnakeRing: int
    Akabeko: int
    Anchor: int
    BagOfMarbles: int
    BagOfPreparation: int
    BloodVial: int
    BronzeScales: int
    Kunai: int
    NinjaScroll: int
    OddlySmoothStone: int
    Shuriken: int
    ThreadAndNeedle: int
    TwistedFunnel: int
    Vajra: int
    Circlet: int
    GoldenIdol: int

class RelicTier(IntEnum):
    Starter: int
    Common: int
    Uncommon: int
    Rare: int
    Boss: int
    Shop: int
    Special: int

class PotionName(IntEnum):
    EnergyPotion: int
    BlockPotion: int
    StrengthPotion: int
    DexterityPotion: int
    FirePotion: int
    ExplosivePotion: int
    WeakPotion: int
    FearPotion: int
    PoisonPotion: int
    SwiftPotion: int
    AttackPotion: int
    SkillPotion: int
    PowerPotion: int
    FruitJuice: int

class PotionRarity(IntEnum):
    Common: int
    Uncommon: int
    Rare: int

class CardName(IntEnum):
    AThousandCuts: int
    Accuracy: int
    Acrobatics: int
    Adrenaline: int
    AfterImage: int
    AllOutAttack: int
    Backflip: int
    Backstab: int
    BandageUp: int
    Bane: int
    BladeDance: int
    Blind: int
    Blur: int
    BouncingFlask: int
    BulletTime: int
    Burn: int
    Burst: int
    CalculatedGamble: int
    Caltrops: int
    Catalyst: int
    Choke: int
    CloakAndDagger: int
    Concentrate: int
    CorpseExplosion: int
    CripplingPoison: int
    DaggerSpray: int
    DaggerThrow: int
    Dash: int
    Dazed: int
    DeadlyPoison: int
    DeepBreath: int
    Defend: int
    Deflect: int
    DieDieDie: int
    Distraction: int
    DodgeAndRoll: int
    Doppelganger: int
    EndlessAgony: int
    Envenom: int
    EscapePlan: int
    Eviscerate: int
    Expertise: int
    Finesse: int
    Finisher: int
    FlashOfSteel: int
    Flechettes: int
    FlyingKnee: int
    Footwork: int
    GlassKnife: int
    GoodInstincts: int
    GrandFinale: int
    HeelHook: int
    InfiniteBlades: int
    LegSweep: int
    Malaise: int
    MasterOfStrategy: int
    MasterfulStab: int
    MindBlast: int
    Neutralize: int
    Nightmare: int
    NoxiousFumes: int
    Outmaneuver: int
    PhantasmalKiller: int
    PiercingWail: int
    PoisonedStab: int
    Predator: int
    Prepared: int
    QuickSlash: int
    Reflex: int
    RiddleWithHoles: int
    Setup: int
    Shiv: int
    Skewer: int
    Slice: int
    Slimed: int
    SneakyStrike: int
    StormOfSteel: int
    Strike: int
    SuckerPunch: int
    Survivor: int
    SwiftStrike: int
    Tactician: int
    Terror: int
    ToolsOfTheTrade: int
    Unload: int
    WellLaidPlans: int
    WraithForm: int

class MonsterName(IntEnum):
    Cultist: int
    FungiBeast: int
    GremlinFat: int
    GremlinNob: int
    GremlinThief: int
    GremlinTsundere: int
    GremlinWarrior: int
    GremlinWizard: int
    Hexaghost: int
    JawWorm: int
    Lagavulin: int
    Looter: int
    LouseDefensive: int
    LouseNormal: int
    Sentry: int
    SlaverBlue: int
    SlaverRed: int
    SlimeAcidLarge: int
    SlimeAcidMedium: int
    SlimeAcidSmall: int
    SlimeBoss: int
    SlimeSpikeLarge: int
    SlimeSpikeMedium: int
    SlimeSpikeSmall: int
    TheGuardian: int

class ActionType(IntEnum):
    CardDiscover: int
    CardPlay: int
    ChestOpen: int
    DeckDuplicate: int
    DeckPurge: int
    DeckTransform: int
    DeckUpgrade: int
    EventSelect: int
    HandDiscard: int
    HandNightmarePick: int
    HandRetain: int
    HandSetupPick: int
    PotionDiscard: int
    PotionUse: int
    RestSiteRest: int
    RestSiteUpgrade: int
    RewardSkip: int
    RewardTakeCard: int
    RewardTakeGold: int
    RewardTakePotion: int
    RewardTakeRelic: int
    RoomSelect: int
    RoomSkip: int
    TurnEnd: int

class CandidatePoolDeckFilter(IntEnum):
    Purgeable: int
    Upgradeable: int
    Any: int
    Transformable: int

class Screen(IntEnum):
    Combat: int
    Reward: int
    Event: int
    Shop: int
    Map: int
    RestSite: int
    Chest: int

class EventName(IntEnum):
    BigFish: int
    TheCleric: int
    Duplicator: int
    GoldenShrine: int
    GoldenIdol: int
    WingStatue: int
    WorldOfGoop: int
    LivingWall: int
    Purifier: int
    ScrapOoze: int
    ShiningLight: int
    TheSsssserpent: int
    Transmogrifier: int
    UpgradeShrine: int

class EventOption:
    label: str
    gated_out: bool
    effects: list

class Event:
    name: EventName
    display_name: str
    options: list[EventOption]
    state: int

class CardCostKind:
    class Fixed:
        def __init__(self) -> None: ...

    class MinusDiscardsThisTurn:
        def __init__(self) -> None: ...

    class GrowsOnDamageInstanceTaken:
        def __init__(self) -> None: ...

    class XCost:
        offset: int
        def __init__(self, offset: int) -> None: ...

class Reward:
    cards: list[Card]
    relic: Relic | None
    potion: Potion | None
    gold: int | None

class PendingInput:
    class Discard:
        num: int
        def __init__(self, num: int) -> None: ...

    class Retain:
        num: int
        def __init__(self, num: int) -> None: ...

    class Setup:
        def __init__(self) -> None: ...

    class Nightmare:
        def __init__(self) -> None: ...

    class Discover:
        cards: list[Card]
        def __init__(self, cards: list[Card]) -> None: ...

    class DeckSelect:
        filter: CandidatePoolDeckFilter
        cards: list[Card]
        def __init__(self, filter: CandidatePoolDeckFilter, cards: list[Card]) -> None: ...

    class RoomSelect:
        def __init__(self) -> None: ...

class SelectionKind:
    class All:
        def __init__(self) -> None: ...

    class Single:
        def __init__(self) -> None: ...

    class Random:
        count: int
        def __init__(self, count: int) -> None: ...

    class Input:
        count: int
        def __init__(self, count: int) -> None: ...

class Target:
    candidate_pool: CandidatePool
    selection_kind: SelectionKind

class Effect:
    class DamagePhysical:
        amount: int
        target: Optional[Target]

    class DamagePhysicalIfPoisoned:
        amount: int
        target: Optional[Target]

    class HeelHookProc:
        target: Optional[Target]

    class EscapePlanCheck:
        block: int
        target: Optional[Target]

    class GlassKnifeDecay:
        delta: int
        target: Optional[Target]

    class CardSetupPick:
        target: Optional[Target]

    class CardNightmarePick:
        target: Optional[Target]

    class CardDiscover:
        kind: CardKind
        count: int
        target: Optional[Target]

    class DistractionAdd:
        target: Optional[Target]

    class SetCostOverride:
        amount: int
        target: Optional[Target]

    class DamageFinisher:
        damage: int
        target: Optional[Target]

    class DamageFlechettes:
        damage: int
        target: Optional[Target]

    class UnloadDiscard:
        target: Optional[Target]

    class StormOfSteelProc:
        upgraded: bool
        target: Optional[Target]

    class SneakyStrikeProc:
        energy: int
        target: Optional[Target]

    class BlockGain:
        amount: int
        target: Optional[Target]

    class ModifierGain:
        kind: ModifierKind
        stacks: int
        target: Optional[Target]

    class ModifierMultiply:
        kind: ModifierKind
        factor: int
        target: Optional[Target]

    class ModifierRemove:
        kind: ModifierKind
        target: Optional[Target]

    class EnergyGain:
        amount: int
        target: Optional[Target]

    class CardAddToHand:
        card_name: str
        count: int
        upgraded: bool
        target: Optional[Target]

    class CardDraw:
        count: int
        target: Optional[Target]

    class CardDrawUpTo:
        amount: int
        target: Optional[Target]

    class CardDiscard:
        target: Optional[Target]

    class DamageMindBlast:
        target: Optional[Target]

    class ShuffleDiscardPileIntoDrawPile:
        target: Optional[Target]

    class CalculatedGamble:
        target: Optional[Target]

class Action:
    action_type: ActionType
    idxs: list[int]
    kind: int | None

    def __init__(
        self,
        action_type: ActionType,
        idxs: list[int],
        kind: int | None = None,
    ) -> None: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class ArgSpec(NamedTuple):
    name: str
    description: str
    optional: bool = False
    variable: bool = False

class ActionSpec(NamedTuple):
    id: ActionType
    name: str
    args: tuple[ArgSpec, ...]
    arity: tuple[int, Optional[int]]

class ActionSpecRegistry:
    CardPlay: ActionSpec
    TurnEnd: ActionSpec
    HandSelect: ActionSpec
    RoomSelect: ActionSpec
    RestSiteRest: ActionSpec
    RestSiteCardUpgrade: ActionSpec
    RoomSkip: ActionSpec
    ChestOpen: ActionSpec
    PotionUse: ActionSpec
    PotionDiscard: ActionSpec
    CardDiscover: ActionSpec
    RewardTakeCard: ActionSpec
    RewardTakeRelic: ActionSpec
    RewardTakePotion: ActionSpec
    RewardTakeGold: ActionSpec
    RewardSkip: ActionSpec

    def __getattr__(self, name: str) -> ActionSpec: ...
    def __getitem__(self, key: Union[int, str, ActionType]) -> ActionSpec: ...
    def __iter__(self) -> Iterator[ActionSpec]: ...
    def __len__(self) -> int: ...
    def __contains__(self, key: object) -> bool: ...

ACTION_SPEC_REGISTRY: ActionSpecRegistry

class Modifier:
    kind: ModifierKind
    stacks: int
    stacks_max: int

class Relic:
    name: RelicName
    tier: RelicTier
    counter: int
    used_up: bool

class Potion:
    name: PotionName
    rarity: PotionRarity
    requires_target: bool
    combat_only: bool
    effects: list[Effect]

class Card:
    name: CardName
    display_name: str

    # Cost-related fields
    cost: int
    cost_base: int
    cost_zero_once: bool
    cost_override: Optional[int]
    cost_kind: Union[
        CardCostKind.Fixed,
        CardCostKind.MinusDiscardsThisTurn,
        CardCostKind.GrowsOnDamageInstanceTaken,
        CardCostKind.XCost,
    ]

    # Categorical fields
    kind: CardKind
    color: CardColor
    rarity: CardRarity

    # Other boolean fields
    upgraded: bool
    exhaust: bool
    ethereal: bool
    innate: bool
    requires_target: bool
    retain: bool
    playable: bool

    # Effects
    effects: list[
        Union[
            Effect.DamagePhysical,
            Effect.DamagePhysicalIfPoisoned,
            Effect.HeelHookProc,
            Effect.EscapePlanCheck,
            Effect.GlassKnifeDecay,
            Effect.CardSetupPick,
            Effect.CardNightmarePick,
            Effect.CardDiscoverRoll,
            Effect.DistractionAdd,
            Effect.SetCostOverride,
            Effect.DamageFinisher,
            Effect.DamageFlechettes,
            Effect.UnloadDiscard,
            Effect.StormOfSteelProc,
            Effect.SneakyStrikeProc,
            Effect.BlockGain,
            Effect.ModifierGain,
            Effect.ModifierMultiply,
            Effect.ModifierRemove,
            Effect.EnergyGain,
            Effect.CardAddToHand,
            Effect.CardDraw,
            Effect.CardDrawUpTo,
            Effect.CardDiscard,
            Effect.DamageMindBlast,
            Effect.ShuffleDiscardPileIntoDrawPile,
            Effect.CalculatedGamble,
        ]
    ]

class Character:
    name: str
    health: int
    health_max: int
    block: int
    modifiers: list[Modifier]
    gold: int

class Intent:
    kind: IntentKind
    damage: Optional[int]
    instances: Optional[int]

class Monster:
    name: MonsterName
    display_name: str
    health: int
    health_max: int
    block: int
    modifiers: list[Modifier]
    intent: Intent

class Energy:
    energy_current: int
    energy_max: int

class Room:
    room_kind: RoomKind
    edges: list[int]

class Map:
    rooms: list[list[Optional[Room]]]
    y_current: Optional[int]
    x_current: Optional[int]
    boss_name: str  # TODO: maybe should be in `GameState`?

class GameState:
    # Actors
    character: Character
    monsters: list[Monster]

    # Card piles
    deck: list[Card]
    hand: list[Card]
    pile_draw: list[Card]
    pile_discard: list[Card]
    pile_exhaust: list[Card]

    # Relics, Energy and Map
    relics: list[Relic]
    energy: Energy
    map: Map
    screen: Screen
    game_over: bool
    reward: Reward | None
    event: Event | None
    pending_input: Union[
        PendingInput.Discard,
        PendingInput.Retain,
        PendingInput.Setup,
        PendingInput.Nightmare,
        PendingInput.Discover,
        PendingInput.DeckSelect,
        PendingInput.RoomSelect,
    ] | None

class GameEnv:
    MAX_MONSTERS: int
    MAX_SIZE_HAND: int
    MAX_COMBAT_CARD_REWARD: int
    CARDS_DRAWN_PER_TURN: int
    NIGHTMARE_COPIES: int
    MAX_BLOCK: int
    MAP_HEIGHT: int
    MAP_WIDTH: int

    def __init__(self, ascension: int = 0) -> None: ...
    def reset(self, seed: int = 42) -> GameState: ...
    def step(self, action: Action) -> tuple[GameState, bool]: ...
    def get_legal_actions(self) -> list[Action]: ...
