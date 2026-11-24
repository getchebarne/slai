from typing import Iterator, NamedTuple, Optional, TypeAlias, Union

from . import _slai as _rs

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
        create_action_spec(ActionType.RestRecall),
        create_action_spec(ActionType.RestSmith),
        create_action_spec(ActionType.RestToke),
        # Reward pickup family
        create_action_spec(ActionType.RewardSingingBowl, ArgSpec("id_card")),
        create_action_spec(ActionType.RewardTakeCard, ArgSpec("id_card")),
        create_action_spec(ActionType.RewardTakeGold),
        create_action_spec(ActionType.RewardTakeGoldStolen),
        create_action_spec(ActionType.RewardTakePotion, ArgSpec("id_potion")),
        create_action_spec(ActionType.RewardTakeRelic, ArgSpec("id_relic")),
        create_action_spec(ActionType.RewardTakeSapphireKey),
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
CardPlay = _rs.CardPlay
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
# The generated _slai.pyi imports the union aliases by name; keep them even when unused here
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
EffectEventAdvanceStage = _rs.EffectEventAdvanceStage
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
EffectModifierStrengthNegativeRemove = _rs.EffectModifierStrengthNegativeRemove
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
Effect: TypeAlias = (
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
    | EffectEventAdvanceStage
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
    | EffectModifierStrengthNegativeRemove
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

CandidatePool = _rs.CandidatePool
SelectionKindAll = _rs.SelectionKindAll
SelectionKindSingle = _rs.SelectionKindSingle
SelectionKindRandom = _rs.SelectionKindRandom
SelectionKindInput = _rs.SelectionKindInput
SelectionKindInputUpTo = _rs.SelectionKindInputUpTo
SelectionKind: TypeAlias = (
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
CardCostKind: TypeAlias = (
    CardCostKindFixed
    | CardCostKindMinusDiscardsThisTurn
    | CardCostKindGrowsOnDamageInstanceTaken
    | CardCostKindXCost
)

LocationStart = _rs.LocationStart
LocationOverworld = _rs.LocationOverworld
LocationBossRoom = _rs.LocationBossRoom
LocationBossTreasure = _rs.LocationBossTreasure
LocationVictory = _rs.LocationVictory
Location: TypeAlias = (
    LocationStart
    | LocationOverworld
    | LocationBossRoom
    | LocationBossTreasure
    | LocationVictory
)

AmountAbsolute = _rs.AmountAbsolute
AmountRelative = _rs.AmountRelative
AmountRange = _rs.AmountRange
Amount: TypeAlias = AmountAbsolute | AmountRelative | AmountRange
AmountScalar: TypeAlias = AmountAbsolute | AmountRelative
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


# Constants surface
ENERGY_MAX_BASE = _rs.ENERGY_MAX_BASE
MAX_MOVE_HISTORY = _rs.MAX_MOVE_HISTORY
NUM_ENCOUNTERS_EASY_ACT1 = _rs.NUM_ENCOUNTERS_EASY_ACT1
TIME_WARP_PLAYS = _rs.TIME_WARP_PLAYS
PANACHE_PLAYS = _rs.PANACHE_PLAYS
MAX_NORMALITY_PLAYS = _rs.MAX_NORMALITY_PLAYS
MAX_VELVET_CHOKER_PLAYS = _rs.MAX_VELVET_CHOKER_PLAYS
MAX_POCKETWATCH_PLAYS = _rs.MAX_POCKETWATCH_PLAYS
ORICHALCUM_BLOCK = _rs.ORICHALCUM_BLOCK
MIN_TORII_DAMAGE = _rs.MIN_TORII_DAMAGE
MAX_TORII_DAMAGE = _rs.MAX_TORII_DAMAGE
TORII_DAMAGE_LANDED = _rs.TORII_DAMAGE_LANDED
BURN_DAMAGE = _rs.BURN_DAMAGE
BURN_DAMAGE_UPGRADED = _rs.BURN_DAMAGE_UPGRADED
DECAY_DAMAGE = _rs.DECAY_DAMAGE
EXPLOSIVE_DAMAGE = _rs.EXPLOSIVE_DAMAGE
NUM_CHEMICAL_X_REPEATS = _rs.NUM_CHEMICAL_X_REPEATS
MAX_MONSTERS = _rs.MAX_MONSTERS
MAX_SIZE_CARD_PILE_HAND = _rs.MAX_SIZE_CARD_PILE_HAND
MAX_COMBAT_CARD_REWARD = _rs.MAX_COMBAT_CARD_REWARD
MAX_REWARD_CARD_BUNDLES = _rs.MAX_REWARD_CARD_BUNDLES
MAX_CARD_REWARD_ROLL = _rs.MAX_CARD_REWARD_ROLL
NUM_CAULDRON_POTIONS = _rs.NUM_CAULDRON_POTIONS
MAX_SIZE_CARD_PILE_EVENT_ROLL = _rs.MAX_SIZE_CARD_PILE_EVENT_ROLL
MAX_SIZE_RELIC_EVENT_ROLL = _rs.MAX_SIZE_RELIC_EVENT_ROLL
MAX_SIZE_POTION_EVENT_ROLL = _rs.MAX_SIZE_POTION_EVENT_ROLL
MAX_POTION_SLOTS = _rs.MAX_POTION_SLOTS
NUM_BOSS_RELIC_REWARDS = _rs.NUM_BOSS_RELIC_REWARDS
SHOP_SLOTS_CARD_TOTAL = _rs.SHOP_SLOTS_CARD_TOTAL
SHOP_SLOTS_RELIC = _rs.SHOP_SLOTS_RELIC
SHOP_SLOTS_POTION = _rs.SHOP_SLOTS_POTION
NUM_DISCOVER_ROLLS = _rs.NUM_DISCOVER_ROLLS
BOMB_FUSE_TURNS = _rs.BOMB_FUSE_TURNS
FACTOR_VULN = _rs.FACTOR_VULN
HEXAGHOST_DIVIDER_HITS = _rs.HEXAGHOST_DIVIDER_HITS
HEXAGHOST_DIVIDER_DIVISOR = _rs.HEXAGHOST_DIVIDER_DIVISOR
MAP_HEIGHT = _rs.MAP_HEIGHT
ACT_FINAL = _rs.ACT_FINAL
MAX_ASCENSION = _rs.MAX_ASCENSION
MAP_WIDTH = _rs.MAP_WIDTH
MAP_ROW_TREASURE = _rs.MAP_ROW_TREASURE
CARD_REWARD_ROLL_OFFSET_BASE = _rs.CARD_REWARD_ROLL_OFFSET_BASE
MIN_CARD_REWARD_ROLL_OFFSET = _rs.MIN_CARD_REWARD_ROLL_OFFSET
POTION_DROP_CHANCE_BASE = _rs.POTION_DROP_CHANCE_BASE
MATCH_AND_KEEP_ATTEMPTS = _rs.MATCH_AND_KEEP_ATTEMPTS
NUM_MATCH_AND_KEEP_CARDS = _rs.NUM_MATCH_AND_KEEP_CARDS
MAX_SHOP_PRICE_CARD = _rs.MAX_SHOP_PRICE_CARD
MAX_SHOP_PRICE_RELIC = _rs.MAX_SHOP_PRICE_RELIC
MAX_SHOP_PRICE_POTION = _rs.MAX_SHOP_PRICE_POTION


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
    "CardPlay",
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
    "Location",
    "LocationStart",
    "LocationOverworld",
    "LocationBossRoom",
    "LocationBossTreasure",
    "LocationVictory",
    "Target",
    "Effect",
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
    "EffectEventAdvanceStage",
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
    "EffectModifierStrengthNegativeRemove",
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
    "AmountScalar",
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
    "ENERGY_MAX_BASE",
    "MAX_MOVE_HISTORY",
    "NUM_ENCOUNTERS_EASY_ACT1",
    "TIME_WARP_PLAYS",
    "PANACHE_PLAYS",
    "MAX_NORMALITY_PLAYS",
    "MAX_VELVET_CHOKER_PLAYS",
    "MAX_POCKETWATCH_PLAYS",
    "ORICHALCUM_BLOCK",
    "MIN_TORII_DAMAGE",
    "MAX_TORII_DAMAGE",
    "TORII_DAMAGE_LANDED",
    "BURN_DAMAGE",
    "BURN_DAMAGE_UPGRADED",
    "DECAY_DAMAGE",
    "EXPLOSIVE_DAMAGE",
    "NUM_CHEMICAL_X_REPEATS",
    "MAX_MONSTERS",
    "MAX_SIZE_CARD_PILE_HAND",
    "MAX_COMBAT_CARD_REWARD",
    "MAX_REWARD_CARD_BUNDLES",
    "MAX_CARD_REWARD_ROLL",
    "NUM_CAULDRON_POTIONS",
    "MAX_SIZE_CARD_PILE_EVENT_ROLL",
    "MAX_SIZE_RELIC_EVENT_ROLL",
    "MAX_SIZE_POTION_EVENT_ROLL",
    "MAX_POTION_SLOTS",
    "NUM_BOSS_RELIC_REWARDS",
    "SHOP_SLOTS_CARD_TOTAL",
    "SHOP_SLOTS_RELIC",
    "SHOP_SLOTS_POTION",
    "NUM_DISCOVER_ROLLS",
    "BOMB_FUSE_TURNS",
    "FACTOR_VULN",
    "HEXAGHOST_DIVIDER_HITS",
    "HEXAGHOST_DIVIDER_DIVISOR",
    "MAP_HEIGHT",
    "ACT_FINAL",
    "MAX_ASCENSION",
    "MAP_WIDTH",
    "MAP_ROW_TREASURE",
    "CARD_REWARD_ROLL_OFFSET_BASE",
    "MIN_CARD_REWARD_ROLL_OFFSET",
    "POTION_DROP_CHANCE_BASE",
    "MATCH_AND_KEEP_ATTEMPTS",
    "NUM_MATCH_AND_KEEP_CARDS",
    "MAX_SHOP_PRICE_CARD",
    "MAX_SHOP_PRICE_RELIC",
    "MAX_SHOP_PRICE_POTION",
]
