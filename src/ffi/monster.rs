use pyo3::prelude::*;

use crate::entity::Intent;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::types::Mode;
use crate::types::MonsterEncounter;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::mode_top;
use crate::utils::scale_attack_damage;
use crate::utils::vuln_factor;
use crate::utils::weak_factor;

use super::modifier::PyModifier;
use super::modifier::snapshot_modifiers;

#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    name = "MonsterName",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyMonsterName {
    Cultist,
    FungiBeast,
    GremlinFat,
    GremlinNob,
    GremlinThief,
    GremlinTsundere,
    GremlinWarrior,
    GremlinWizard,
    Hexaghost,
    JawWorm,
    Lagavulin,
    Looter,
    LouseDefensive,
    LouseNormal,
    Sentry,
    SlaverBlue,
    SlaverRed,
    SlimeAcidLarge,
    SlimeAcidMedium,
    SlimeAcidSmall,
    SlimeBoss,
    SlimeSpikeLarge,
    SlimeSpikeMedium,
    SlimeSpikeSmall,
    TheGuardian,
}

impl From<MonsterName> for PyMonsterName {
    fn from(name: MonsterName) -> Self {
        match name {
            MonsterName::Cultist => Self::Cultist,
            MonsterName::FungiBeast => Self::FungiBeast,
            MonsterName::GremlinFat => Self::GremlinFat,
            MonsterName::GremlinNob => Self::GremlinNob,
            MonsterName::GremlinThief => Self::GremlinThief,
            MonsterName::GremlinTsundere => Self::GremlinTsundere,
            MonsterName::GremlinWarrior => Self::GremlinWarrior,
            MonsterName::GremlinWizard => Self::GremlinWizard,
            MonsterName::Hexaghost => Self::Hexaghost,
            MonsterName::JawWorm => Self::JawWorm,
            MonsterName::Lagavulin => Self::Lagavulin,
            MonsterName::Looter => Self::Looter,
            MonsterName::LouseDefensive => Self::LouseDefensive,
            MonsterName::LouseNormal => Self::LouseNormal,
            MonsterName::Sentry => Self::Sentry,
            MonsterName::SlaverBlue => Self::SlaverBlue,
            MonsterName::SlaverRed => Self::SlaverRed,
            MonsterName::SlimeAcidLarge => Self::SlimeAcidLarge,
            MonsterName::SlimeAcidMedium => Self::SlimeAcidMedium,
            MonsterName::SlimeAcidSmall => Self::SlimeAcidSmall,
            MonsterName::SlimeBoss => Self::SlimeBoss,
            MonsterName::SlimeSpikeLarge => Self::SlimeSpikeLarge,
            MonsterName::SlimeSpikeMedium => Self::SlimeSpikeMedium,
            MonsterName::SlimeSpikeSmall => Self::SlimeSpikeSmall,
            MonsterName::TheGuardian => Self::TheGuardian,
        }
    }
}

#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    name = "MonsterEncounter",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyMonsterEncounter {
    Cultist,
    JawWorm,
    TwoLouse,
    SmallSlimes,
    BlueSlaver,
    RedSlaver,
    Looter,
    TwoFungiBeasts,
    ThreeLouse,
    LargeSlime,
    LotsOfSlimes,
    GremlinGang,
    ExordiumThugs,
    ExordiumWildlife,
    GremlinNob,
    Lagavulin,
    ThreeSentries,
    TheGuardian,
    Hexaghost,
    SlimeBoss,
    ThreeFungiBeasts,
}

impl From<MonsterEncounter> for PyMonsterEncounter {
    fn from(e: MonsterEncounter) -> Self {
        match e {
            MonsterEncounter::Cultist => Self::Cultist,
            MonsterEncounter::JawWorm => Self::JawWorm,
            MonsterEncounter::TwoLouse => Self::TwoLouse,
            MonsterEncounter::SmallSlimes => Self::SmallSlimes,
            MonsterEncounter::BlueSlaver => Self::BlueSlaver,
            MonsterEncounter::RedSlaver => Self::RedSlaver,
            MonsterEncounter::Looter => Self::Looter,
            MonsterEncounter::TwoFungiBeasts => Self::TwoFungiBeasts,
            MonsterEncounter::ThreeLouse => Self::ThreeLouse,
            MonsterEncounter::LargeSlime => Self::LargeSlime,
            MonsterEncounter::LotsOfSlimes => Self::LotsOfSlimes,
            MonsterEncounter::GremlinGang => Self::GremlinGang,
            MonsterEncounter::ExordiumThugs => Self::ExordiumThugs,
            MonsterEncounter::ExordiumWildlife => Self::ExordiumWildlife,
            MonsterEncounter::GremlinNob => Self::GremlinNob,
            MonsterEncounter::Lagavulin => Self::Lagavulin,
            MonsterEncounter::ThreeSentries => Self::ThreeSentries,
            MonsterEncounter::TheGuardian => Self::TheGuardian,
            MonsterEncounter::Hexaghost => Self::Hexaghost,
            MonsterEncounter::SlimeBoss => Self::SlimeBoss,
            MonsterEncounter::ThreeFungiBeasts => Self::ThreeFungiBeasts,
        }
    }
}

#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    name = "IntentKind",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyIntentKind {
    Attack,
    AttackBlock,
    AttackBuff,
    AttackDebuff,
    Block,
    BlockBuff,
    Buff,
    Debuff,
    DebuffPowerful,
    Escape,
    Sleep,
    Stunned,
    Unknown,
}

impl From<Intent> for PyIntentKind {
    fn from(intent: Intent) -> Self {
        match intent {
            Intent::Attack { .. } => Self::Attack,
            Intent::AttackBlock { .. } => Self::AttackBlock,
            Intent::AttackBuff { .. } => Self::AttackBuff,
            Intent::AttackDebuff { .. } => Self::AttackDebuff,
            Intent::Block => Self::Block,
            Intent::BlockBuff => Self::BlockBuff,
            Intent::Buff => Self::Buff,
            Intent::Debuff => Self::Debuff,
            Intent::DebuffPowerful => Self::DebuffPowerful,
            Intent::Escape => Self::Escape,
            Intent::Sleep => Self::Sleep,
            Intent::Stunned => Self::Stunned,
            Intent::Unknown => Self::Unknown,
        }
    }
}

#[pyclass(from_py_object, frozen, get_all, name = "Intent", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyIntent {
    pub kind: PyIntentKind,
    pub damage: Option<u16>,
    pub instances: Option<u8>,
}

#[pymethods]
impl PyIntent {
    #[new]
    fn new(kind: PyIntentKind, damage: Option<u16>, instances: Option<u8>) -> Self {
        Self {
            kind,
            damage,
            instances,
        }
    }
}

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "Monster",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyMonster {
    pub name: PyMonsterName,
    pub display_name: String,
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub modifiers: Vec<PyModifier>,
    pub intent: PyIntent,
}

#[pymethods]
impl PyMonster {
    #[new]
    fn new(
        name: PyMonsterName,
        display_name: String,
        health: u16,
        health_max: u16,
        block: u16,
        modifiers: Vec<PyModifier>,
        intent: PyIntent,
    ) -> Self {
        Self {
            name,
            display_name,
            health,
            health_max,
            block,
            modifiers,
            intent,
        }
    }
}

impl MonsterName {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cultist => "Cultist",
            Self::FungiBeast => "Fungi Beast",
            Self::GremlinFat => "Fat Gremlin",
            Self::GremlinNob => "Gremlin Nob",
            Self::GremlinThief => "Sneaky Gremlin",
            Self::GremlinTsundere => "Shield Gremlin",
            Self::GremlinWarrior => "Mad Gremlin",
            Self::GremlinWizard => "Gremlin Wizard",
            Self::Hexaghost => "Hexaghost",
            Self::JawWorm => "Jaw Worm",
            Self::Lagavulin => "Lagavulin",
            Self::Looter => "Looter",
            Self::LouseDefensive => "Green Louse",
            Self::LouseNormal => "Red Louse",
            Self::Sentry => "Sentry",
            Self::SlaverBlue => "Blue Slaver",
            Self::SlaverRed => "Red Slaver",
            Self::SlimeAcidLarge => "Acid Slime (L)",
            Self::SlimeAcidMedium => "Acid Slime (M)",
            Self::SlimeAcidSmall => "Acid Slime (S)",
            Self::SlimeBoss => "Slime Boss",
            Self::SlimeSpikeLarge => "Spike Slime (L)",
            Self::SlimeSpikeMedium => "Spike Slime (M)",
            Self::SlimeSpikeSmall => "Spike Slime (S)",
            Self::TheGuardian => "The Guardian",
        }
    }
}

impl MonsterEncounter {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cultist => "Cultist",
            Self::JawWorm => "Jaw Worm",
            Self::TwoLouse => "2 Louse",
            Self::SmallSlimes => "Small Slimes",
            Self::BlueSlaver => "Blue Slaver",
            Self::RedSlaver => "Red Slaver",
            Self::Looter => "Looter",
            Self::TwoFungiBeasts => "2 Fungi Beasts",
            Self::ThreeFungiBeasts => "3 Fungi Beasts",
            Self::ThreeLouse => "3 Louse",
            Self::LargeSlime => "Large Slime",
            Self::LotsOfSlimes => "Lots of Slimes",
            Self::GremlinGang => "Gremlin Gang",
            Self::ExordiumThugs => "Exordium Thugs",
            Self::ExordiumWildlife => "Exordium Wildlife",
            Self::GremlinNob => "Gremlin Nob",
            Self::Lagavulin => "Lagavulin",
            Self::ThreeSentries => "3 Sentries",
            Self::TheGuardian => "The Guardian",
            Self::Hexaghost => "Hexaghost",
            Self::SlimeBoss => "Slime Boss",
        }
    }
}

pub(crate) fn snapshot_monsters(state: &GameState) -> Vec<PyMonster> {
    let Mode::Combat { id_monsters, .. } = mode_top(&state.mode_stack) else {
        return Vec::new();
    };
    let character = &state.entities[state.id_character];
    let mods_char = &character.modifiers;
    id_monsters
        .iter()
        .flatten()
        .copied()
        .map(|id_monster| {
            let m = &state.entities[id_monster];

            let intent = if let Some(move_idx) = m.monster_move_current {
                let mv = &m.monster_moves[move_idx];
                let (base_damage, instances) = match mv.intent {
                    Intent::Attack { damage, instances }
                    | Intent::AttackBlock { damage, instances }
                    | Intent::AttackBuff { damage, instances }
                    | Intent::AttackDebuff { damage, instances } => (Some(damage), Some(instances)),
                    Intent::Block
                    | Intent::BlockBuff
                    | Intent::Buff
                    | Intent::Debuff
                    | Intent::DebuffPowerful
                    | Intent::Escape
                    | Intent::Sleep
                    | Intent::Stunned
                    | Intent::Unknown => (None, None),
                };

                let damage = base_damage.map(|d| {
                    let str_stacks = if has_modifier(&m.modifiers, ModifierKind::Strength) {
                        modifier_stacks(&m.modifiers, ModifierKind::Strength)
                    } else {
                        0
                    };
                    let mut scaled = scale_attack_damage(
                        d,
                        str_stacks,
                        weak_factor(
                            has_modifier(&m.modifiers, ModifierKind::Weak),
                            has_relic(&state.id_relics, RelicName::PaperKrane),
                        ),
                        vuln_factor(
                            has_modifier(mods_char, ModifierKind::Vulnerable),
                            has_relic(&state.id_relics, RelicName::OddMushroom),
                        ),
                    );
                    if has_modifier(mods_char, ModifierKind::Intangible) && scaled > 1 {
                        scaled = 1;
                    }
                    scaled
                });

                PyIntent {
                    kind: mv.intent.into(),
                    damage,
                    instances,
                }
            } else {
                PyIntent {
                    kind: PyIntentKind::Unknown,
                    damage: None,
                    instances: None,
                }
            };

            PyMonster {
                name: m.monster_name.into(),
                display_name: m.monster_name.as_str().to_string(),
                health: m.vitals.health,
                health_max: m.vitals.health_max,
                block: m.vitals.block,
                modifiers: snapshot_modifiers(&m.modifiers),
                intent,
            }
        })
        .collect()
}
