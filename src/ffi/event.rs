use pyo3::prelude::*;

use super::macros::mirror_enum;
use crate::types::EventName;

mirror_enum!(PyEventName from EventName, "EventName", {
    BigFish, TheCleric, Duplicator, GoldenShrine, WingStatue, WorldOfGoop, LivingWall, Purifier,
    ShiningLight, TheSsssserpent, Transmogrifier, UpgradeShrine, TheDivineFountain, TheLab,
    TheWomanInBlue, WheelOfChange, BonfireSpirits, OminousForge, FaceTrader, Mushrooms,
    GoldenIdol, ScrapOoze, WeMeetAgain, DeadAdventurer, Neow, Addict, Beggar, Ghosts,
    BackToBasics, MaskedBandits, TheJoust, TheLibrary, TheMausoleum, Vampires, Colosseum,
    Designer, KnowingSkull, Nest, CursedTome, DrugDealer, ForgottenAltar, Nloth,
});
