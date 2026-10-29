from typing import Iterator, NamedTuple, Optional, Union

from . import slai as _rs

# Every enum is re-exported raw
ActionType = _rs.ActionType
CardKind = _rs.CardKind
CardColor = _rs.CardColor
CardRarity = _rs.CardRarity
CardPile = _rs.CardPile
CostScope = _rs.CostScope
PlayRestriction = _rs.PlayRestriction
PlaySource = _rs.PlaySource
RoomKind = _rs.RoomKind
RelicTier = _rs.RelicTier
RelicExclusion = _rs.RelicExclusion
CardName = _rs.CardName
MonsterName = _rs.MonsterName
MonsterEncounter = _rs.MonsterEncounter
RelicName = _rs.RelicName
PotionName = _rs.PotionName
PotionRarity = _rs.PotionRarity
ModifierKind = _rs.ModifierKind
IntentKind = _rs.IntentKind
CandidateFilter = _rs.CandidateFilter
MonsterKind = _rs.MonsterKind
EventName = _rs.EventName


# Action schema types
class ArgSpec(NamedTuple):
    name: str
    optional: bool = False
    variable: bool = False


class ActionSpec(NamedTuple):
    action_type: ActionType
    name: str
    args: tuple[ArgSpec, ...]
    arity: tuple[int, Optional[int]]


class ActionSpecRegistry:
    def __init__(self, specs: list[ActionSpec]) -> None:
        self._list: list[ActionSpec] = specs
        self._by_name: dict[str, ActionSpec] = {s.name: s for s in specs}
        self._by_action_type: dict[ActionType, ActionSpec] = {
            s.action_type: s for s in specs
        }

    def __getattr__(self, name: str) -> ActionSpec:
        try:
            return self._by_name[name]
        except KeyError:
            raise AttributeError(name) from None

    def __getitem__(self, key: Union[ActionType, str]) -> ActionSpec:
        if isinstance(key, str):
            return self._by_name[key]
        return self._by_action_type[key]

    def __iter__(self) -> Iterator[ActionSpec]:
        return iter(self._list)

    def __len__(self) -> int:
        return len(self._list)

    def __contains__(self, key: object) -> bool:
        return key in self._by_name or key in self._by_action_type


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
        action_type=action_type,
        name=action_type.name,
        args=args,
        arity=_arity_from_args(args),
    )


# Action spec registry
ACTION_SPEC_REGISTRY = ActionSpecRegistry(
    [
        create_action_spec(
            ActionType.CardPlay,
            ArgSpec("id_card"),
            ArgSpec("id_monster", optional=True),
        ),
        create_action_spec(ActionType.ChestOpen),
        # Halting operations: a pending halt's selection, one per operation
        create_action_spec(ActionType.BonfireOffer, ArgSpec("id_card")),
        create_action_spec(ActionType.CardAdopt, ArgSpec("id_card")),
        create_action_spec(ActionType.CardBottle, ArgSpec("id_card")),
        create_action_spec(ActionType.CardDiscard, ArgSpec("id_card")),
        create_action_spec(ActionType.CardDiscoverSelect, ArgSpec("id_card")),
        create_action_spec(ActionType.CardDuplicate, ArgSpec("id_card")),
        create_action_spec(ActionType.CardExhaust, ArgSpec("id_card")),
        create_action_spec(ActionType.CardMove, ArgSpec("id_card")),
        create_action_spec(ActionType.CardNightmareSelect, ArgSpec("id_card")),
        create_action_spec(ActionType.CardPurge, ArgSpec("id_card")),
        create_action_spec(ActionType.CardRetain, ArgSpec("id_card")),
        create_action_spec(ActionType.CardSetupSelect, ArgSpec("id_card")),
        create_action_spec(ActionType.CardTransform, ArgSpec("id_card")),
        create_action_spec(ActionType.CardUpgrade, ArgSpec("id_card")),
        create_action_spec(ActionType.MatchFlipSeen, ArgSpec("id_card")),
        create_action_spec(ActionType.RelicLose, ArgSpec("id_relic")),
        create_action_spec(ActionType.SelectionSkip),
        create_action_spec(ActionType.EventOptionSelect, ArgSpec("id_option")),
        create_action_spec(ActionType.PotionDiscard, ArgSpec("id_potion")),
        create_action_spec(
            ActionType.PotionUse,
            ArgSpec("id_potion"),
            ArgSpec("id_monster", optional=True),
        ),
        create_action_spec(ActionType.Rest),
        create_action_spec(ActionType.RestDig),
        create_action_spec(ActionType.RestLift),
        create_action_spec(ActionType.RestSmith),
        create_action_spec(ActionType.RestToke),
        # Reward pickup family
        create_action_spec(ActionType.RewardSingingBowl, ArgSpec("id_card")),
        create_action_spec(ActionType.RewardTakeCard, ArgSpec("id_card")),
        create_action_spec(ActionType.RewardTakeGold),
        create_action_spec(ActionType.RewardTakeGoldStolen),
        create_action_spec(ActionType.RewardTakePotion, ArgSpec("id_potion")),
        create_action_spec(ActionType.RewardTakeRelic, ArgSpec("id_relic")),
        create_action_spec(ActionType.RoomSelect, ArgSpec("id_room")),
        create_action_spec(ActionType.RoomExit),
        # Shop
        create_action_spec(ActionType.ShopBuyCard, ArgSpec("id_card")),
        create_action_spec(ActionType.ShopBuyRelic, ArgSpec("id_relic")),
        create_action_spec(ActionType.ShopBuyPotion, ArgSpec("id_potion")),
        create_action_spec(ActionType.ShopPurge),
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
EventOption = _rs.EventOption
GameState = _rs.GameState
Intent = _rs.Intent
Map = _rs.Map
Room = _rs.Room
Modifier = _rs.Modifier
Monster = _rs.Monster
Relic = _rs.Relic
Potion = _rs.Potion

# Plain struct view: how a target gets chosen (pool + filters + selection)
Target = _rs.Target

# Content catalog: immutable template views over the static definitions plus
# state-free enumeration functions; live snapshots join back via (name, upgraded).
# PotionTemplate split from Potion when live entities gained the `id` join key
CardTemplate = _rs.CardTemplate
RelicTemplate = _rs.RelicTemplate
MonsterTemplate = _rs.MonsterTemplate
MonsterMoveTemplate = _rs.MonsterMoveTemplate
ModifierSpawnTemplate = _rs.ModifierSpawnTemplate
EventOptionTemplate = _rs.EventOptionTemplate
PotionTemplate = _rs.PotionTemplate
get_card_templates = _rs.get_card_templates
get_relic_templates = _rs.get_relic_templates
get_potion_templates = _rs.get_potion_templates
get_monster_templates = _rs.get_monster_templates
get_event_option_templates = _rs.get_event_option_templates

# Sum types: one flat frozen class per variant (EffectDamagePhysical, ...) plus a
# PEP 604 union alias per family. The union works as annotation, isinstance target,
# and typing.get_args source alike; the FFI returns instances of the flat classes.
EffectDamagePhysical = _rs.EffectDamagePhysical
EffectHeelHookProc = _rs.EffectHeelHookProc
EffectSkewerProc = _rs.EffectSkewerProc
EffectEscapePlanCheck = _rs.EffectEscapePlanCheck
EffectCardBottle = _rs.EffectCardBottle
EffectGlassKnifeDecay = _rs.EffectGlassKnifeDecay
EffectCardSetupSelect = _rs.EffectCardSetupSelect
EffectCardNightmareSelect = _rs.EffectCardNightmareSelect
EffectDistractionAdd = _rs.EffectDistractionAdd
EffectSetCostOverride = _rs.EffectSetCostOverride
EffectDamageFinisher = _rs.EffectDamageFinisher
EffectDamageFlechettes = _rs.EffectDamageFlechettes
EffectUnloadDiscard = _rs.EffectUnloadDiscard
EffectStormOfSteelProc = _rs.EffectStormOfSteelProc
EffectSneakyStrikeProc = _rs.EffectSneakyStrikeProc
EffectBlockGain = _rs.EffectBlockGain
EffectBombArm = _rs.EffectBombArm
EffectLifestealHeal = _rs.EffectLifestealHeal
EffectModifierDelta = _rs.EffectModifierDelta
EffectModifierMultiply = _rs.EffectModifierMultiply
EffectEnergyDelta = _rs.EffectEnergyDelta
EffectCardAdd = _rs.EffectCardAdd
EffectCardAddRandom = _rs.EffectCardAddRandom
EffectCardMove = _rs.EffectCardMove
EffectCardExhaust = _rs.EffectCardExhaust
EffectCardDrawIfNoAttacks = _rs.EffectCardDrawIfNoAttacks
EffectHandOfGreedProc = _rs.EffectHandOfGreedProc
EffectRitualDaggerProc = _rs.EffectRitualDaggerProc
EffectCardDraw = _rs.EffectCardDraw
EffectCardDrawUpTo = _rs.EffectCardDrawUpTo
EffectCardDiscard = _rs.EffectCardDiscard
EffectCardRetain = _rs.EffectCardRetain
EffectDamageMindBlast = _rs.EffectDamageMindBlast
EffectShuffleCardPileDiscardIntoCardPileDraw = _rs.EffectShuffleCardPileDiscardIntoCardPileDraw
EffectMaxHealthDelta = _rs.EffectMaxHealthDelta
EffectHealthDelta = _rs.EffectHealthDelta
EffectPotionAddRandom = _rs.EffectPotionAddRandom
EffectPotionDiscard = _rs.EffectPotionDiscard
EffectRewardRollPotions = _rs.EffectRewardRollPotions
EffectRewardRollGold = _rs.EffectRewardRollGold
EffectCardDiscoverRoll = _rs.EffectCardDiscoverRoll
EffectGoldDelta = _rs.EffectGoldDelta
EffectRelicGrantRandom = _rs.EffectRelicGrantRandom
EffectWheelSpin = _rs.EffectWheelSpin
EffectBonfireOffer = _rs.EffectBonfireOffer
EffectMonsterSpawn = _rs.EffectMonsterSpawn
EffectCombatStart = _rs.EffectCombatStart
EffectAdventurerSearch = _rs.EffectAdventurerSearch
EffectRelicGrantSpecific = _rs.EffectRelicGrantSpecific
EffectEventAdvanceState = _rs.EffectEventAdvanceState
EffectScrapOozeReach = _rs.EffectScrapOozeReach
EffectEventConsume = _rs.EffectEventConsume
EffectEventClose = _rs.EffectEventClose
EffectCardDiscoverSelect = _rs.EffectCardDiscoverSelect
EffectCardAdopt = _rs.EffectCardAdopt
EffectCardPileEventRollClear = _rs.EffectCardPileEventRollClear
EffectCardPurge = _rs.EffectCardPurge
EffectCardUpgrade = _rs.EffectCardUpgrade
EffectRestSmith = _rs.EffectRestSmith
EffectRestToke = _rs.EffectRestToke
EffectCardDuplicate = _rs.EffectCardDuplicate
EffectCardTransform = _rs.EffectCardTransform
EffectCardPlayFromCardPileDrawTop = _rs.EffectCardPlayFromCardPileDrawTop
EffectGamble = _rs.EffectGamble
EffectCombatEnd = _rs.EffectCombatEnd
EffectRelicLose = _rs.EffectRelicLose
EffectStrengthLoseTemp = _rs.EffectStrengthLoseTemp
EffectMausoleumOpen = _rs.EffectMausoleumOpen
EffectKnowingSkullCostBump = _rs.EffectKnowingSkullCostBump
EffectJoustBet = _rs.EffectJoustBet
EffectRelicGrantPool = _rs.EffectRelicGrantPool
EffectDebuffsClear = _rs.EffectDebuffsClear
EffectHexaghostBurnIncrease = _rs.EffectHexaghostBurnIncrease
EffectModifierRemove = _rs.EffectModifierRemove
EffectMonsterEscape = _rs.EffectMonsterEscape
EffectMonsterRemove = _rs.EffectMonsterRemove
EffectMonsterSplit = _rs.EffectMonsterSplit
EffectMonsterSummon = _rs.EffectMonsterSummon
EffectCardStasisSteal = _rs.EffectCardStasisSteal
EffectRewardRollCards = _rs.EffectRewardRollCards
EffectNeowCardsRoll = _rs.EffectNeowCardsRoll
EffectRewardRollRelic = _rs.EffectRewardRollRelic
EffectDamageDeal = _rs.EffectDamageDeal
EffectMatchFlipSeen = _rs.EffectMatchFlipSeen
EffectMatchFlipUnseen = _rs.EffectMatchFlipUnseen
EffectHalfDeadRebirth = _rs.EffectHalfDeadRebirth
EffectHealthSet = _rs.EffectHealthSet
EffectRoomEnter = _rs.EffectRoomEnter
EffectPhilosopherStoneProc = _rs.EffectPhilosopherStoneProc
RewardRollTrigger = _rs.RewardRollTrigger
Effect = (
    EffectDamagePhysical
    | EffectHeelHookProc
    | EffectSkewerProc
    | EffectEscapePlanCheck
    | EffectCardBottle
    | EffectGlassKnifeDecay
    | EffectCardSetupSelect
    | EffectCardNightmareSelect
    | EffectDistractionAdd
    | EffectSetCostOverride
    | EffectDamageFinisher
    | EffectDamageFlechettes
    | EffectUnloadDiscard
    | EffectStormOfSteelProc
    | EffectSneakyStrikeProc
    | EffectBlockGain
    | EffectBombArm
    | EffectLifestealHeal
    | EffectModifierDelta
    | EffectModifierMultiply
    | EffectEnergyDelta
    | EffectCardAdd
    | EffectCardAddRandom
    | EffectCardMove
    | EffectCardExhaust
    | EffectCardDrawIfNoAttacks
    | EffectHandOfGreedProc
    | EffectRitualDaggerProc
    | EffectCardDraw
    | EffectCardDrawUpTo
    | EffectCardDiscard
    | EffectCardRetain
    | EffectDamageMindBlast
    | EffectShuffleCardPileDiscardIntoCardPileDraw
    | EffectMaxHealthDelta
    | EffectHealthDelta
    | EffectPotionAddRandom
    | EffectPotionDiscard
    | EffectRewardRollPotions
    | EffectRewardRollGold
    | EffectCardDiscoverRoll
    | EffectGoldDelta
    | EffectRelicGrantRandom
    | EffectWheelSpin
    | EffectBonfireOffer
    | EffectMonsterSpawn
    | EffectCombatStart
    | EffectAdventurerSearch
    | EffectRelicGrantSpecific
    | EffectEventAdvanceState
    | EffectScrapOozeReach
    | EffectEventConsume
    | EffectEventClose
    | EffectCardDiscoverSelect
    | EffectCardAdopt
    | EffectCardPileEventRollClear
    | EffectCardPurge
    | EffectCardUpgrade
    | EffectRestSmith
    | EffectRestToke
    | EffectCardDuplicate
    | EffectCardTransform
    | EffectCardPlayFromCardPileDrawTop
    | EffectGamble
    | EffectCombatEnd
    | EffectRelicLose
    | EffectStrengthLoseTemp
    | EffectMausoleumOpen
    | EffectKnowingSkullCostBump
    | EffectJoustBet
    | EffectRelicGrantPool
    | EffectDebuffsClear
    | EffectHexaghostBurnIncrease
    | EffectModifierRemove
    | EffectMonsterEscape
    | EffectMonsterRemove
    | EffectMonsterSplit
    | EffectMonsterSummon
    | EffectCardStasisSteal
    | EffectRewardRollCards
    | EffectNeowCardsRoll
    | EffectRewardRollRelic
    | EffectDamageDeal
    | EffectMatchFlipSeen
    | EffectMatchFlipUnseen
    | EffectHalfDeadRebirth
    | EffectHealthSet
    | EffectRoomEnter
    | EffectPhilosopherStoneProc
)

# The kinds that can park in GameState.effect_pending. A halt only happens on a
# Target::Resolve carrying Input/InputUpTo, so this is a strict subset of Effect
# — mirrored from ffi/effect.rs's PyPendingEffect, which is what `pending`
# actually returns.
PendingEffect = (
    EffectBonfireOffer
    | EffectCardAdopt
    | EffectCardBottle
    | EffectCardDiscard
    | EffectCardDiscoverSelect
    | EffectCardDuplicate
    | EffectCardExhaust
    | EffectCardMove
    | EffectCardNightmareSelect
    | EffectCardPurge
    | EffectCardRetain
    | EffectCardSetupSelect
    | EffectCardTransform
    | EffectCardUpgrade
    | EffectMatchFlipSeen
    | EffectRelicLose
    | EffectRestSmith
    | EffectRestToke
)
CandidatePool = _rs.CandidatePool
SelectionKindAll = _rs.SelectionKindAll
SelectionKindSingle = _rs.SelectionKindSingle
SelectionKindRandom = _rs.SelectionKindRandom
SelectionKindInput = _rs.SelectionKindInput
SelectionKindInputUpTo = _rs.SelectionKindInputUpTo
SelectionKind = (
    SelectionKindAll
    | SelectionKindSingle
    | SelectionKindRandom
    | SelectionKindInput
    | SelectionKindInputUpTo
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
Amount = AmountAbsolute | AmountRelative | AmountRange
RestSite = _rs.RestSite
Chest = _rs.Chest
ChestKind = _rs.ChestKind
Combat = _rs.Combat
Reward = _rs.Reward
Shop = _rs.Shop
Event = _rs.Event

DeltaSign = _rs.DeltaSign
ReadAt = _rs.ReadAt
Rounding = _rs.Rounding


# Constants surface: load-bearing engine tunables (the engine's own roll/formula
# sites read these same values) plus derived shop price ceilings
STARTING_GOLD = _rs.STARTING_GOLD
MAX_GOLD = _rs.MAX_GOLD
GOLD_MONSTER_MIN = _rs.GOLD_MONSTER_MIN
GOLD_MONSTER_MAX = _rs.GOLD_MONSTER_MAX
GOLD_ELITE_MIN = _rs.GOLD_ELITE_MIN
GOLD_ELITE_MAX = _rs.GOLD_ELITE_MAX
GOLD_BOSS_MIN = _rs.GOLD_BOSS_MIN
GOLD_BOSS_MAX = _rs.GOLD_BOSS_MAX
CHEST_SMALL_GOLD_CHANCE = _rs.CHEST_SMALL_GOLD_CHANCE
CHEST_SMALL_GOLD_BASE = _rs.CHEST_SMALL_GOLD_BASE
CHEST_MEDIUM_GOLD_CHANCE = _rs.CHEST_MEDIUM_GOLD_CHANCE
CHEST_MEDIUM_GOLD_BASE = _rs.CHEST_MEDIUM_GOLD_BASE
CHEST_LARGE_GOLD_CHANCE = _rs.CHEST_LARGE_GOLD_CHANCE
CHEST_LARGE_GOLD_BASE = _rs.CHEST_LARGE_GOLD_BASE
CHEST_GOLD_VARIANCE_MIN = _rs.CHEST_GOLD_VARIANCE_MIN
CHEST_GOLD_VARIANCE_MAX = _rs.CHEST_GOLD_VARIANCE_MAX
SHOP_PRICE_CARD_COMMON = _rs.SHOP_PRICE_CARD_COMMON
SHOP_PRICE_CARD_UNCOMMON = _rs.SHOP_PRICE_CARD_UNCOMMON
SHOP_PRICE_CARD_RARE = _rs.SHOP_PRICE_CARD_RARE
SHOP_PRICE_COLORLESS_NUMER = _rs.SHOP_PRICE_COLORLESS_NUMER
SHOP_PRICE_COLORLESS_DENOM = _rs.SHOP_PRICE_COLORLESS_DENOM
SHOP_PRICE_CARD_VARIANCE_MIN = _rs.SHOP_PRICE_CARD_VARIANCE_MIN
SHOP_PRICE_CARD_VARIANCE_MAX = _rs.SHOP_PRICE_CARD_VARIANCE_MAX
SHOP_PRICE_POTION_COMMON = _rs.SHOP_PRICE_POTION_COMMON
SHOP_PRICE_POTION_UNCOMMON = _rs.SHOP_PRICE_POTION_UNCOMMON
SHOP_PRICE_POTION_RARE = _rs.SHOP_PRICE_POTION_RARE
SHOP_PRICE_RELIC_COMMON = _rs.SHOP_PRICE_RELIC_COMMON
SHOP_PRICE_RELIC_UNCOMMON = _rs.SHOP_PRICE_RELIC_UNCOMMON
SHOP_PRICE_RELIC_RARE = _rs.SHOP_PRICE_RELIC_RARE
SHOP_PRICE_RELIC_SHOP = _rs.SHOP_PRICE_RELIC_SHOP
SHOP_PRICE_RELIC_POTION_VARIANCE_MIN = _rs.SHOP_PRICE_RELIC_POTION_VARIANCE_MIN
SHOP_PRICE_RELIC_POTION_VARIANCE_MAX = _rs.SHOP_PRICE_RELIC_POTION_VARIANCE_MAX
SHOP_SALE_DIVISOR = _rs.SHOP_SALE_DIVISOR
SHOP_PURGE_COST_BASE = _rs.SHOP_PURGE_COST_BASE
SHOP_PURGE_COST_INCREMENT = _rs.SHOP_PURGE_COST_INCREMENT
ASCENSION_SHOP_PRICE_BUMP_LEVEL = _rs.ASCENSION_SHOP_PRICE_BUMP_LEVEL
ASCENSION_SHOP_PRICE_BUMP_NUMER = _rs.ASCENSION_SHOP_PRICE_BUMP_NUMER
ASCENSION_SHOP_PRICE_BUMP_DENOM = _rs.ASCENSION_SHOP_PRICE_BUMP_DENOM
SHOP_PRICE_CARD_MAX = _rs.SHOP_PRICE_CARD_MAX
SHOP_PRICE_RELIC_MAX = _rs.SHOP_PRICE_RELIC_MAX
SHOP_PRICE_POTION_MAX = _rs.SHOP_PRICE_POTION_MAX
WE_MEET_AGAIN_GOLD_ASK_MIN = _rs.WE_MEET_AGAIN_GOLD_ASK_MIN
WE_MEET_AGAIN_GOLD_ASK_MAX = _rs.WE_MEET_AGAIN_GOLD_ASK_MAX
NEOW_GOLD_SMALL = _rs.NEOW_GOLD_SMALL
NEOW_GOLD_LARGE = _rs.NEOW_GOLD_LARGE
NEOW_CARD_COUNT = _rs.NEOW_CARD_COUNT
NEOW_POTION_COUNT = _rs.NEOW_POTION_COUNT
SILENT_HP_MAX_BASE = _rs.SILENT_HP_MAX_BASE
SILENT_HP_MAX_A14_DELTA = _rs.SILENT_HP_MAX_A14_DELTA
HEALTH_START_A6_NUMER = _rs.HEALTH_START_A6_NUMER
HEALTH_START_A6_DENOM = _rs.HEALTH_START_A6_DENOM


__all__ = [
    # Environment + action
    "GameEnv",
    "Action",
    "ActionType",
    "ArgSpec",
    "ActionSpec",
    "ActionSpecRegistry",
    "create_action_spec",
    "ACTION_SPEC_REGISTRY",
    # Views
    "Card",
    "Character",
    "Energy",
    "EventOption",
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
    "CardPile",
    "CostScope",
    "PlayRestriction",
    "PlaySource",
    "ModifierKind",
    "IntentKind",
    "CandidateFilter",
    "RoomKind",
    "RelicName",
    "RelicTier",
    "RelicExclusion",
    "PotionName",
    "PotionRarity",
    "CardName",
    "MonsterName",
    "MonsterEncounter",
    # Sum types (flat variant classes + union aliases)
    "CandidatePool",
    "SelectionKind",
    "SelectionKindAll",
    "SelectionKindSingle",
    "SelectionKindRandom",
    "SelectionKindInput",
    "SelectionKindInputUpTo",
    "CardCostKind",
    "CardCostKindFixed",
    "CardCostKindMinusDiscardsThisTurn",
    "CardCostKindGrowsOnDamageInstanceTaken",
    "CardCostKindXCost",
    "Target",
    "Effect",
    "PendingEffect",
    "EffectDamagePhysical",
    "EffectHeelHookProc",
    "EffectSkewerProc",
    "EffectEscapePlanCheck",
    "EffectCardBottle",
    "EffectGlassKnifeDecay",
    "EffectCardSetupSelect",
    "EffectCardNightmareSelect",
    "EffectDistractionAdd",
    "EffectSetCostOverride",
    "EffectDamageFinisher",
    "EffectDamageFlechettes",
    "EffectUnloadDiscard",
    "EffectStormOfSteelProc",
    "EffectSneakyStrikeProc",
    "EffectBlockGain",
    "EffectBombArm",
    "EffectLifestealHeal",
    "EffectModifierDelta",
    "EffectModifierMultiply",
    "EffectEnergyDelta",
    "EffectCardAdd",
    "EffectCardAddRandom",
    "EffectCardMove",
    "EffectCardExhaust",
    "EffectCardDrawIfNoAttacks",
    "EffectHandOfGreedProc",
    "EffectRitualDaggerProc",
    "EffectCardDraw",
    "EffectCardDrawUpTo",
    "EffectCardDiscard",
    "EffectCardRetain",
    "EffectDamageMindBlast",
    "EffectShuffleCardPileDiscardIntoCardPileDraw",
    "EffectMaxHealthDelta",
    "EffectHealthDelta",
    "EffectPotionAddRandom",
    "EffectPotionDiscard",
    "EffectRewardRollPotions",
    "EffectRewardRollGold",
    "EffectCardDiscoverRoll",
    "EffectGoldDelta",
    "EffectRelicGrantRandom",
    "EffectWheelSpin",
    "EffectBonfireOffer",
    "EffectMonsterSpawn",
    "EffectCombatStart",
    "EffectAdventurerSearch",
    "EffectRelicGrantSpecific",
    "EffectEventAdvanceState",
    "EffectScrapOozeReach",
    "EffectEventConsume",
    "EffectEventClose",
    "EffectCardDiscoverSelect",
    "EffectCardAdopt",
    "EffectCardPileEventRollClear",
    "EffectCardPurge",
    "EffectCardUpgrade",
    "EffectRestSmith",
    "EffectRestToke",
    "EffectCardDuplicate",
    "EffectCardTransform",
    "EffectCardPlayFromCardPileDrawTop",
    "EffectGamble",
    "EffectCombatEnd",
    "EffectRelicLose",
    "EffectStrengthLoseTemp",
    "EffectMausoleumOpen",
    "EffectKnowingSkullCostBump",
    "EffectJoustBet",
    "EffectRelicGrantPool",
    "EffectDebuffsClear",
    "EffectHexaghostBurnIncrease",
    "EffectModifierRemove",
    "EffectMonsterEscape",
    "EffectMonsterRemove",
    "EffectMonsterSplit",
    "EffectMonsterSummon",
    "EffectCardStasisSteal",
    "EffectRewardRollCards",
    "EffectNeowCardsRoll",
    "EffectRewardRollRelic",
    "EffectDamageDeal",
    "EffectMatchFlipSeen",
    "EffectMatchFlipUnseen",
    "EffectHalfDeadRebirth",
    "EffectHealthSet",
    "EffectRoomEnter",
    "EffectPhilosopherStoneProc",
    "RewardRollTrigger",
    "Amount",
    "AmountAbsolute",
    "AmountRelative",
    "AmountRange",
    "RestSite",
    "Chest",
    "ChestKind",
    "Combat",
    "Reward",
    "Shop",
    "Event",
    "DeltaSign",
    "ReadAt",
    "Rounding",
    # Potion
    "Potion",
    # Content catalog
    "CardTemplate",
    "RelicTemplate",
    "PotionTemplate",
    "MonsterTemplate",
    "MonsterMoveTemplate",
    "ModifierSpawnTemplate",
    "EventOptionTemplate",
    "MonsterKind",
    "EventName",
    "get_card_templates",
    "get_relic_templates",
    "get_potion_templates",
    "get_monster_templates",
    "get_event_option_templates",
    # Constants surface
    "STARTING_GOLD",
    "MAX_GOLD",
    "GOLD_MONSTER_MIN",
    "GOLD_MONSTER_MAX",
    "GOLD_ELITE_MIN",
    "GOLD_ELITE_MAX",
    "GOLD_BOSS_MIN",
    "GOLD_BOSS_MAX",
    "CHEST_SMALL_GOLD_CHANCE",
    "CHEST_SMALL_GOLD_BASE",
    "CHEST_MEDIUM_GOLD_CHANCE",
    "CHEST_MEDIUM_GOLD_BASE",
    "CHEST_LARGE_GOLD_CHANCE",
    "CHEST_LARGE_GOLD_BASE",
    "CHEST_GOLD_VARIANCE_MIN",
    "CHEST_GOLD_VARIANCE_MAX",
    "SHOP_PRICE_CARD_COMMON",
    "SHOP_PRICE_CARD_UNCOMMON",
    "SHOP_PRICE_CARD_RARE",
    "SHOP_PRICE_COLORLESS_NUMER",
    "SHOP_PRICE_COLORLESS_DENOM",
    "SHOP_PRICE_CARD_VARIANCE_MIN",
    "SHOP_PRICE_CARD_VARIANCE_MAX",
    "SHOP_PRICE_POTION_COMMON",
    "SHOP_PRICE_POTION_UNCOMMON",
    "SHOP_PRICE_POTION_RARE",
    "SHOP_PRICE_RELIC_COMMON",
    "SHOP_PRICE_RELIC_UNCOMMON",
    "SHOP_PRICE_RELIC_RARE",
    "SHOP_PRICE_RELIC_SHOP",
    "SHOP_PRICE_RELIC_POTION_VARIANCE_MIN",
    "SHOP_PRICE_RELIC_POTION_VARIANCE_MAX",
    "SHOP_SALE_DIVISOR",
    "SHOP_PURGE_COST_BASE",
    "SHOP_PURGE_COST_INCREMENT",
    "ASCENSION_SHOP_PRICE_BUMP_LEVEL",
    "ASCENSION_SHOP_PRICE_BUMP_NUMER",
    "ASCENSION_SHOP_PRICE_BUMP_DENOM",
    "SHOP_PRICE_CARD_MAX",
    "SHOP_PRICE_RELIC_MAX",
    "SHOP_PRICE_POTION_MAX",
    "WE_MEET_AGAIN_GOLD_ASK_MIN",
    "WE_MEET_AGAIN_GOLD_ASK_MAX",
    "NEOW_GOLD_SMALL",
    "NEOW_GOLD_LARGE",
    "NEOW_CARD_COUNT",
    "NEOW_POTION_COUNT",
    "SILENT_HP_MAX_BASE",
    "SILENT_HP_MAX_A14_DELTA",
    "HEALTH_START_A6_NUMER",
    "HEALTH_START_A6_DENOM",
]
