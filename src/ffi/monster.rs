use pyo3::prelude::*;

use super::macros::mirror_enum;

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

mirror_enum!(PyMonsterName from MonsterName, "MonsterName", from_py_object, {
    Cultist, FungiBeast, GremlinFat, GremlinNob, GremlinThief, GremlinTsundere, GremlinWarrior,
    GremlinWizard, Hexaghost, JawWorm, Lagavulin, Looter, LouseDefensive, LouseNormal, Sentry,
    SlaverBlue, SlaverRed, SlimeAcidLarge, SlimeAcidMedium, SlimeAcidSmall, SlimeBoss,
    SlimeSpikeLarge, SlimeSpikeMedium, SlimeSpikeSmall, TheGuardian, Byrd, Centurion, Chosen,
    Healer, Mugger, ShelledParasite, SnakePlant, Snecko, SphericGuardian,
});

mirror_enum!(PyMonsterEncounter from MonsterEncounter, "MonsterEncounter", from_py_object, {
    Cultist, JawWorm, TwoLouse, SmallSlimes, BlueSlaver, RedSlaver, Looter, TwoFungiBeasts,
    ThreeLouse, LargeSlime, LotsOfSlimes, GremlinGang, ExordiumThugs, ExordiumWildlife,
    GremlinNob, Lagavulin, ThreeSentries, TheGuardian, Hexaghost, SlimeBoss, ThreeFungiBeasts,
    SphericGuardian, Chosen, ShelledParasite, ThreeByrds, TwoThieves, SnakePlant,
    CenturionAndHealer, Snecko, CultistAndChosen, ThreeCultists, ShelledParasiteAndFungi,
    ChosenAndByrds, SentryAndSphere,
});

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

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Intent",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyIntent {
    pub kind: PyIntentKind,
    pub damage: Option<u16>,
    pub instances: Option<u8>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Monster",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyMonster {
    pub name: PyMonsterName,
    pub display_name: String,
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub modifiers: Vec<PyModifier>,
    pub intent: PyIntent,
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
            Self::Byrd => "Byrd",
            Self::Centurion => "Centurion",
            Self::Chosen => "Chosen",
            Self::Healer => "Mystic",
            Self::Mugger => "Mugger",
            Self::ShelledParasite => "Shelled Parasite",
            Self::SnakePlant => "Snake Plant",
            Self::Snecko => "Snecko",
            Self::SphericGuardian => "Spheric Guardian",
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
