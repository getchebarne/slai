use pyo3::prelude::*;

use super::macros::mirror_enum;

use crate::action::Action;
use crate::game::GameState;
use crate::types::EventName;

use super::effect::PyEffect;
use super::effect::snapshot_effects;

mirror_enum!(PyEventName from EventName, "EventName", {
    Addict, BackToBasics, Beggar, BigFish, BonfireSpirits, Colosseum, CursedTome, DeadAdventurer,
    Designer, DrugDealer, Duplicator, FaceTrader, Falling, ForgottenAltar, Ghosts, GoldenIdol,
    GoldenShrine, KnowingSkull, LivingWall, MaskedBandits, MatchAndKeep, MindBloom, Mushrooms,
    MysteriousSphere, Neow, Nest, Nloth, OminousForge, Purifier, ScrapOoze, SecretPortal,
    SensoryStone, ShiningLight, TheCleric, TheDivineFountain, TheJoust, TheLab, TheLibrary,
    TheMausoleum, TheMoaiHead, TheSsssserpent, TheWomanInBlue, TombOfLordRedMask, Transmogrifier,
    UpgradeShrine, Vampires, WeMeetAgain, WheelOfChange, WindingHalls, WingStatue, WorldOfGoop,
});

#[pyclass(eq, hash, frozen, get_all, name = "EventOption", module = "slai._slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyEventOption {
    pub id: usize,
    pub effects: Vec<PyEffect>,
    pub available: bool, // Selectable now: an EventOptionSelect for it is legal
}

pub(crate) fn snapshot_event_option(state: &GameState, id: usize) -> PyEventOption {
    let entity = &state.entities[id];
    PyEventOption {
        id,
        effects: snapshot_effects(
            state,
            &entity.event_option_effects[..entity.event_option_effects_len as usize],
            None,
        ),
        available: state
            .legal_actions
            .contains(&Action::EventOptionSelect { id_option: id }),
    }
}
