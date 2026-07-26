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
CandidatePoolMonstersFilter = _rs.CandidatePoolMonstersFilter
CandidatePoolCardFilter = _rs.CandidatePoolCardFilter


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
_HAND_POS = "position in state.mode.hand (the current hand)"
_MONSTER_POS = "position in the alive-monster list at dispatch time"
_REWARD_POS = "slot in state.mode.cards"
_DECK_POS = "position in state.deck (the full deck)"
_MAP_COL = "column on the next map row (0..MAP_WIDTH)"
_SLOT_POS = "slot in state.potions"
_REWARD_POTION_POS = "slot in state.mode.potions"
_DISCOVER_POS = "position in state.mode.discover (the discovery offer)"
_SHOP_CARD_POS = "position in state.mode.cards"
_SHOP_RELIC_POS = "position in state.mode.relics"
_SHOP_POTION_POS = "position in state.mode.potions"


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
        create_action_spec(
            ActionType.EventOptionSelect, ArgSpec("idx", "position in state.mode.options")
        ),
        # Hand-pick family (resolves a hand-pick halt)
        create_action_spec(ActionType.CardDiscard, ArgSpec("idx_hand", _HAND_POS)),
        create_action_spec(ActionType.CardExhaust, ArgSpec("idx_hand", _HAND_POS)),
        create_action_spec(
            ActionType.CardMoveToHand, ArgSpec("idx", "position in state.pile_draw")
        ),
        create_action_spec(ActionType.PickSkip),
        create_action_spec(ActionType.CardNightmare, ArgSpec("idx_hand", _HAND_POS)),
        create_action_spec(ActionType.CardRetain, ArgSpec("idx_hand", _HAND_POS)),
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
        create_action_spec(
            ActionType.RewardTakePotion, ArgSpec("idx", _REWARD_POTION_POS)
        ),
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
Potion = _rs.Potion

# Plain struct view
Target = _rs.Target

# Sum types: one flat frozen class per variant (EffectDamagePhysical, ...) plus a
# PEP 604 union alias per family. The union works as annotation, isinstance target,
# and typing.get_args source alike; the FFI returns instances of the flat classes.
EffectDamagePhysical = _rs.EffectDamagePhysical
EffectDamagePhysicalIfPoisoned = _rs.EffectDamagePhysicalIfPoisoned
EffectHeelHookProc = _rs.EffectHeelHookProc
EffectEscapePlanCheck = _rs.EffectEscapePlanCheck
EffectGlassKnifeDecay = _rs.EffectGlassKnifeDecay
EffectCardSetupPick = _rs.EffectCardSetupPick
EffectCardNightmarePick = _rs.EffectCardNightmarePick
EffectDistractionAdd = _rs.EffectDistractionAdd
EffectSetCostOverride = _rs.EffectSetCostOverride
EffectDamageFinisher = _rs.EffectDamageFinisher
EffectDamageFlechettes = _rs.EffectDamageFlechettes
EffectUnloadDiscard = _rs.EffectUnloadDiscard
EffectStormOfSteelProc = _rs.EffectStormOfSteelProc
EffectSneakyStrikeProc = _rs.EffectSneakyStrikeProc
EffectBlockGain = _rs.EffectBlockGain
EffectModifierGain = _rs.EffectModifierGain
EffectModifierMultiply = _rs.EffectModifierMultiply
EffectModifierRemove = _rs.EffectModifierRemove
EffectEnergyGain = _rs.EffectEnergyGain
EffectCardAddToHand = _rs.EffectCardAddToHand
EffectCardDraw = _rs.EffectCardDraw
EffectCardDrawUpTo = _rs.EffectCardDrawUpTo
EffectCardDiscard = _rs.EffectCardDiscard
EffectCardRetain = _rs.EffectCardRetain
EffectDamageMindBlast = _rs.EffectDamageMindBlast
EffectShuffleDiscardPileIntoDrawPile = _rs.EffectShuffleDiscardPileIntoDrawPile
EffectCalculatedGamble = _rs.EffectCalculatedGamble
EffectMaxHealthDelta = _rs.EffectMaxHealthDelta
EffectHealthDelta = _rs.EffectHealthDelta
EffectPotionAddRandom = _rs.EffectPotionAddRandom
EffectPotionDiscard = _rs.EffectPotionDiscard
EffectRewardRollPotions = _rs.EffectRewardRollPotions
EffectCardDiscoverRoll = _rs.EffectCardDiscoverRoll
EffectGoldDelta = _rs.EffectGoldDelta
EffectRelicGrantRandom = _rs.EffectRelicGrantRandom
EffectWheelSpin = _rs.EffectWheelSpin
EffectBonfireOffer = _rs.EffectBonfireOffer
EffectFaceTrade = _rs.EffectFaceTrade
EffectMonsterSpawn = _rs.EffectMonsterSpawn
EffectCombatStart = _rs.EffectCombatStart
EffectAdventurerSearch = _rs.EffectAdventurerSearch
EffectRelicGrantSpecific = _rs.EffectRelicGrantSpecific
EffectEventAdvanceState = _rs.EffectEventAdvanceState
EffectScrapOozeReach = _rs.EffectScrapOozeReach
EffectEventConsume = _rs.EffectEventConsume
EffectCardDiscoverPick = _rs.EffectCardDiscoverPick
EffectCardAddToDeck = _rs.EffectCardAddToDeck
EffectCardPurge = _rs.EffectCardPurge
EffectCardUpgrade = _rs.EffectCardUpgrade
EffectCardDuplicate = _rs.EffectCardDuplicate
EffectCardTransform = _rs.EffectCardTransform
Effect = (
    EffectDamagePhysical
    | EffectDamagePhysicalIfPoisoned
    | EffectHeelHookProc
    | EffectEscapePlanCheck
    | EffectGlassKnifeDecay
    | EffectCardSetupPick
    | EffectCardNightmarePick
    | EffectDistractionAdd
    | EffectSetCostOverride
    | EffectDamageFinisher
    | EffectDamageFlechettes
    | EffectUnloadDiscard
    | EffectStormOfSteelProc
    | EffectSneakyStrikeProc
    | EffectBlockGain
    | EffectModifierGain
    | EffectModifierMultiply
    | EffectModifierRemove
    | EffectEnergyGain
    | EffectCardAddToHand
    | EffectCardDraw
    | EffectCardDrawUpTo
    | EffectCardDiscard
    | EffectCardRetain
    | EffectDamageMindBlast
    | EffectShuffleDiscardPileIntoDrawPile
    | EffectCalculatedGamble
    | EffectMaxHealthDelta
    | EffectHealthDelta
    | EffectPotionAddRandom
    | EffectPotionDiscard
    | EffectRewardRollPotions
    | EffectCardDiscoverRoll
    | EffectGoldDelta
    | EffectRelicGrantRandom
    | EffectWheelSpin
    | EffectBonfireOffer
    | EffectFaceTrade
    | EffectMonsterSpawn
    | EffectCombatStart
    | EffectAdventurerSearch
    | EffectRelicGrantSpecific
    | EffectEventAdvanceState
    | EffectScrapOozeReach
    | EffectEventConsume
    | EffectCardDiscoverPick
    | EffectCardAddToDeck
    | EffectCardPurge
    | EffectCardUpgrade
    | EffectCardDuplicate
    | EffectCardTransform
)
CandidatePoolHand = _rs.CandidatePoolHand
CandidatePoolCharacter = _rs.CandidatePoolCharacter
CandidatePoolMonsters = _rs.CandidatePoolMonsters
CandidatePoolSource = _rs.CandidatePoolSource
CandidatePoolDiscover = _rs.CandidatePoolDiscover
CandidatePoolDeck = _rs.CandidatePoolDeck
CandidatePoolEventPickCard = _rs.CandidatePoolEventPickCard
CandidatePoolEventPickPotion = _rs.CandidatePoolEventPickPotion
CandidatePool = (
    CandidatePoolHand
    | CandidatePoolCharacter
    | CandidatePoolMonsters
    | CandidatePoolSource
    | CandidatePoolDiscover
    | CandidatePoolDeck
    | CandidatePoolEventPickCard
    | CandidatePoolEventPickPotion
)
SelectionKindAll = _rs.SelectionKindAll
SelectionKindSingle = _rs.SelectionKindSingle
SelectionKindRandom = _rs.SelectionKindRandom
SelectionKindInput = _rs.SelectionKindInput
SelectionKind = (
    SelectionKindAll | SelectionKindSingle | SelectionKindRandom | SelectionKindInput
)

# Flat variant classes + PEP 604 union aliases. The union works as annotation,
# isinstance target, and typing.get_args source alike
CardCostKindFixed = _rs.CardCostKindFixed
CardCostKindMinusDiscardsThisTurn = _rs.CardCostKindMinusDiscardsThisTurn
CardCostKindGrowsOnDamageInstanceTaken = _rs.CardCostKindGrowsOnDamageInstanceTaken
CardCostKindXCost = _rs.CardCostKindXCost
CardCostKind = (
    CardCostKindFixed
    | CardCostKindMinusDiscardsThisTurn
    | CardCostKindGrowsOnDamageInstanceTaken
    | CardCostKindXCost
)

AmountAbsolute = _rs.AmountAbsolute
AmountRelative = _rs.AmountRelative
AmountRange = _rs.AmountRange
AmountEventGoldAsk = _rs.AmountEventGoldAsk
Amount = AmountAbsolute | AmountRelative | AmountRange | AmountEventGoldAsk
ModeMap = _rs.ModeMap
ModeRestSite = _rs.ModeRestSite
ModeChest = _rs.ModeChest
ModeChestOpened = _rs.ModeChestOpened
ModeCombatEnded = _rs.ModeCombatEnded
ModeCombat = _rs.ModeCombat
ModeReward = _rs.ModeReward
ModeShop = _rs.ModeShop
ModeEvent = _rs.ModeEvent
Mode = (
    ModeMap
    | ModeRestSite
    | ModeChest
    | ModeChestOpened
    | ModeCombatEnded
    | ModeCombat
    | ModeReward
    | ModeShop
    | ModeEvent
)
EventKindBigFish = _rs.EventKindBigFish
EventKindTheCleric = _rs.EventKindTheCleric
EventKindDuplicator = _rs.EventKindDuplicator
EventKindGoldenShrine = _rs.EventKindGoldenShrine
EventKindWingStatue = _rs.EventKindWingStatue
EventKindWorldOfGoop = _rs.EventKindWorldOfGoop
EventKindLivingWall = _rs.EventKindLivingWall
EventKindPurifier = _rs.EventKindPurifier
EventKindShiningLight = _rs.EventKindShiningLight
EventKindTheSsssserpent = _rs.EventKindTheSsssserpent
EventKindTransmogrifier = _rs.EventKindTransmogrifier
EventKindUpgradeShrine = _rs.EventKindUpgradeShrine
EventKindTheDivineFountain = _rs.EventKindTheDivineFountain
EventKindTheLab = _rs.EventKindTheLab
EventKindTheWomanInBlue = _rs.EventKindTheWomanInBlue
EventKindWheelOfChange = _rs.EventKindWheelOfChange
EventKindBonfireSpirits = _rs.EventKindBonfireSpirits
EventKindOminousForge = _rs.EventKindOminousForge
EventKindFaceTrader = _rs.EventKindFaceTrader
EventKindMushrooms = _rs.EventKindMushrooms
EventKindGoldenIdol = _rs.EventKindGoldenIdol
EventKindScrapOoze = _rs.EventKindScrapOoze
EventKindWeMeetAgain = _rs.EventKindWeMeetAgain
EventKindDeadAdventurer = _rs.EventKindDeadAdventurer
EventKind = (
    EventKindBigFish
    | EventKindTheCleric
    | EventKindDuplicator
    | EventKindGoldenShrine
    | EventKindWingStatue
    | EventKindWorldOfGoop
    | EventKindLivingWall
    | EventKindPurifier
    | EventKindShiningLight
    | EventKindTheSsssserpent
    | EventKindTransmogrifier
    | EventKindUpgradeShrine
    | EventKindTheDivineFountain
    | EventKindTheLab
    | EventKindTheWomanInBlue
    | EventKindWheelOfChange
    | EventKindBonfireSpirits
    | EventKindOminousForge
    | EventKindFaceTrader
    | EventKindMushrooms
    | EventKindGoldenIdol
    | EventKindScrapOoze
    | EventKindWeMeetAgain
    | EventKindDeadAdventurer
)

DeltaSign = _rs.DeltaSign

# EventKind variants in engine declaration order — the stable event-identity
# index for featurization (variant classes carry no discriminant of their own)
EVENT_KIND_ORDER: tuple[type, ...] = (
    EventKindBigFish,
    EventKindTheCleric,
    EventKindDuplicator,
    EventKindGoldenShrine,
    EventKindWingStatue,
    EventKindWorldOfGoop,
    EventKindLivingWall,
    EventKindPurifier,
    EventKindShiningLight,
    EventKindTheSsssserpent,
    EventKindTransmogrifier,
    EventKindUpgradeShrine,
    EventKindTheDivineFountain,
    EventKindTheLab,
    EventKindTheWomanInBlue,
    EventKindWheelOfChange,
    EventKindBonfireSpirits,
    EventKindOminousForge,
    EventKindFaceTrader,
    EventKindMushrooms,
    EventKindGoldenIdol,
    EventKindScrapOoze,
    EventKindWeMeetAgain,
    EventKindDeadAdventurer,
)


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
    "CandidatePoolCardFilter",
    # Sum types (flat variant classes + union aliases)
    "CandidatePool",
    "CandidatePoolHand",
    "CandidatePoolCharacter",
    "CandidatePoolMonsters",
    "CandidatePoolSource",
    "CandidatePoolDiscover",
    "CandidatePoolDeck",
    "CandidatePoolEventPickCard",
    "CandidatePoolEventPickPotion",
    "SelectionKind",
    "SelectionKindAll",
    "SelectionKindSingle",
    "SelectionKindRandom",
    "SelectionKindInput",
    "CardCostKind",
    "CardCostKindFixed",
    "CardCostKindMinusDiscardsThisTurn",
    "CardCostKindGrowsOnDamageInstanceTaken",
    "CardCostKindXCost",
    "Target",
    "Effect",
    "EffectDamagePhysical",
    "EffectDamagePhysicalIfPoisoned",
    "EffectHeelHookProc",
    "EffectEscapePlanCheck",
    "EffectGlassKnifeDecay",
    "EffectCardSetupPick",
    "EffectCardNightmarePick",
    "EffectDistractionAdd",
    "EffectSetCostOverride",
    "EffectDamageFinisher",
    "EffectDamageFlechettes",
    "EffectUnloadDiscard",
    "EffectStormOfSteelProc",
    "EffectSneakyStrikeProc",
    "EffectBlockGain",
    "EffectModifierGain",
    "EffectModifierMultiply",
    "EffectModifierRemove",
    "EffectEnergyGain",
    "EffectCardAddToHand",
    "EffectCardDraw",
    "EffectCardDrawUpTo",
    "EffectCardDiscard",
    "EffectCardRetain",
    "EffectDamageMindBlast",
    "EffectShuffleDiscardPileIntoDrawPile",
    "EffectCalculatedGamble",
    "EffectMaxHealthDelta",
    "EffectHealthDelta",
    "EffectPotionAddRandom",
    "EffectPotionDiscard",
    "EffectRewardRollPotions",
    "EffectCardDiscoverRoll",
    "EffectGoldDelta",
    "EffectRelicGrantRandom",
    "EffectWheelSpin",
    "EffectBonfireOffer",
    "EffectFaceTrade",
    "EffectMonsterSpawn",
    "EffectCombatStart",
    "EffectAdventurerSearch",
    "EffectRelicGrantSpecific",
    "EffectEventAdvanceState",
    "EffectScrapOozeReach",
    "EffectEventConsume",
    "EffectCardDiscoverPick",
    "EffectCardAddToDeck",
    "EffectCardPurge",
    "EffectCardUpgrade",
    "EffectCardDuplicate",
    "EffectCardTransform",
    "Amount",
    "AmountAbsolute",
    "AmountRelative",
    "AmountRange",
    "AmountEventGoldAsk",
    "Mode",
    "ModeMap",
    "ModeRestSite",
    "ModeChest",
    "ModeChestOpened",
    "ModeCombatEnded",
    "ModeCombat",
    "ModeReward",
    "ModeShop",
    "ModeEvent",
    "EventKind",
    "EventKindBigFish",
    "EventKindTheCleric",
    "EventKindDuplicator",
    "EventKindGoldenShrine",
    "EventKindWingStatue",
    "EventKindWorldOfGoop",
    "EventKindLivingWall",
    "EventKindPurifier",
    "EventKindShiningLight",
    "EventKindTheSsssserpent",
    "EventKindTransmogrifier",
    "EventKindUpgradeShrine",
    "EventKindTheDivineFountain",
    "EventKindTheLab",
    "EventKindTheWomanInBlue",
    "EventKindWheelOfChange",
    "EventKindBonfireSpirits",
    "EventKindOminousForge",
    "EventKindFaceTrader",
    "EventKindMushrooms",
    "EventKindGoldenIdol",
    "EventKindScrapOoze",
    "EventKindWeMeetAgain",
    "EventKindDeadAdventurer",
    "EVENT_KIND_ORDER",
    "DeltaSign",
    # Potion
    "Potion",
]
