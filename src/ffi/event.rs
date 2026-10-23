use pyo3::prelude::*;

use super::macros::mirror_enum;

use crate::entity::Entity;
use crate::types::EventName;

use super::effect::PyEffect;
use super::effect::snapshot_effect;

mirror_enum!(PyEventName from EventName, "EventName", {
    BigFish, TheCleric, Duplicator, GoldenShrine, WingStatue, WorldOfGoop, LivingWall, Purifier,
    ShiningLight, TheSsssserpent, Transmogrifier, UpgradeShrine, TheDivineFountain, TheLab,
    TheWomanInBlue, WheelOfChange, BonfireSpirits, OminousForge, FaceTrader, Falling, Mushrooms,
    GoldenIdol, ScrapOoze, WeMeetAgain, DeadAdventurer, Neow, Addict, Beggar, Ghosts,
    BackToBasics, MaskedBandits, TheJoust, TheLibrary, TheMausoleum, Vampires, Colosseum,
    Designer, KnowingSkull, Nest, CursedTome, DrugDealer, ForgottenAltar, Nloth, MatchAndKeep,
});

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EventOption",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEventOption {
    pub id: usize,
    pub effects: Vec<PyEffect>,
}

pub(crate) fn snapshot_event_option(id: usize, entity: &Entity) -> PyEventOption {
    PyEventOption {
        id,
        effects: entity.event_option_effects[..entity.event_option_effects_len as usize]
            .iter()
            .map(snapshot_effect)
            .collect(),
    }
}
