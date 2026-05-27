from enum import IntEnum
from typing import Iterator, NamedTuple, Optional, Union

from . import slai as _rs


# IntEnum shims for unit-only PyO3 enums
def _to_intenum(name: str, rust_cls: type) -> type:
    members = {
        k: int(getattr(rust_cls, k))
        for k in dir(rust_cls)
        if not k.startswith("_") and isinstance(getattr(rust_cls, k), rust_cls)
    }
    return IntEnum(name, members)


ActionType = _to_intenum("ActionType", _rs.ActionType)
CardKind = _to_intenum("CardKind", _rs.CardKind)
CardColor = _to_intenum("CardColor", _rs.CardColor)
CardRarity = _to_intenum("CardRarity", _rs.CardRarity)
RoomKind = _to_intenum("RoomKind", _rs.RoomKind)
ChestKind = _to_intenum("ChestKind", _rs.ChestKind)
RelicTier = _to_intenum("RelicTier", _rs.RelicTier)
CardName = _to_intenum("CardName", _rs.CardName)
MonsterName = _to_intenum("MonsterName", _rs.MonsterName)
RelicName = _to_intenum("RelicName", _rs.RelicName)
PotionName = _to_intenum("PotionName", _rs.PotionName)
PotionRarity = _to_intenum("PotionRarity", _rs.PotionRarity)
ModifierKind = _to_intenum("ModifierKind", _rs.ModifierKind)
IntentKind = _to_intenum("IntentKind", _rs.IntentKind)
CandidatePool = _to_intenum("CandidatePool", _rs.CandidatePool)
CandidatePoolMonstersFilter = _to_intenum("CandidatePoolMonstersFilter", _rs.CandidatePoolMonstersFilter)
EventName = _to_intenum("EventName", _rs.EventName)
CandidatePoolDeckFilter = _to_intenum("CandidatePoolDeckFilter", _rs.CandidatePoolDeckFilter)


# Action schema types
class ArgSpec(NamedTuple):
    name: str
    description: str
    optional: bool = False
    variable: bool = False


class ActionSpec(NamedTuple):
    id: ActionType  # type: ignore[valid-type]
    name: str
    args: tuple[ArgSpec, ...]
    arity: tuple[int, Optional[int]]


class ActionSpecRegistry:
    def __init__(self, specs: list[ActionSpec]) -> None:
        self._list: list[ActionSpec] = specs
        self._by_name: dict[str, ActionSpec] = {s.name: s for s in specs}
        self._by_id: dict[int, ActionSpec] = {int(s.id): s for s in specs}

    def __getattr__(self, name: str) -> ActionSpec:
        try:
            return self._by_name[name]
        except KeyError:
            raise AttributeError(name) from None

    def __getitem__(self, key: Union[int, str]) -> ActionSpec:
        if isinstance(key, int):
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


def create_action_spec(action_type: ActionType, *args: ArgSpec) -> ActionSpec:  # type: ignore[valid-type]
    return ActionSpec(id=action_type, name=action_type.name, args=args, arity=_arity_from_args(args))


# Per-slot description strings
_HAND_POS = "position in state.hand (the current hand)"
_MONSTER_POS = "position in the alive-monster list at dispatch time"
_REWARD_POS = "slot in state.rewards_card / state.rewards_relic"
_DECK_POS = "position in state.deck (the full deck)"
_MAP_COL = "column on the next map row (0..MAP_WIDTH)"
_SLOT_POS = "slot in state.character.potion_slots"
_DISCOVER_POS = "position in state.picks_card (the discovery offer)"


# Action spec registry
ACTION_SPEC_REGISTRY = ActionSpecRegistry(
    [
        create_action_spec(
            ActionType.CardPlay,
            ArgSpec("idx_hand", _HAND_POS),
            ArgSpec("idx_monster", _MONSTER_POS, optional=True),
        ),
        create_action_spec(ActionType.EndTurn),
        create_action_spec(ActionType.HandSelect, ArgSpec("idx_hand", _HAND_POS, variable=True)),
        create_action_spec(ActionType.RoomSelect, ArgSpec("idx_column", _MAP_COL)),
        create_action_spec(ActionType.RestSiteRest),
        create_action_spec(ActionType.RestSiteCardUpgrade, ArgSpec("idx_deck", _DECK_POS)),
        create_action_spec(ActionType.RoomSkip),
        create_action_spec(ActionType.ChestOpen),
        create_action_spec(
            ActionType.PotionUse,
            ArgSpec("idx_slot", _SLOT_POS),
            ArgSpec("idx_monster", _MONSTER_POS, optional=True),
        ),
        create_action_spec(ActionType.PotionDiscard, ArgSpec("idx_slot", _SLOT_POS)),
        create_action_spec(
            ActionType.CardDiscover, ArgSpec("idx_option", _DISCOVER_POS)
        ),
        # Reward pickup family
        create_action_spec(ActionType.RewardTakeCard, ArgSpec("idx_reward", _REWARD_POS)),
        create_action_spec(ActionType.RewardTakeRelic),
        create_action_spec(ActionType.RewardTakePotion),
        create_action_spec(ActionType.RewardTakeGold),
        create_action_spec(ActionType.RewardSkip),
    ]
)


# Environment + action
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
Event = _rs.Event
EventOption = _rs.EventOption

# Complex enums
SelectionKind = _rs.SelectionKind
Target = _rs.Target
Effect = _rs.Effect
CardCostKind = _rs.CardCostKind

# Reward + PendingInput surface
Screen = _to_intenum("Screen", _rs.Screen)
Reward = _rs.Reward
PendingInput = _rs.PendingInput

__all__ = [
    # Environment + action
    "GameEnv",
    "Action",
    "ActionType",
    "ArgSpec",
    "ActionSpec",
    "ActionSpecRegistry",
    "ACTION_SPEC_REGISTRY",
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
    "Event",
    "EventOption",
    # Unit-enum shims
    "CardKind",
    "CardColor",
    "CardRarity",
    "CardCostKind",
    "ModifierKind",
    "IntentKind",
    "CandidatePool",
    "CandidatePoolMonstersFilter",
    "RoomKind",
    "ChestKind",
    "RelicName",
    "RelicTier",
    "PotionName",
    "PotionRarity",
    "CardName",
    "MonsterName",
    "EventName",
    "CandidatePoolDeckFilter",
    "Screen",
    # Complex enums
    "SelectionKind",
    "Target",
    "Effect",
    # Reward + PendingInput
    "Reward",
    "PendingInput",
]
