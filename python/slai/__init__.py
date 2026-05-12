"""slai — Slay-the-Spire-like engine with a Python binding.

The PyO3 compiled extension lives at `slai.slai` and exposes raw PyO3
classes. This wrapper:

  1. Re-exports the views, complex enums, GameEnv, and Action from the
     compiled extension.
  2. Synthesizes real `enum.IntEnum` types for every unit-only PyO3 enum
     (`CardKind`, `CardColor`, ..., `ActionType`) so users get `.name`,
     `.value`, iteration, and `isinstance(x, IntEnum)`. PySC2's
     `_Functions = enum.IntEnum("_Functions", {...})` idiom, adapted to
     read the variant table from the compiled extension.
  3. Builds the `ACTION_SPECS` registry: bare `ActionType` for identity,
     `ArgSpec` / `ActionSpec` NamedTuples for the per-action schema, an
     `ActionSpecs` lookup class. Mirrors PySC2's `Functions` / `Function`
     / `ArgumentType` / `FUNCTIONS` separation in `pysc2/lib/actions.py`.
"""

from enum import IntEnum
from typing import Iterator, NamedTuple, Optional, Union

from . import slai as _rs


# ───────── IntEnum shim for unit-only PyO3 enums ─────────


def _to_intenum(name: str, rs_cls: type) -> type:
    """Synthesize a real enum.IntEnum from a PyO3 unit-only enum class.

    Reads each public class attribute that is an instance of `rs_cls` and
    builds an IntEnum with the same name->int mapping.
    """
    members = {
        k: int(getattr(rs_cls, k))
        for k in dir(rs_cls)
        if not k.startswith("_") and isinstance(getattr(rs_cls, k), rs_cls)
    }
    return IntEnum(name, members)


ActionType = _to_intenum("ActionType", _rs.ActionType)
CardKind = _to_intenum("CardKind", _rs.CardKind)
CardColor = _to_intenum("CardColor", _rs.CardColor)
CardRarity = _to_intenum("CardRarity", _rs.CardRarity)
RoomKind = _to_intenum("RoomKind", _rs.RoomKind)
RelicTier = _to_intenum("RelicTier", _rs.RelicTier)
CardName = _to_intenum("CardName", _rs.CardName)
MonsterName = _to_intenum("MonsterName", _rs.MonsterName)
RelicName = _to_intenum("RelicName", _rs.RelicName)
ModifierKind = _to_intenum("ModifierKind", _rs.ModifierKind)
IntentKind = _to_intenum("IntentKind", _rs.IntentKind)
CandidatePool = _to_intenum("CandidatePool", _rs.CandidatePool)


# ───────── Action schema registry (PySC2 Functions / Function pattern) ─────────


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

    `ActionType` carries identity; this namedtuple carries the data.
    """

    id: ActionType  # type: ignore[valid-type]
    name: str
    args: tuple[ArgSpec, ...]
    arity: tuple[int, Optional[int]]


class ActionSpecs:
    """Registry of action type specs. Mirrors `pysc2.lib.actions.Functions`.

    Supports lookup by attribute (name), by int discriminant, by
    `ActionType` member (IntEnum is int), and iteration.
    """

    def __init__(self, specs: list[ActionSpec]) -> None:
        self._by_name: dict[str, ActionSpec] = {s.name: s for s in specs}
        self._by_id: dict[int, ActionSpec] = {int(s.id): s for s in specs}
        self._list: list[ActionSpec] = list(specs)

    def __getattr__(self, name: str) -> ActionSpec:
        try:
            return self._by_name[name]
        except KeyError:
            raise AttributeError(name) from None

    def __getitem__(self, key: Union[int, str]) -> ActionSpec:
        if isinstance(key, int):  # also catches IntEnum members (IntEnum is int)
            return self._by_id[int(key)]
        return self._by_name[key]

    def __iter__(self) -> Iterator[ActionSpec]:
        return iter(self._list)

    def __len__(self) -> int:
        return len(self._list)

    def __contains__(self, key: object) -> bool:
        if isinstance(key, int):
            return int(key) in self._by_id
        if isinstance(key, str):
            return key in self._by_name
        return False


def _arity_from_args(args: tuple[ArgSpec, ...]) -> tuple[int, Optional[int]]:
    if not args:
        return (0, 0)
    last = args[-1]
    if last.variable:
        return (len(args) - 1, None)
    min_len = sum(1 for s in args if not s.optional)
    return (min_len, len(args))


def _spec(at: ActionType, *args: ArgSpec) -> ActionSpec:  # type: ignore[valid-type]
    return ActionSpec(id=at, name=at.name, args=args, arity=_arity_from_args(args))


_HAND_POS = "position in state.hand (the current hand)"
_MONSTER_POS = "position in the alive-monster list at dispatch time"
_REWARD_POS = "slot in state.card_rewards / state.relic_rewards"
_DECK_POS = "position in state.deck (the full deck)"
_MAP_COL = "column on the next map row (0..MAP_WIDTH)"


ACTION_SPECS = ActionSpecs(
    [
        _spec(
            ActionType.CardPlay,
            ArgSpec("idx_hand", _HAND_POS),
            ArgSpec("idx_monster", _MONSTER_POS, optional=True),
        ),
        _spec(ActionType.EndTurn),
        _spec(ActionType.CardDiscard, ArgSpec("idx_hand", _HAND_POS, variable=True)),
        _spec(ActionType.CardRetain, ArgSpec("idx_hand", _HAND_POS, variable=True)),
        _spec(ActionType.CardSetup, ArgSpec("idx_hand", _HAND_POS)),
        _spec(ActionType.CardNightmare, ArgSpec("idx_hand", _HAND_POS)),
        _spec(ActionType.RoomSelect, ArgSpec("idx_column", _MAP_COL)),
        _spec(ActionType.CardRewardSelect, ArgSpec("idx_reward", _REWARD_POS)),
        _spec(ActionType.CardRewardSkip),
        _spec(ActionType.RelicRewardSelect, ArgSpec("idx_reward", _REWARD_POS)),
        _spec(ActionType.RelicRewardSkip),
        _spec(ActionType.RestSiteRest),
        _spec(ActionType.RestSiteCardUpgrade, ArgSpec("idx_deck", _DECK_POS)),
        _spec(ActionType.RoomSkip),
    ]
)


# ───────── Re-exports from the compiled extension ─────────


GameEnv = _rs.GameEnv
Action = _rs.Action

# Views
Card = _rs.Card
Character = _rs.Character
Energy = _rs.Energy
GameState = _rs.GameState
Intent = _rs.Intent
Map = _rs.Map
Room = _rs.Room
Modifier = _rs.Modifier
Monster = _rs.Monster
Relic = _rs.Relic

# Complex (data-bearing) enums — kept as PyO3 #[pyclass] complex enums
Phase = _rs.Phase
Selection = _rs.Selection
Target = _rs.Target
Effect = _rs.Effect
CardCostKind = _rs.CardCostKind


__all__ = [
    # Environment & action
    "GameEnv",
    "Action",
    "ActionType",
    "ArgSpec",
    "ActionSpec",
    "ActionSpecs",
    "ACTION_SPECS",
    # Views
    "Card",
    "Character",
    "Energy",
    "GameState",
    "Intent",
    "Map",
    "Room",
    "Modifier",
    "Monster",
    "Relic",
    # Unit-enum IntEnum shims
    "CardKind",
    "CardColor",
    "CardRarity",
    "CardCostKind",
    "ModifierKind",
    "IntentKind",
    "CandidatePool",
    "RoomKind",
    "RelicName",
    "RelicTier",
    "CardName",
    "MonsterName",
    # Complex enums
    "Phase",
    "Selection",
    "Target",
    "Effect",
]
