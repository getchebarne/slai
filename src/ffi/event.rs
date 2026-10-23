use pyo3::prelude::*;

use super::macros::mirror_enum;

use crate::entity::Entity;
use crate::types::EventName;

use super::effect::PyEffect;
use super::effect::snapshot_effect;

mirror_enum!(PyEventName from EventName, "EventName", {
    Addict, BackToBasics, Beggar, BigFish, BonfireSpirits, Colosseum, CursedTome, DeadAdventurer,
    Designer, DrugDealer, Duplicator, FaceTrader, Falling, ForgottenAltar, Ghosts, GoldenIdol,
    GoldenShrine, KnowingSkull, LivingWall, MaskedBandits, MatchAndKeep, Mushrooms, Neow, Nest,
    Nloth, OminousForge, Purifier, ScrapOoze, ShiningLight, TheCleric, TheDivineFountain, TheJoust,
    TheLab, TheLibrary, TheMausoleum, TheSsssserpent, TheWomanInBlue, Transmogrifier, UpgradeShrine,
    Vampires, WeMeetAgain, WheelOfChange, WingStatue, WorldOfGoop,
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
