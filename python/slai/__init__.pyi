"""Type stubs for the slai native extension.

Hand-maintained. Keep in sync with src/ffi.rs and src/lib.rs when adding
or removing fields / methods / classes / enum variants.

The architecture: `slai` is a Rust+PyO3 extension. Internal Rust enums and
structs live in pure-Rust modules (no PyO3); the boundary at src/ffi.rs
defines #[pyclass] mirrors for everything Python sees, with `From` impls
translating one to the other.
"""

from typing import Optional, Union

# ───────── Unit enums (PyO3 mirrors of internal Rust unit enums) ─────────
#
# Real Python classes (not StrEnum). Instances are hashable singletons that
# support `==`, `hash()`, and integer comparison via `eq_int`.

class CardKind:
    Attack: "CardKind"
    Skill: "CardKind"
    Power: "CardKind"
    Curse: "CardKind"
    Status: "CardKind"

class CardColor:
    Green: "CardColor"
    Colorless: "CardColor"
    Curse: "CardColor"

class CardRarity:
    Basic: "CardRarity"
    Common: "CardRarity"
    Uncommon: "CardRarity"
    Rare: "CardRarity"
    Special: "CardRarity"
    Curse: "CardRarity"

class RoomKind:
    CombatMonster: "RoomKind"
    CombatBoss: "RoomKind"
    RestSite: "RoomKind"

class ModifierKind:
    Accuracy: "ModifierKind"
    AfterImage: "ModifierKind"
    Blur: "ModifierKind"
    Burst: "ModifierKind"
    Dexterity: "ModifierKind"
    DoubleDamage: "ModifierKind"
    InfiniteBlades: "ModifierKind"
    ModeShift: "ModifierKind"
    NextTurnBlock: "ModifierKind"
    NextTurnEnergy: "ModifierKind"
    Phantasmal: "ModifierKind"
    Ritual: "ModifierKind"
    SharpHide: "ModifierKind"
    SporeCloud: "ModifierKind"
    Strength: "ModifierKind"
    ThousandCuts: "ModifierKind"
    Vulnerable: "ModifierKind"
    Weak: "ModifierKind"

class CandidatePool:
    Hand: "CandidatePool"
    CardTarget: "CandidatePool"
    Character: "CandidatePool"
    Monsters: "CandidatePool"
    Source: "CandidatePool"
    NextRowRooms: "CandidatePool"
    CardRewardPool: "CandidatePool"

# ───────── Complex enums (parent class + variant subclasses) ─────────
#
# Each parent class is the type hint for the field; variants are reachable
# as `Parent.Variant` and behave like proper subclasses for `isinstance`
# and structural pattern matching.

class Phase:
    """Game phase. The `CombatAwaitDiscard` variant carries the number of
    cards the player must discard."""

    class Map:
        def __init__(self) -> None: ...

    class CombatDefault:
        def __init__(self) -> None: ...

    class CombatAwaitDiscard:
        num: int
        def __init__(self, num: int) -> None: ...

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

    class BlockGain:
        amount: int
        target: Optional[Target]

    class ModifierGain:
        kind: ModifierKind
        stacks: int
        target: Optional[Target]

    class ModifierRemove:
        kind: ModifierKind
        target: Optional[Target]

    class EnergyGain:
        amount: int
        target: Optional[Target]

    class ShivAdd:
        count: int
        target: Optional[Target]

    class CardDraw:
        count: int
        target: Optional[Target]

    class CardDiscard:
        target: Optional[Target]

    class CalculatedGamble:
        target: Optional[Target]

class Action:
    """A player action passed to `GameEnv.step()`. Construct variants as
    `Action.CardPlay(idx_hand=0, idx_monster=2)` etc."""

    class CardPlay:
        idx_hand: int
        idx_monster: Optional[int]
        def __init__(self, idx_hand: int, idx_monster: Optional[int] = None) -> None: ...

    class EndTurn:
        def __init__(self) -> None: ...

    class CardDiscard:
        """Resolve a pending input-selection prompt (e.g. discard-a-card).
        `indices_hand` length must match the count the prompt requires.
        Valid only in `Phase.CombatAwaitDiscard`."""
        indices_hand: list[int]
        def __init__(self, indices_hand: list[int]) -> None: ...

    class RoomSelect:
        idx_column: int
        def __init__(self, idx_column: int) -> None: ...

    class CardRewardSelect:
        idx_reward: int
        def __init__(self, idx_reward: int) -> None: ...

    class CardRewardSkip:
        def __init__(self) -> None: ...

    class RestSiteRest:
        def __init__(self) -> None: ...

    class RestSiteCardUpgrade:
        idx_deck: int
        def __init__(self, idx_deck: int) -> None: ...

# ───────── View structs (read-only state snapshots) ─────────

class Modifier:
    kind: ModifierKind
    stacks: int
    """Signed: debuffs and buffs both use this field. For some modifiers
    (Vulnerable, Weak), stacks counts duration in turns."""

class Card:
    name: str
    """Display name. Upgraded cards have a trailing `+`."""

    kind: CardKind
    color: CardColor
    rarity: CardRarity
    cost: int
    upgraded: bool
    innate: bool
    exhaust: bool
    requires_target: bool
    """If True, playing this card requires `idx_monster` in `Action.CardPlay`."""

    effects: list[
        Union[
            Effect.DamagePhysical,
            Effect.BlockGain,
            Effect.ModifierGain,
            Effect.ModifierRemove,
            Effect.EnergyGain,
            Effect.ShivAdd,
            Effect.CardDraw,
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

class Intent:
    """Monster intent for the next turn. Multiple flags can co-occur
    (e.g. AttackBuff: `damage` set AND `buff` True)."""

    damage: Optional[int]
    """Damage per instance, post-modifier (Strength/Weak/Vulnerable applied).
    None for non-attack intents."""

    instances: Optional[int]
    """Number of attack instances (e.g. 6×2)."""

    block: bool
    buff: bool
    debuff: bool

class Monster:
    name: str
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

class GameState:
    """Full read-only snapshot of the game state."""

    character: Character
    monsters: list[Monster]
    """Alive monsters only. Indices align with `idx_monster` in `Action.CardPlay`."""

    deck: list[Card]
    hand: list[Card]
    pile_draw: list[Card]
    pile_discard: list[Card]
    pile_exhaust: list[Card]
    card_rewards: list[Card]
    """Cards offered as post-combat reward. Non-empty only during `Phase.CombatReward`."""

    energy: Energy
    map: Map

    phase: Union[
        Phase.Map,
        Phase.CombatDefault,
        Phase.CombatAwaitDiscard,
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

    def __init__(self, ascension: int = 0) -> None: ...

    def reset(
        self, seed: int = 42
    ) -> tuple[
        GameState,
        dict,
    ]:
        """Start a fresh run. Returns `(obs, info)`."""
        ...

    def step(
        self,
        action: Union[
            Action.CardPlay,
            Action.EndTurn,
            Action.CardDiscard,
            Action.RoomSelect,
            Action.CardRewardSelect,
            Action.CardRewardSkip,
            Action.RestSiteRest,
            Action.RestSiteCardUpgrade,
        ],
    ) -> tuple[GameState, float, bool, bool, dict]:
        """Apply an action. Returns `(obs, reward, terminated, truncated, info)`.
        Raises `ValueError` if the action is invalid in the current phase."""
        ...
