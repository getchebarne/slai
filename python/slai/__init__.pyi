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
    CardTarget: int
    Character: int
    Monsters: int
    OtherMonsters: int
    Source: int
    NextRowRooms: int
    CardRewardPool: int

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

class RelicTier(IntEnum):
    Starter: int
    Common: int
    Uncommon: int
    Rare: int
    Boss: int
    Shop: int
    Special: int

class CardName(IntEnum):
    AThousandCuts: int
    Accuracy: int
    Acrobatics: int
    Adrenaline: int
    AfterImage: int
    AllOutAttack: int
    Backflip: int
    Backstab: int
    Bane: int
    BladeDance: int
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
    Finisher: int
    Flechettes: int
    FlyingKnee: int
    Footwork: int
    GlassKnife: int
    GrandFinale: int
    HeelHook: int
    InfiniteBlades: int
    LegSweep: int
    Malaise: int
    MasterfulStab: int
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
    CardPlay: int
    EndTurn: int
    CardDiscard: int
    CardRetain: int
    CardSetup: int
    CardNightmare: int
    RoomSelect: int
    CardRewardSelect: int
    RelicRewardSelect: int
    RestSiteRest: int
    RestSiteCardUpgrade: int
    RoomSkip: int
    ChestOpen: int
    PotionUse: int
    PotionDiscard: int
    CardDiscoverPick: int
    PotionRewardSelect: int
    GoldRewardTake: int
    RewardSkip: int


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

class Phase:
    class Map:
        def __init__(self) -> None: ...

    class CombatDefault:
        def __init__(self) -> None: ...

    class CombatAwaitDiscard:
        num: int
        def __init__(self, num: int) -> None: ...

    class CombatAwaitRetain:
        num: int
        def __init__(self, num: int) -> None: ...

    class CombatAwaitNightmare:
        def __init__(self) -> None: ...

    class CombatAwaitSetup:
        def __init__(self) -> None: ...

    class CombatAwaitDiscover:
        cards: list[Card]
        def __init__(self, cards: list[Card]) -> None: ...

    class Reward:
        cards: list[Card]
        relic: Relic | None
        potion: Potion | None
        gold: int | None
        def __init__(
            self,
            cards: list[Card],
            relic: Relic | None,
            potion: Potion | None,
            gold: int | None,
        ) -> None: ...

    class RestSite:
        def __init__(self) -> None: ...

    class GameOver:
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

    class CardDiscoverPick:
        kind: CardKind
        count: int
        target: Optional[Target]

    class DistractionAdd:
        target: Optional[Target]

    class SetCostOverride:
        amount: int
        target: Optional[Target]

    class FinisherDamage:
        damage: int
        target: Optional[Target]

    class FlechettesDamage:
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

    class DrawUpTo:
        amount: int
        target: Optional[Target]

    class CardDiscard:
        target: Optional[Target]

    class CalculatedGamble:
        target: Optional[Target]


class Action:
    action_type: ActionType
    idxs: list[int]

    def __init__(self, action_type: ActionType, idxs: list[int]) -> None: ...
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
    EndTurn: ActionSpec
    CardDiscard: ActionSpec
    CardRetain: ActionSpec
    CardSetup: ActionSpec
    CardNightmare: ActionSpec
    RoomSelect: ActionSpec
    CardRewardSelect: ActionSpec
    RelicRewardSelect: ActionSpec
    RestSiteRest: ActionSpec
    RestSiteCardUpgrade: ActionSpec
    RoomSkip: ActionSpec
    ChestOpen: ActionSpec
    PotionUse: ActionSpec
    PotionDiscard: ActionSpec
    CardDiscoverPick: ActionSpec
    PotionRewardSelect: ActionSpec
    GoldRewardTake: ActionSpec
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
            Effect.CardDiscoverPick,
            Effect.DistractionAdd,
            Effect.SetCostOverride,
            Effect.FinisherDamage,
            Effect.FlechettesDamage,
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
            Effect.DrawUpTo,
            Effect.CardDiscard,
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
    current: int
    max: int

class Room:
    room_kind: RoomKind
    edges: list[int]

class Map:
    rooms: list[list[Optional[Room]]]
    y_current: Optional[int]
    x_current: Optional[int]
    boss_name: str # TODO: maybe should be in `GameState`?

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

    # Phase
    phase: Union[
        Phase.Map,
        Phase.CombatDefault,
        Phase.CombatAwaitDiscard,
        Phase.CombatAwaitRetain,
        Phase.CombatAwaitNightmare,
        Phase.CombatAwaitSetup,
        Phase.Reward,
        Phase.RestSite,
        Phase.GameOver,
    ]


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
