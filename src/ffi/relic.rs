use pyo3::prelude::*;

use super::macros::mirror_enum;

use crate::entity::Entity;
use crate::types::RelicName;
use crate::types::RelicTier;

use super::effect::PyEffect;
use super::effect::snapshot_effect;

mirror_enum!(PyRelicName from RelicName, "RelicName", skip_from_py_object, {
    SnakeRing, Akabeko, Anchor, BagOfMarbles, BagOfPreparation, BloodVial, BronzeScales, Kunai,
    NinjaScroll, OddlySmoothStone, Shuriken, ThreadAndNeedle, TwistedFunnel, Vajra, Circlet,
    GoldenIdol, Lantern, ClockworkSouvenir, GremlinVisage, RedMask, Nunchaku, InkBottle,
    LetterOpener, OrnamentalFan, BirdFacedUrn, MummifiedHand, OrangePellets, StrangeSpoon,
    ChemicalX, ArtOfWar, Orichalcum, Pocketwatch, StoneCalendar, Abacus, Sundial,
    WhiteBeastStatue, DollysMirror, LeesWaffle, HappyFlower, IncenseBurner, MercuryHourglass,
    HornCleat, CaptainsWheel, Calipers, IceCream, SneckoSkull, Ginger, Turnip, Tingsha,
    ToughBandages, GremlinHorn, TheSpecimen, LizardTail, Boot, Torii, TungstenRod, HandDrill,
    StrikeDummy, PaperKrane, CentennialPuzzle, MealTicket, MawBank, JuzuBracelet, TinyChest,
    EternalFeather, AncientTeaSet, RegalPillow, MeatOnTheBone, Omamori, DarkstonePeriapt,
    CeramicFish, FrozenEgg, MoltenEgg, ToxicEgg, ToyOrnithopter, SmilingMask, DeadBranch,
    DuVuDoll, Pantograph, SlingOfCourage, Strawberry, Pear, Mango, OldCoin, PotionBelt,
    WarPaint, Whetstone, EmptyCage, PandorasBox, PenNib, FossilizedHelix, PreservedInsect,
    UnceasingTop, BlueCandle, MedicalKit, SpiritPoop, WarpedTongs, CultistHeadpiece,
    FaceOfCleric, NlothsHungryFace, SsserpentHead, OddMushroom, PhilosopherStone,
    CoffeeDripper, FusionHammer, Sozu, CursedKey, BustedCrown, SlaversCollar, Ectoplasm,
    VelvetChoker, WristBlade, HoveringKite, DreamCatcher, Cauldron, MembershipCard, TheCourier,
    GamblingChip, BottledFlame, BottledLightning, BottledTornado, Matryoshka, Orrery, Toolbox,
    SneckoEye, Astrolabe, CallingBell, TinyHouse, BlackStar, Girya, PeacePipe, Shovel,
    WingBoots, QuestionCard, SingingBowl, PrayerWheel, RunicPyramid, RingOfTheSerpent,
    SacredBark,
});

mirror_enum!(PyRelicTier from RelicTier, "RelicTier", skip_from_py_object, {
    Starter, Common, Uncommon, Rare, Boss, Shop, Special,
});

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "Relic",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyRelic {
    pub name: PyRelicName,
    pub tier: PyRelicTier,
    pub counter: i16,
    pub used_up: bool,
    pub effects_on_combat_start: Vec<PyEffect>,
}

pub(crate) fn snapshot_relic(entity: &Entity) -> PyRelic {
    PyRelic {
        name: entity.relic_name.into(),
        tier: entity.relic_tier.into(),
        counter: entity.relic_counter,
        used_up: entity.relic_used_up,
        effects_on_combat_start: entity
            .relic_effects_on_combat_start
            .iter()
            .map(snapshot_effect)
            .collect(),
    }
}
