from typing import Iterator, NamedTuple, Optional, Union

from . import slai as _rs


def members(rust_enum: type) -> list:
    """Members of a raw pyo3 unit enum, in declaration (dir) order. Raw pyo3 enums
    are not iterable, so this is the iteration mechanism for building index tables."""
    return [
        getattr(rust_enum, k)
        for k in dir(rust_enum)
        if not k.startswith("_") and isinstance(getattr(rust_enum, k), rust_enum)
    ]


# Every enum is re-exported raw — exactly what the FFI hands back, so annotations /
# isinstance / match agree with runtime values. Iterate with members(). ActionType is
# an input type (Python constructs actions); its Rust `.name` feeds the registry below.
ActionType = _rs.ActionType
CardKind = _rs.CardKind
CardColor = _rs.CardColor
CardRarity = _rs.CardRarity
PlayRestriction = _rs.PlayRestriction
RoomKind = _rs.RoomKind
RelicTier = _rs.RelicTier
CardName = _rs.CardName
MonsterName = _rs.MonsterName
MonsterEncounter = _rs.MonsterEncounter
RelicName = _rs.RelicName
PotionName = _rs.PotionName
PotionRarity = _rs.PotionRarity
ModifierKind = _rs.ModifierKind
IntentKind = _rs.IntentKind
Screen = _rs.Screen
EventName = _rs.EventName
CandidatePoolMonstersFilter = _rs.CandidatePoolMonstersFilter
CandidatePoolDeckFilter = _rs.CandidatePoolDeckFilter


# Action schema types
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
    def __init__(self, specs: list[ActionSpec]) -> None:
        self._list: list[ActionSpec] = specs
        self._by_name: dict[str, ActionSpec] = {s.name: s for s in specs}
        self._by_id: dict[ActionType, ActionSpec] = {s.id: s for s in specs}

    def __getattr__(self, name: str) -> ActionSpec:
        try:
            return self._by_name[name]
        except KeyError:
            raise AttributeError(name) from None

    def __getitem__(self, key: Union[ActionType, str]) -> ActionSpec:
        if isinstance(key, str):
            return self._by_name[key]
        return self._by_id[key]

    def __iter__(self) -> Iterator[ActionSpec]:
        return iter(self._list)

    def __len__(self) -> int:
        return len(self._list)

    def __contains__(self, key: object) -> bool:
        return key in self._by_name or key in self._by_id


def _arity_from_args(args: tuple[ArgSpec, ...]) -> tuple[int, Optional[int]]:
    if not args:
        return (0, 0)
    last = args[-1]
    if last.variable:
        return (len(args) - 1, None)
    min_len = sum(1 for s in args if not s.optional)
    return (min_len, len(args))


def create_action_spec(action_type: ActionType, *args: ArgSpec) -> ActionSpec:
    return ActionSpec(
        id=action_type, name=action_type.name, args=args, arity=_arity_from_args(args)
    )


# Per-slot description strings
_HAND_POS = "position in state.hand (the current hand)"
_MONSTER_POS = "position in the alive-monster list at dispatch time"
_REWARD_POS = "slot in state.rewards_card / state.rewards_relic"
_DECK_POS = "position in state.deck (the full deck)"
_MAP_COL = "column on the next map row (0..MAP_WIDTH)"
_SLOT_POS = "slot in state.potions"
_DISCOVER_POS = "position in state.picks_card (the discovery offer)"
_SHOP_CARD_POS = "position in state.shop.cards"
_SHOP_RELIC_POS = "position in state.shop.relics"
_SHOP_POTION_POS = "position in state.shop.potions"


# Action spec registry
ACTION_SPEC_REGISTRY = ActionSpecRegistry(
    [
        create_action_spec(ActionType.CardDiscover, ArgSpec("idx", _DISCOVER_POS)),
        create_action_spec(
            ActionType.CardPlay,
            ArgSpec("idx_card", _HAND_POS),
            ArgSpec("idx_monster", _MONSTER_POS, optional=True),
        ),
        create_action_spec(ActionType.ChestOpen),
        # Deck-pick family (resolves a deck-pick halt)
        create_action_spec(ActionType.CardDuplicate, ArgSpec("idx", _DECK_POS)),
        create_action_spec(ActionType.CardPurge, ArgSpec("idx", _DECK_POS)),
        create_action_spec(ActionType.CardTransform, ArgSpec("idx", _DECK_POS)),
        create_action_spec(ActionType.CardUpgrade, ArgSpec("idx", _DECK_POS)),
        create_action_spec(ActionType.EventOptionSelect, ArgSpec("idx", _HAND_POS)),
        # Hand-pick family (resolves a hand-pick halt)
        create_action_spec(
            ActionType.CardDiscard, ArgSpec("idx_hand", _HAND_POS, variable=True)
        ),
        create_action_spec(ActionType.CardNightmare, ArgSpec("idx_hand", _HAND_POS)),
        create_action_spec(
            ActionType.CardRetain, ArgSpec("idx_hand", _HAND_POS, variable=True)
        ),
        create_action_spec(ActionType.CardSetup, ArgSpec("idx_hand", _HAND_POS)),
        create_action_spec(ActionType.PotionDiscard, ArgSpec("idx_slot", _SLOT_POS)),
        create_action_spec(
            ActionType.PotionUse,
            ArgSpec("idx_potion", _SLOT_POS),
            ArgSpec("idx_monster", _MONSTER_POS, optional=True),
        ),
        create_action_spec(ActionType.Rest),
        # Reward pickup family
        create_action_spec(ActionType.RewardTakeCard, ArgSpec("idx", _REWARD_POS)),
        create_action_spec(ActionType.RewardTakeGold),
        create_action_spec(ActionType.RewardTakePotion),
        create_action_spec(ActionType.RewardTakeRelic),
        create_action_spec(ActionType.RoomSelect, ArgSpec("idx", _MAP_COL)),
        create_action_spec(ActionType.RoomExit),
        # Shop
        create_action_spec(ActionType.ShopBuyCard, ArgSpec("idx", _SHOP_CARD_POS)),
        create_action_spec(ActionType.ShopBuyRelic, ArgSpec("idx", _SHOP_RELIC_POS)),
        create_action_spec(ActionType.ShopBuyPotion, ArgSpec("idx", _SHOP_POTION_POS)),
        create_action_spec(ActionType.ShopPurge, ArgSpec("idx", _DECK_POS)),
        create_action_spec(ActionType.TurnEnd),
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
Reward = _rs.Reward
Shop = _rs.Shop
Potion = _rs.Potion

# Plain struct view
Target = _rs.Target

# Complex enums: re-export the raw pyo3 nested-variant classes directly. Each is a
# real class whose variants are nested subclasses (Effect.DamagePhysical, ...) with
# field properties + __match_args__ for native isinstance/match — exactly what the
# FFI returns, so annotations match runtime values.
Effect = _rs.Effect
CandidatePool = _rs.CandidatePool
SelectionKind = _rs.SelectionKind
CardCostKind = _rs.CardCostKind
HealthDeltaAmount = _rs.HealthDeltaAmount
GoldDeltaKind = _rs.GoldDeltaKind

DeltaSign = _rs.DeltaSign


__all__ = [
    # Environment + action
    "GameEnv",
    "Action",
    "ActionType",
    "ArgSpec",
    "ActionSpec",
    "ActionSpecRegistry",
    "ACTION_SPEC_REGISTRY",
    "members",
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
    # Unit enums (raw)
    "CardKind",
    "CardColor",
    "CardRarity",
    "PlayRestriction",
    "ModifierKind",
    "IntentKind",
    "CandidatePoolMonstersFilter",
    "RoomKind",
    "RelicName",
    "RelicTier",
    "PotionName",
    "PotionRarity",
    "CardName",
    "MonsterName",
    "MonsterEncounter",
    "EventName",
    "CandidatePoolDeckFilter",
    "Screen",
    # Complex enums (raw nested-variant classes)
    "CandidatePool",
    "SelectionKind",
    "CardCostKind",
    "Target",
    "Effect",
    "HealthDeltaAmount",
    "GoldDeltaKind",
    "DeltaSign",
    # Reward
    "Reward",
    # Shop
    "Shop",
    # Potion
    "Potion",
]
