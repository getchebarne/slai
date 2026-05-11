"""Type stubs for the slai package.

Hand-maintained. Keep in sync with `src/ffi.rs`, `src/lib.rs`, and the
IntEnum shim + `ACTION_SPECS` registry in `python/slai/__init__.py`.

Architecture:
- The PyO3 compiled extension at `slai.slai` exposes raw `#[pyclass]`
  types: views, `GameEnv`, `Action`, complex enums (`Phase`, `Effect`,
  `Target`, `Selection`, `CardCostKind`), and the raw unit enums.
- `python/slai/__init__.py` wraps every unit-only PyO3 enum as a real
  `enum.IntEnum` and builds the `ACTION_SPECS` registry (PySC2's
  `Functions`/`Function`/`ArgumentType` separation). Those IntEnums are
  what users see as `slai.CardKind`, `slai.ActionType`, etc.
- These stubs describe what users see at the `slai` namespace, not what
  the raw `slai.slai` extension exposes.
"""

from enum import IntEnum
from typing import Iterator, NamedTuple, Optional, Union

# ───────── Unit enums (IntEnum shims over PyO3 unit-only enums) ─────────

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
    """Categorical tag on a monster's telegraphed intent. Multiple flags
    on `Intent` (block / buff / debuff) can co-occur with these."""

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
    """Canonical card identity (78 variants). Stable across upgrades —
    use `Card.upgraded` to distinguish base from upgraded form. Preferred
    over the display string `Card.name` for one-hot encoding."""

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
    """Canonical monster identity (25 variants). Preferred over the
    display string `Monster.name` for one-hot encoding."""

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
    """Discriminant for `Action`. Pure identity — per-action argument
    schema lives in `ACTION_SPECS`."""

    CardPlay: int
    EndTurn: int
    CardDiscard: int
    CardRetain: int
    CardSetup: int
    CardNightmare: int
    RoomSelect: int
    CardRewardSelect: int
    CardRewardSkip: int
    RelicRewardSelect: int
    RelicRewardSkip: int
    RestSiteRest: int
    RestSiteCardUpgrade: int

# ───────── Complex enums (PyO3 #[pyclass] enums, kept as-is) ─────────

class CardCostKind:
    """How `Card.cost` is derived from `Card.base_cost`."""

    class Fixed:
        """`cost == base_cost` always. Most cards."""
        def __init__(self) -> None: ...

    class MinusDiscardsThisTurn:
        """Cost decreases by the number of discards this turn (Eviscerate)."""
        def __init__(self) -> None: ...

    class GrowsOnDamageInstanceTaken:
        """Cost grows with damage instances taken this combat (Glass Knife)."""
        def __init__(self) -> None: ...

    class XCost:
        """Cost is current energy at play time (X-cost cards)."""
        offset: int
        def __init__(self, offset: int) -> None: ...

class Phase:
    """Game phase. Variants carry data for halts that need a count."""

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

    class CombatReward:
        def __init__(self) -> None: ...

    class RestSite:
        def __init__(self) -> None: ...

    class GameOver:
        def __init__(self) -> None: ...

class Selection:
    """How the engine picks targets from a CandidatePool."""

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
    """The pool an effect resolves against, plus how one or more targets
    are picked from it. `Effect.target` is None when the effect needs no
    resolution at all (e.g. CardDraw, EnergyGain on the player)."""

    candidates: CandidatePool
    selection: Selection

class Effect:
    """A static card or monster effect. `target` is None when the effect
    needs no resolution (e.g. Adrenaline gives the player energy directly)."""

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
        target: int
        target_field: Optional[Target]

    class CardDiscard:
        target: Optional[Target]

    class CalculatedGamble:
        target: Optional[Target]

# ───────── Flat Action + schema registry ─────────

class Action:
    """A player action passed to `GameEnv.step()`.

    Construct as `Action(action_type, indices)` where `indices` is a list
    of positional indices whose meaning depends on `action_type`. See
    `ACTION_SPECS` for the per-`ActionType` schema, or:

      CardPlay            : [idx_hand] or [idx_hand, idx_monster]
      EndTurn             : []
      CardDiscard         : [idx_hand, idx_hand, ...]  (count set by Phase)
      CardRetain          : [idx_hand, idx_hand, ...]  (count set by Phase)
      CardSetup           : [idx_hand]
      CardNightmare       : [idx_hand]
      RoomSelect          : [idx_column]
      CardRewardSelect    : [idx_reward]
      CardRewardSkip      : []
      RelicRewardSelect   : [idx_reward]
      RelicRewardSkip     : []
      RestSiteRest        : []
      RestSiteCardUpgrade : [idx_deck]

    All indices are positions into per-action collections (hand, alive
    monsters, map row, reward slots, deck), not entity ids.
    """

    action_type: ActionType
    indices: list[int]

    def __init__(self, action_type: ActionType, indices: list[int]) -> None: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class ArgSpec(NamedTuple):
    """Schema for one positional slot in `Action.indices`.

    Mirrors `pysc2.lib.actions.ArgumentType`.
    """

    name: str
    description: str
    optional: bool = False
    variable: bool = False

class ActionSpec(NamedTuple):
    """Full schema for one action type. Mirrors `pysc2.lib.actions.Function`.

    `ActionType` carries identity; this carries the data.
    """

    id: ActionType
    name: str
    args: tuple[ArgSpec, ...]
    arity: tuple[int, Optional[int]]

class ActionSpecs:
    """Registry of action type specs. Mirrors `pysc2.lib.actions.Functions`."""

    CardPlay: ActionSpec
    EndTurn: ActionSpec
    CardDiscard: ActionSpec
    CardRetain: ActionSpec
    CardSetup: ActionSpec
    CardNightmare: ActionSpec
    RoomSelect: ActionSpec
    CardRewardSelect: ActionSpec
    CardRewardSkip: ActionSpec
    RelicRewardSelect: ActionSpec
    RelicRewardSkip: ActionSpec
    RestSiteRest: ActionSpec
    RestSiteCardUpgrade: ActionSpec

    def __getattr__(self, name: str) -> ActionSpec: ...
    def __getitem__(self, key: Union[int, str, ActionType]) -> ActionSpec: ...
    def __iter__(self) -> Iterator[ActionSpec]: ...
    def __len__(self) -> int: ...
    def __contains__(self, key: object) -> bool: ...

ACTION_SPECS: ActionSpecs

# ───────── View structs (read-only state snapshots) ─────────

class Modifier:
    kind: ModifierKind
    stacks: int
    """Signed: debuffs and buffs both use this field. For some modifiers
    (Vulnerable, Weak), stacks counts duration in turns."""

    @staticmethod
    def stacks_max_for(kind: ModifierKind) -> int:
        """Per-`ModifierKind` stack ceiling from the engine's
        `MODIFIER_DEFS`. Useful for normalizing stacks before feeding to
        ML encoders. Soft caps (e.g. 999) are common — clamp again on the
        consumer side if a tighter normalization range is wanted."""
        ...

class Relic:
    name: RelicName
    tier: RelicTier
    counter: int
    """Per-relic counter slot. Used by tiered counter-driven relics
    (HappyFlower, Sundial, etc.)."""

    used_up: bool
    """True for one-shot relics that have already triggered (Omamori,
    LizardTail, MawBank). Tier 0 relics never set this."""

class Card:
    name: str
    """Display name. Upgraded cards have a trailing `+`."""

    card_name: CardName
    """Canonical enum slot — stable across upgrades, suitable for one-hot.
    Use this instead of parsing `name` for ML inputs."""

    kind: CardKind
    color: CardColor
    rarity: CardRarity

    cost: int
    """Effective cost right now (post free-to-play, post BulletTime
    override, post dynamic-cost variant). For X-cost cards this is
    `energy.current`."""

    base_cost: int
    """Static base cost (the deck-instance value, before any modifiers).
    Distinct from `cost` for dynamic-cost cards (Eviscerate, Glass Knife,
    X-cost). Use this to recover the un-discounted card cost."""

    cost_kind: Union[
        CardCostKind.Fixed,
        CardCostKind.MinusDiscardsThisTurn,
        CardCostKind.GrowsOnDamageInstanceTaken,
        CardCostKind.XCost,
    ]
    """How `cost` is derived. Lets the agent reason about X-cost vs
    discounted vs growing without inferring it from card identity."""

    upgraded: bool
    exhaust: bool
    ethereal: bool
    innate: bool
    requires_target: bool
    """If True, playing this card requires an `idx_monster` second index
    in `Action(ActionType.CardPlay, [idx_hand, idx_monster])`."""

    retain: bool
    free_to_play_once: bool
    """Per-instance flag set by Setup / Distraction. When True, the next
    play of this card instance ignores energy cost."""

    playable: bool
    """Whether this card can be played given the current game state.
    Combines static play restrictions (e.g. DrawPileEmpty) with relevant
    state. **Energy cost is NOT factored in** — clients should also check
    `card.cost <= energy.current` before offering it as a legal action."""

    effects: list[
        Union[
            Effect.DamagePhysical,
            Effect.DamagePhysicalIfPoisoned,
            Effect.HeelHookProc,
            Effect.EscapePlanCheck,
            Effect.GlassKnifeDecay,
            Effect.CardSetupPick,
            Effect.CardNightmarePick,
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
    character_reward_roll_offset: int
    """Pity offset used when rolling card-reward rarities."""

    gold: int

class Intent:
    """Monster intent for the next turn. `kind` is the categorical tag;
    `block`/`buff`/`debuff` are independent flags that can co-occur."""

    kind: IntentKind

    damage: Optional[int]
    """Damage per instance, post-modifier (Strength/Weak/Vulnerable
    applied). None for non-attack intents."""

    instances: Optional[int]
    """Number of attack instances (e.g. 6×2)."""

    block: bool
    buff: bool
    debuff: bool

class Monster:
    name: str
    """Display name (e.g. "Acid Slime (L)", "Gremlin Nob")."""

    monster_name: MonsterName
    """Canonical enum slot — suitable for one-hot."""

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
    """Valid next-row columns reachable from this node."""

class Map:
    rooms: list[list[Optional[Room]]]
    """Grid indexed `rooms[y][x]`. `y=0` is the bottom row. `None` means
    no node at that coordinate."""

    y_current: Optional[int]
    """Current row. None at run start; equals `MAP_HEIGHT` while in the boss room."""

    x_current: Optional[int]

    boss_name: str
    """Display name of the act-boss this run was seeded with."""

class GameState:
    """Full read-only snapshot of the game state."""

    character: Character
    monsters: list[Monster]
    """Alive monsters only. Indices align with `idx_monster` in
    `Action(ActionType.CardPlay, [idx_hand, idx_monster])`."""

    deck: list[Card]
    hand: list[Card]
    pile_draw: list[Card]
    pile_discard: list[Card]
    pile_exhaust: list[Card]

    card_rewards: list[Card]
    """Cards offered as post-combat reward. Non-empty only during `Phase.CombatReward`."""

    relics: list[Relic]
    """Run-persistent relics owned by the player. Always includes the
    innate SnakeRing for the Silent."""

    relic_rewards: list[Relic]
    """Relics offered as post-combat reward. Non-empty only after Elite
    combats during `Phase.CombatReward`."""

    energy: Energy
    map: Map

    phase: Union[
        Phase.Map,
        Phase.CombatDefault,
        Phase.CombatAwaitDiscard,
        Phase.CombatAwaitRetain,
        Phase.CombatAwaitNightmare,
        Phase.CombatAwaitSetup,
        Phase.CombatReward,
        Phase.RestSite,
        Phase.GameOver,
    ]

# ───────── Environment ─────────

class GameEnv:
    """Game state container with a gymnasium-aligned step interface,
    suitable for interactive play and reinforcement-learning training loops.

    Reward is currently always 0.0 — no reward function is defined yet.
    Truncated is currently always False — no step-limit truncation."""

    # Game-shape constants — class attributes, not instance state.
    # Mirror of `crate::consts` values consumers need at module load.
    # Deliberate omissions: deck / draw / discard pile sizes are
    # *unbounded* in the engine — those caps are encoder concerns.
    MAX_MONSTERS: int
    MAX_SIZE_HAND: int
    MAX_COMBAT_CARD_REWARD: int
    CARDS_DRAWN_PER_TURN: int
    NIGHTMARE_COPIES: int
    MAX_BLOCK: int
    MAP_HEIGHT: int
    MAP_WIDTH: int

    def __init__(self, ascension: int = 0) -> None: ...
    def reset(self, seed: int = 42) -> tuple[GameState, dict]:
        """Start a fresh run. Returns `(obs, info)`."""
        ...

    def step(self, action: Action) -> tuple[GameState, float, bool, bool, dict]:
        """Apply an action. Returns `(obs, reward, terminated, truncated, info)`.
        Raises `ValueError` if the action is malformed (wrong arity) or
        invalid in the current phase."""
        ...

    def dev_grant_relic(self, name: RelicName) -> None:
        """Grant a relic immediately, bypassing the normal reward path.
        For dev/testing only."""
        ...
