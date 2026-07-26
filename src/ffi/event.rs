use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::events::EventKind;
use crate::game::GameState;

use super::card::PyCard;
use super::card::snapshot_card;
use super::macros::variant_union;
use super::potion::PyPotion;
use super::potion::snapshot_potion;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindBigFish",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindBigFish;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindTheCleric",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindTheCleric;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindDuplicator",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindDuplicator;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindGoldenShrine",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindGoldenShrine;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindWingStatue",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindWingStatue;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindWorldOfGoop",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindWorldOfGoop;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindLivingWall",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindLivingWall;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindPurifier",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindPurifier;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindShiningLight",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindShiningLight;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindTheSsssserpent",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindTheSsssserpent;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindTransmogrifier",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindTransmogrifier;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindUpgradeShrine",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindUpgradeShrine;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindTheDivineFountain",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindTheDivineFountain;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindTheLab",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindTheLab;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindTheWomanInBlue",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindTheWomanInBlue;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindWheelOfChange",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindWheelOfChange;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindBonfireSpirits",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindBonfireSpirits;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindOminousForge",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindOminousForge;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindFaceTrader",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindFaceTrader;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "EventKindMushrooms",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindMushrooms;

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "EventKindGoldenIdol",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindGoldenIdol {
    pub stage: u8,
}

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "EventKindScrapOoze",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindScrapOoze {
    pub attempts: u8,
}

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "EventKindWeMeetAgain",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindWeMeetAgain {
    pub pick_card: Option<PyCard>,
    pub pick_potion: Option<PyPotion>,
    pub gold_ask: Option<u16>,
}

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "EventKindDeadAdventurer",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindDeadAdventurer {
    pub found_gold: bool,
    pub found_nothing: bool,
    pub found_relic: bool,
    pub searches: u8,
}

#[derive(Debug, Clone)]
pub enum PyEventKind {
    BigFish(PyEventKindBigFish),
    TheCleric(PyEventKindTheCleric),
    Duplicator(PyEventKindDuplicator),
    GoldenShrine(PyEventKindGoldenShrine),
    WingStatue(PyEventKindWingStatue),
    WorldOfGoop(PyEventKindWorldOfGoop),
    LivingWall(PyEventKindLivingWall),
    Purifier(PyEventKindPurifier),
    ShiningLight(PyEventKindShiningLight),
    TheSsssserpent(PyEventKindTheSsssserpent),
    Transmogrifier(PyEventKindTransmogrifier),
    UpgradeShrine(PyEventKindUpgradeShrine),
    TheDivineFountain(PyEventKindTheDivineFountain),
    TheLab(PyEventKindTheLab),
    TheWomanInBlue(PyEventKindTheWomanInBlue),
    WheelOfChange(PyEventKindWheelOfChange),
    BonfireSpirits(PyEventKindBonfireSpirits),
    OminousForge(PyEventKindOminousForge),
    FaceTrader(PyEventKindFaceTrader),
    Mushrooms(PyEventKindMushrooms),
    GoldenIdol(PyEventKindGoldenIdol),
    ScrapOoze(PyEventKindScrapOoze),
    WeMeetAgain(PyEventKindWeMeetAgain),
    DeadAdventurer(PyEventKindDeadAdventurer),
}

variant_union!(PyEventKind {
    BigFish => PyEventKindBigFish,
    TheCleric => PyEventKindTheCleric,
    Duplicator => PyEventKindDuplicator,
    GoldenShrine => PyEventKindGoldenShrine,
    WingStatue => PyEventKindWingStatue,
    WorldOfGoop => PyEventKindWorldOfGoop,
    LivingWall => PyEventKindLivingWall,
    Purifier => PyEventKindPurifier,
    ShiningLight => PyEventKindShiningLight,
    TheSsssserpent => PyEventKindTheSsssserpent,
    Transmogrifier => PyEventKindTransmogrifier,
    UpgradeShrine => PyEventKindUpgradeShrine,
    TheDivineFountain => PyEventKindTheDivineFountain,
    TheLab => PyEventKindTheLab,
    TheWomanInBlue => PyEventKindTheWomanInBlue,
    WheelOfChange => PyEventKindWheelOfChange,
    BonfireSpirits => PyEventKindBonfireSpirits,
    OminousForge => PyEventKindOminousForge,
    FaceTrader => PyEventKindFaceTrader,
    Mushrooms => PyEventKindMushrooms,
    GoldenIdol => PyEventKindGoldenIdol,
    ScrapOoze => PyEventKindScrapOoze,
    WeMeetAgain => PyEventKindWeMeetAgain,
    DeadAdventurer => PyEventKindDeadAdventurer,
});

pub(crate) fn snapshot_event_kind(state: &GameState, kind: EventKind) -> PyEventKind {
    match kind {
        EventKind::BigFish => PyEventKind::BigFish(PyEventKindBigFish),
        EventKind::TheCleric => PyEventKind::TheCleric(PyEventKindTheCleric),
        EventKind::Duplicator => PyEventKind::Duplicator(PyEventKindDuplicator),
        EventKind::GoldenShrine => PyEventKind::GoldenShrine(PyEventKindGoldenShrine),
        EventKind::WingStatue => PyEventKind::WingStatue(PyEventKindWingStatue),
        EventKind::WorldOfGoop => PyEventKind::WorldOfGoop(PyEventKindWorldOfGoop),
        EventKind::LivingWall => PyEventKind::LivingWall(PyEventKindLivingWall),
        EventKind::Purifier => PyEventKind::Purifier(PyEventKindPurifier),
        EventKind::ShiningLight => PyEventKind::ShiningLight(PyEventKindShiningLight),
        EventKind::TheSsssserpent => PyEventKind::TheSsssserpent(PyEventKindTheSsssserpent),
        EventKind::Transmogrifier => PyEventKind::Transmogrifier(PyEventKindTransmogrifier),
        EventKind::UpgradeShrine => PyEventKind::UpgradeShrine(PyEventKindUpgradeShrine),
        EventKind::TheDivineFountain => {
            PyEventKind::TheDivineFountain(PyEventKindTheDivineFountain)
        }
        EventKind::TheLab => PyEventKind::TheLab(PyEventKindTheLab),
        EventKind::TheWomanInBlue => PyEventKind::TheWomanInBlue(PyEventKindTheWomanInBlue),
        EventKind::WheelOfChange => PyEventKind::WheelOfChange(PyEventKindWheelOfChange),
        EventKind::BonfireSpirits => PyEventKind::BonfireSpirits(PyEventKindBonfireSpirits),
        EventKind::OminousForge => PyEventKind::OminousForge(PyEventKindOminousForge),
        EventKind::FaceTrader => PyEventKind::FaceTrader(PyEventKindFaceTrader),
        EventKind::Mushrooms => PyEventKind::Mushrooms(PyEventKindMushrooms),
        EventKind::GoldenIdol { stage } => PyEventKind::GoldenIdol(PyEventKindGoldenIdol { stage }),
        EventKind::ScrapOoze { attempts } => {
            PyEventKind::ScrapOoze(PyEventKindScrapOoze { attempts })
        }
        EventKind::WeMeetAgain {
            id_card,
            id_potion,
            gold_ask,
        } => PyEventKind::WeMeetAgain(PyEventKindWeMeetAgain {
            pick_card: id_card.map(|id| snapshot_card(state, id)),
            pick_potion: id_potion.map(|id| snapshot_potion(&state.entities[id])),
            gold_ask,
        }),
        EventKind::DeadAdventurer {
            found_gold,
            found_nothing,
            found_relic,
            searches,
        } => PyEventKind::DeadAdventurer(PyEventKindDeadAdventurer {
            found_gold,
            found_nothing,
            found_relic,
            searches,
        }),
    }
}
