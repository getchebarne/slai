use pyo3::prelude::*;

use super::macros::mirror_enum;

use crate::entity::Intent;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::types::MonsterEncounter;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::scale_attack_damage;
use crate::utils::vuln_factor;
use crate::utils::weak_factor;

use super::modifier::PyModifier;
use super::modifier::snapshot_modifiers;

mirror_enum!(PyMonsterKind from MonsterKind, "MonsterKind", skip_from_py_object, {
    Normal, Elite, Boss,
});

mirror_enum!(PyMonsterName from MonsterName, "MonsterName", from_py_object, {
    Cultist, FungiBeast, GremlinFat, GremlinNob, GremlinThief, GremlinTsundere, GremlinWarrior,
    GremlinWizard, Hexaghost, JawWorm, Lagavulin, Looter, LouseDefensive, LouseNormal, Sentry,
    SlaverBlue, SlaverRed, SlimeAcidLarge, SlimeAcidMedium, SlimeAcidSmall, SlimeBoss,
    SlimeSpikeLarge, SlimeSpikeMedium, SlimeSpikeSmall, TheGuardian, Byrd, Centurion, Chosen,
    Healer, Mugger, ShelledParasite, SnakePlant, Snecko, SphericGuardian, BookOfStabbing,
    GremlinLeader, Taskmaster, BronzeAutomaton, BronzeOrb, Champ, TheCollector, TorchHead,
    BanditBear, BanditLeader, BanditPointy,
});

mirror_enum!(PyMonsterEncounter from MonsterEncounter, "MonsterEncounter", from_py_object, {
    Cultist, JawWorm, TwoLouse, SmallSlimes, BlueSlaver, RedSlaver, Looter, TwoFungiBeasts,
    ThreeLouse, LargeSlime, LotsOfSlimes, GremlinGang, ExordiumThugs, ExordiumWildlife,
    GremlinNob, Lagavulin, ThreeSentries, TheGuardian, Hexaghost, SlimeBoss, ThreeFungiBeasts,
    SphericGuardian, Chosen, ShelledParasite, ThreeByrds, TwoThieves, SnakePlant,
    CenturionAndHealer, Snecko, CultistAndChosen, ThreeCultists, ShelledParasiteAndFungi,
    ChosenAndByrds, SentryAndSphere, GremlinLeader, Slavers, BookOfStabbing, BronzeAutomaton,
    TheCollector, Champ,
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
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub modifiers: Vec<PyModifier>,
    pub intent: PyIntent,
    // Gold held by Looter/Mugger thievery: returned on kill, lost on escape
    pub stolen_gold: u16,
}

pub(crate) fn snapshot_monsters(state: &GameState) -> Vec<PyMonster> {
    if !state.combat.active {
        return Vec::new();
    }
    let character = &state.entities[state.id_character];
    let mods_char = &character.modifiers;
    state
        .combat
        .id_monsters
        .iter()
        .flatten()
        .copied()
        .map(|id_monster| {
            let monster = &state.entities[id_monster];

            let intent = if let Some(move_idx) = monster.monster_move_current {
                let mv = &monster.monster_moves[move_idx];
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

                // Divider-style locked damage replaces the template before scaling
                let base_damage = base_damage
                    .map(|damage| monster.monster_move_damage_override.unwrap_or(damage));
                let damage = base_damage.map(|damage| {
                    let str_stacks = if has_modifier(&monster.modifiers, ModifierKind::Strength) {
                        modifier_stacks(&monster.modifiers, ModifierKind::Strength)
                    } else {
                        0
                    };
                    let mut scaled = scale_attack_damage(
                        damage,
                        str_stacks,
                        weak_factor(
                            has_modifier(&monster.modifiers, ModifierKind::Weak),
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
                name: monster.monster_name.into(),
                health: monster.vitals.health,
                health_max: monster.vitals.health_max,
                block: monster.vitals.block,
                modifiers: snapshot_modifiers(&monster.modifiers),
                intent,
                stolen_gold: monster.monster_stolen_gold,
            }
        })
        .collect()
}
