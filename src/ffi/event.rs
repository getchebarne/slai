use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::events::EventKind;
use crate::game::GameState;

use super::card::PyCard;
use super::card::snapshot_card;
use super::macros::flat_variants;
use super::potion::PyPotion;
use super::potion::snapshot_potion;

flat_variants!(plain PyEventKind {
    BigFish => PyEventKindBigFish as "EventKindBigFish",
    TheCleric => PyEventKindTheCleric as "EventKindTheCleric",
    Duplicator => PyEventKindDuplicator as "EventKindDuplicator",
    GoldenShrine => PyEventKindGoldenShrine as "EventKindGoldenShrine",
    WingStatue => PyEventKindWingStatue as "EventKindWingStatue",
    WorldOfGoop => PyEventKindWorldOfGoop as "EventKindWorldOfGoop",
    LivingWall => PyEventKindLivingWall as "EventKindLivingWall",
    Purifier => PyEventKindPurifier as "EventKindPurifier",
    ShiningLight => PyEventKindShiningLight as "EventKindShiningLight",
    TheSsssserpent => PyEventKindTheSsssserpent as "EventKindTheSsssserpent",
    Transmogrifier => PyEventKindTransmogrifier as "EventKindTransmogrifier",
    UpgradeShrine => PyEventKindUpgradeShrine as "EventKindUpgradeShrine",
    TheDivineFountain => PyEventKindTheDivineFountain as "EventKindTheDivineFountain",
    TheLab => PyEventKindTheLab as "EventKindTheLab",
    TheWomanInBlue => PyEventKindTheWomanInBlue as "EventKindTheWomanInBlue",
    WheelOfChange => PyEventKindWheelOfChange as "EventKindWheelOfChange",
    BonfireSpirits => PyEventKindBonfireSpirits as "EventKindBonfireSpirits",
    OminousForge => PyEventKindOminousForge as "EventKindOminousForge",
    FaceTrader => PyEventKindFaceTrader as "EventKindFaceTrader",
    Mushrooms => PyEventKindMushrooms as "EventKindMushrooms",
    GoldenIdol => PyEventKindGoldenIdol as "EventKindGoldenIdol" { stage: u8 },
    ScrapOoze => PyEventKindScrapOoze as "EventKindScrapOoze" { attempts: u8 },
    WeMeetAgain => PyEventKindWeMeetAgain as "EventKindWeMeetAgain" { pick_card: Option<PyCard>, pick_potion: Option<PyPotion>, gold_ask: Option<u16> },
    DeadAdventurer => PyEventKindDeadAdventurer as "EventKindDeadAdventurer" { found_gold: bool, found_nothing: bool, found_relic: bool, searches: u8 },
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
