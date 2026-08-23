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
CardPile = _rs.CardPile
CostScope = _rs.CostScope
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
CandidateFilter = _rs.CandidateFilter
MonsterKind = _rs.MonsterKind
EventName = _rs.EventName


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
_HAND_POS = "position in state.combat.hand (the current hand)"
_MONSTER_POS = "position in the alive-monster list at dispatch time"
_REWARD_BUNDLE_POS = "bundle in state.reward.cards"
_REWARD_POS = "card within the bundle"
_REWARD_RELIC_POS = "slot in state.reward.relics"
_DECK_POS = "position in state.deck (the full deck)"
_MAP_COL = "column on the next map row (0..MAP_WIDTH)"
_SLOT_POS = "slot in state.potions"
_REWARD_POTION_POS = "slot in state.reward.potions"
_DISCOVER_POS = "position in state.combat.discover (the discovery offer)"
_SHOP_CARD_POS = "position in state.shop.cards"
_SHOP_RELIC_POS = "position in state.shop.relics"
_SHOP_POTION_POS = "position in state.shop.potions"


# Action spec registry
ACTION_SPEC_REGISTRY = ActionSpecRegistry(
    [
        create_action_spec(
            ActionType.CardPlay,
            ArgSpec("idx_card", _HAND_POS),
            ArgSpec("idx_monster", _MONSTER_POS, optional=True),
        ),
        create_action_spec(ActionType.ChestOpen),
        # Pending-halt family: one resolver, one skip
        create_action_spec(
            ActionType.EffectPendingResolve,
            ArgSpec(
                "idx",
                "position in the collection named by state.pending.target.candidate_pool "
                "(hand, deck, discover, pile, or event roll list)",
            ),
        ),
        create_action_spec(ActionType.PickSkip),
        create_action_spec(
            ActionType.EventOptionSelect, ArgSpec("idx", "position in state.event.options")
        ),
        create_action_spec(ActionType.PotionDiscard, ArgSpec("idx_slot", _SLOT_POS)),
        create_action_spec(
            ActionType.PotionUse,
            ArgSpec("idx_potion", _SLOT_POS),
            ArgSpec("idx_monster", _MONSTER_POS, optional=True),
        ),
        create_action_spec(ActionType.Rest),
        create_action_spec(ActionType.RestDig),
        create_action_spec(ActionType.RestLift),
        create_action_spec(ActionType.RestSmith),
        create_action_spec(ActionType.RestToke),
        # Reward pickup family
        create_action_spec(
            ActionType.RewardSingingBowl, ArgSpec("idx_bundle", _REWARD_BUNDLE_POS)
        ),
        create_action_spec(
            ActionType.RewardTakeCard,
            ArgSpec("idx_bundle", _REWARD_BUNDLE_POS),
            ArgSpec("idx_card", _REWARD_POS),
        ),
        create_action_spec(ActionType.RewardTakeGold),
        create_action_spec(
            ActionType.RewardTakePotion, ArgSpec("idx", _REWARD_POTION_POS)
        ),
        create_action_spec(ActionType.RewardTakeRelic, ArgSpec("idx", _REWARD_RELIC_POS)),
        create_action_spec(ActionType.RoomSelect, ArgSpec("idx", _MAP_COL)),
        create_action_spec(ActionType.RoomExit),
        # Shop
        create_action_spec(ActionType.ShopBuyCard, ArgSpec("idx", _SHOP_CARD_POS)),
        create_action_spec(ActionType.ShopBuyRelic, ArgSpec("idx", _SHOP_RELIC_POS)),
        create_action_spec(ActionType.ShopBuyPotion, ArgSpec("idx", _SHOP_POTION_POS)),
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
GameState = _rs.GameState
Intent = _rs.Intent
Map = _rs.Map
Room = _rs.Room
Modifier = _rs.Modifier
Monster = _rs.Monster
Relic = _rs.Relic
Potion = _rs.Potion

# Plain struct view: how a target gets chosen (pool + filter + selection)
Target = _rs.Target

# Content catalog: immutable template views over the static definitions plus
# state-free enumeration functions; live snapshots join back via (name, upgraded).
# PotionTemplate split from Potion when live entities gained the `id` join key
CardTemplate = _rs.CardTemplate
RelicTemplate = _rs.RelicTemplate
MonsterTemplate = _rs.MonsterTemplate
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
EffectDamagePhysicalIfPoisoned = _rs.EffectDamagePhysicalIfPoisoned
EffectHeelHookProc = _rs.EffectHeelHookProc
EffectEscapePlanCheck = _rs.EffectEscapePlanCheck
EffectCardBottle = _rs.EffectCardBottle
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
EffectShuffleDiscardPileIntoDrawPile = _rs.EffectShuffleDiscardPileIntoDrawPile
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
EffectMonsterSpawn = _rs.EffectMonsterSpawn
EffectCombatStart = _rs.EffectCombatStart
EffectAdventurerSearch = _rs.EffectAdventurerSearch
EffectRelicGrantSpecific = _rs.EffectRelicGrantSpecific
EffectEventAdvanceState = _rs.EffectEventAdvanceState
EffectScrapOozeReach = _rs.EffectScrapOozeReach
EffectEventConsume = _rs.EffectEventConsume
EffectCardDiscoverPick = _rs.EffectCardDiscoverPick
EffectCardPurge = _rs.EffectCardPurge
EffectCardUpgrade = _rs.EffectCardUpgrade
EffectCardDuplicate = _rs.EffectCardDuplicate
EffectCardTransform = _rs.EffectCardTransform
EffectCardPlayFromDrawTop = _rs.EffectCardPlayFromDrawTop
EffectGamble = _rs.EffectGamble
EffectCombatEnd = _rs.EffectCombatEnd
EffectRelicLose = _rs.EffectRelicLose
EffectRewardRollNeowCards = _rs.EffectRewardRollNeowCards
EffectStrengthLoseTemp = _rs.EffectStrengthLoseTemp
EffectMausoleumOpen = _rs.EffectMausoleumOpen
EffectKnowingSkullCostBump = _rs.EffectKnowingSkullCostBump
EffectJoustBet = _rs.EffectJoustBet
EffectRewardRollLibraryCards = _rs.EffectRewardRollLibraryCards
EffectRelicGrantPool = _rs.EffectRelicGrantPool
Effect = (
    EffectDamagePhysical
    | EffectDamagePhysicalIfPoisoned
    | EffectHeelHookProc
    | EffectEscapePlanCheck
    | EffectCardBottle
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
    | EffectShuffleDiscardPileIntoDrawPile
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
    | EffectMonsterSpawn
    | EffectCombatStart
    | EffectAdventurerSearch
    | EffectRelicGrantSpecific
    | EffectEventAdvanceState
    | EffectScrapOozeReach
    | EffectEventConsume
    | EffectCardDiscoverPick
    | EffectCardPurge
    | EffectCardUpgrade
    | EffectCardDuplicate
    | EffectCardTransform
    | EffectCardPlayFromDrawTop
    | EffectGamble
    | EffectCombatEnd
    | EffectRelicLose
    | EffectRewardRollNeowCards
    | EffectStrengthLoseTemp
    | EffectMausoleumOpen
    | EffectKnowingSkullCostBump
    | EffectJoustBet
    | EffectRewardRollLibraryCards
    | EffectRelicGrantPool
)

# The kinds that can park in GameState.pending. A halt only happens on a
# Target::Resolve carrying Input/InputUpTo, so this is a strict subset of Effect
# — mirrored from ffi/effect.rs's PyPendingEffect, which is what `pending`
# actually returns.
PendingEffect = (
    EffectBonfireOffer
    | EffectCardBottle
    | EffectCardDiscard
    | EffectCardDiscoverPick
    | EffectCardDuplicate
    | EffectCardExhaust
    | EffectCardMove
    | EffectCardNightmarePick
    | EffectCardPurge
    | EffectCardRetain
    | EffectCardSetupPick
    | EffectCardTransform
    | EffectCardUpgrade
    | EffectRelicLose
)
CandidatePoolHand = _rs.CandidatePoolHand
CandidatePoolCharacter = _rs.CandidatePoolCharacter
CandidatePoolMonsters = _rs.CandidatePoolMonsters
CandidatePoolSource = _rs.CandidatePoolSource
CandidatePoolDiscover = _rs.CandidatePoolDiscover
CandidatePoolDeck = _rs.CandidatePoolDeck
CandidatePoolPileDraw = _rs.CandidatePoolPileDraw
CandidatePoolPileDiscard = _rs.CandidatePoolPileDiscard
CandidatePoolPileExhaust = _rs.CandidatePoolPileExhaust
CandidatePoolEventRollCard = _rs.CandidatePoolEventRollCard
CandidatePoolEventRollRelic = _rs.CandidatePoolEventRollRelic
CandidatePoolEventRollPotion = _rs.CandidatePoolEventRollPotion
CandidatePool = (
    CandidatePoolHand
    | CandidatePoolCharacter
    | CandidatePoolMonsters
    | CandidatePoolSource
    | CandidatePoolDiscover
    | CandidatePoolDeck
    | CandidatePoolPileDraw
    | CandidatePoolPileDiscard
    | CandidatePoolPileExhaust
    | CandidatePoolEventRollCard
    | CandidatePoolEventRollRelic
    | CandidatePoolEventRollPotion
)
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
HP_START_FRACTION_A6 = _rs.HP_START_FRACTION_A6


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
    "CardPile",
    "CostScope",
    "PlayRestriction",
    "ModifierKind",
    "IntentKind",
    "CandidateFilter",
    "RoomKind",
    "RelicName",
    "RelicTier",
    "PotionName",
    "PotionRarity",
    "CardName",
    "MonsterName",
    "MonsterEncounter",
    # Sum types (flat variant classes + union aliases)
    "CandidatePool",
    "CandidatePoolHand",
    "CandidatePoolCharacter",
    "CandidatePoolMonsters",
    "CandidatePoolSource",
    "CandidatePoolDiscover",
    "CandidatePoolDeck",
    "CandidatePoolPileDraw",
    "CandidatePoolPileDiscard",
    "CandidatePoolPileExhaust",
    "CandidatePoolEventRollCard",
    "CandidatePoolEventRollRelic",
    "CandidatePoolEventRollPotion",
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
    "EffectDamagePhysicalIfPoisoned",
    "EffectHeelHookProc",
    "EffectEscapePlanCheck",
    "EffectCardBottle",
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
    "EffectShuffleDiscardPileIntoDrawPile",
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
    "EffectMonsterSpawn",
    "EffectCombatStart",
    "EffectAdventurerSearch",
    "EffectRelicGrantSpecific",
    "EffectEventAdvanceState",
    "EffectScrapOozeReach",
    "EffectEventConsume",
    "EffectCardDiscoverPick",
    "EffectCardPurge",
    "EffectCardUpgrade",
    "EffectCardDuplicate",
    "EffectCardTransform",
    "EffectCardPlayFromDrawTop",
    "EffectGamble",
    "EffectCombatEnd",
    "EffectRelicLose",
    "EffectRewardRollNeowCards",
    "EffectStrengthLoseTemp",
    "EffectMausoleumOpen",
    "EffectKnowingSkullCostBump",
    "EffectJoustBet",
    "EffectRewardRollLibraryCards",
    "EffectRelicGrantPool",
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
    # Potion
    "Potion",
    # Content catalog
    "CardTemplate",
    "RelicTemplate",
    "PotionTemplate",
    "MonsterTemplate",
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
    "HP_START_FRACTION_A6",
]
