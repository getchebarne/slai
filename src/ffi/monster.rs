use pyo3::prelude::*;
use strum::IntoEnumIterator;

use super::macros::mirror_enum;

use super::effect::PyEffect;
use super::effect::snapshot_effect;
use crate::effect::EffectKind;
use crate::effect::PlaySource;
use crate::entity::Intent;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::monsters::move_ultimate;
use crate::types::MonsterEncounter;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::utils::entity_requires_target;
use crate::utils::has_relic;
use crate::utils::scale_attack_damage;
use crate::utils::vuln_factor;
use crate::utils::weak_factor;

use super::modifier::PyModifier;
use super::modifier::snapshot_modifiers;

mirror_enum!(PyMonsterKind from MonsterKind, "MonsterKind", {
    Normal, Elite, Boss,
});

mirror_enum!(PyMonsterName from MonsterName, "MonsterName", {
    AwakenedOne, Cultist, Darkling, Deca, Donu, Exploder, FungiBeast, GremlinFat, GremlinNob,
    GremlinThief, GremlinTsundere, GremlinWarrior, GremlinWizard, Hexaghost, JawWorm, JawWormHard,
    Lagavulin, Looter, LouseDefensive, LouseNormal, Maw, Nemesis, OrbWalker, Reptomancer, Repulsor,
    Sentry, SlaverBlue, SlaverRed, SlimeAcidLarge, SlimeAcidMedium, SlimeAcidSmall, SlimeBoss,
    SlimeSpikeLarge, SlimeSpikeMedium, SlimeSpikeSmall, SnakeDagger, Spiker, SpireGrowth,
    TheGuardian, TimeEater, Transient, WrithingMass, Byrd, Centurion, Chosen, Healer, Mugger,
    ShelledParasite, SnakePlant, Snecko, SphericGuardian, BookOfStabbing, GremlinLeader, Taskmaster,
    BronzeAutomaton, BronzeOrb, Champ, TheCollector, TorchHead, BanditBear, BanditLeader,
    BanditPointy,
});

mirror_enum!(PyMonsterEncounter from MonsterEncounter, "MonsterEncounter", {
    Cultist, JawWorm, TwoLouse, SmallSlimes, BlueSlaver, RedSlaver, Looter, TwoFungiBeasts,
    ThreeLouse, LargeSlime, LotsOfSlimes, GremlinGang, ExordiumThugs, ExordiumWildlife, GremlinNob,
    Lagavulin, ThreeSentries, TheGuardian, Hexaghost, SlimeBoss, ThreeFungiBeasts, LagavulinEvent,
    SphericGuardian, Chosen, ShelledParasite, ThreeByrds, TwoThieves, SnakePlant,
    CenturionAndHealer, Snecko, CultistAndChosen, ThreeCultists, ShelledParasiteAndFungi,
    ChosenAndByrds, SentryAndSphere, GremlinLeader, Slavers, BookOfStabbing, BronzeAutomaton,
    TheCollector, Champ, OrbWalker, ThreeDarklings, ThreeShapes, FourShapes, JawWormHorde, Maw,
    SphereAndTwoShapes, SpireGrowth, Transient, WrithingMass, Nemesis, Reptomancer, AwakenedOne,
    DonuAndDeca, TimeEater,
});

#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    name = "IntentKind",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum PyIntentKind {
    Attack,
    AttackBlock,
    AttackBuff,
    AttackDebuff,
    Block,
    BlockBuff,
    BlockDebuff,
    Buff,
    Debuff,
    DebuffPowerful,
    Escape,
    Sleep,
    Stunned,
    Unknown,
}

#[pymethods]
impl PyIntentKind {
    // Declaration order, which is also int() order
    #[staticmethod]
    fn members() -> Vec<PyIntentKind> {
        PyIntentKind::iter().collect()
    }

    // Hash by raw discriminant so eq_int and hash agree; the derived hash
    // breaks Python's eq/hash contract for int-keyed dicts
    fn __hash__(&self) -> isize {
        *self as isize
    }
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
            Intent::BlockDebuff => Self::BlockDebuff,
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
    pub id: usize,
    pub slot: u8, // The roster index: Monsters act in slot order, and a summon takes its leader's first free post
    pub name: PyMonsterName,
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub half_dead: bool, // At 0 HP, out of every Card's, hit's and Power's reach until its rebirth or its next Death
    pub modifiers: Vec<PyModifier>,
    pub intent: Option<PyIntent>,    // None while Runic Dome hides it
    pub move_effects: Vec<PyEffect>, // The current move's Effect payload; empty when the intent is hidden
    pub move_current: Option<u8>,    // Index into the spawn-rolled moveset; None when hidden
    pub move_history: Vec<u8>, // Selection-ordered move indices; the last entry IS the current move, and under Runic Dome only executed moves
    pub gold_stolen: u16,      // Only relevant for Looters and Muggers
    pub ultimate_used: bool, // The Monster's once-per-combat move has been used: Red Slaver's Entangle, Champ's Anger, Bronze Orb's Stasis, the Collector's Mega Debuff, Time Eater's Haste, Writhing Mass's Implant
    pub replay_target: bool, // A waiting replay aims at this Monster
}

pub(crate) fn snapshot_monsters(state: &GameState) -> Vec<PyMonster> {
    if !state.combat.active {
        return Vec::new();
    }

    // Runic Dome hides every Monster's upcoming move; its history shows only the executed moves
    let move_hidden = has_relic(&state.id_relics, RelicName::RunicDome);
    let character = &state.entities[state.id_character];
    let mods_char = &character.modifiers;
    state
        .combat
        .id_monsters
        .iter()
        .enumerate()
        .filter_map(|(slot, &id_monster)| id_monster.map(|id_monster| (slot, id_monster)))
        .map(|(slot, id_monster)| {
            let monster = &state.entities[id_monster];

            let len_history = monster.monster_move_history_len as usize;
            let move_history = &monster.monster_move_history[..len_history];
            let move_history = if move_hidden {
                // The upcoming move and any move replaced before it executed were never seen
                move_history
                    .iter()
                    .zip(&monster.monster_move_history_exec[..len_history])
                    .filter(|(_, executed)| **executed)
                    .map(|(&move_idx, _)| move_idx)
                    .collect()
            } else {
                move_history.to_vec()
            };

            let (intent, move_effects) = if move_hidden {
                (None, Vec::new())
            } else if let Some(move_idx) = monster.monster_move_current {
                let mv = &monster.monster_moves[move_idx];
                let (base_damage, instances) = match mv.intent {
                    Intent::Attack { damage, instances }
                    | Intent::AttackBlock { damage, instances }
                    | Intent::AttackBuff { damage, instances }
                    | Intent::AttackDebuff { damage, instances } => (Some(damage), Some(instances)),
                    Intent::Block
                    | Intent::BlockBuff
                    | Intent::BlockDebuff
                    | Intent::Buff
                    | Intent::Debuff
                    | Intent::DebuffPowerful
                    | Intent::Escape
                    | Intent::Sleep
                    | Intent::Stunned
                    | Intent::Unknown => (None, None),
                };

                // The Monster's copy of the move holds the hit as chosen: Divider's and the Transient's locked damage, Multi-Stab's and Nom's locked counts
                let move_effects =
                    &monster.monster_move_effects[..monster.monster_move_effects_len as usize];
                let hit = move_effects.iter().find_map(|effect| match effect.kind {
                    EffectKind::DamagePhysical {
                        amount, instances, ..
                    } => Some((amount, instances)),
                    _ => None,
                });
                let base_damage =
                    base_damage.map(|damage| hit.map_or(damage, |(amount, _)| amount));
                let instances =
                    instances.map(|count| hit.map_or(count, |(_, instances)| instances as u8));
                let damage = base_damage.map(|damage| {
                    let str_stacks = if has_modifier(&monster.modifiers, ModifierKind::Strength) {
                        modifier_stacks(&monster.modifiers, ModifierKind::Strength)
                    } else {
                        0
                    };
                    let mut scaled = scale_attack_damage(
                        damage,
                        str_stacks,
                        has_modifier(&monster.modifiers, ModifierKind::DoubleDamage),
                        has_modifier(&monster.modifiers, ModifierKind::PenNib),
                        weak_factor(
                            has_modifier(&monster.modifiers, ModifierKind::Weak),
                            has_relic(&state.id_relics, RelicName::PaperKrane),
                        ),
                        vuln_factor(
                            has_modifier(mods_char, ModifierKind::Vulnerable),
                            has_relic(&state.id_relics, RelicName::OddMushroom),
                        ),
                        has_modifier(mods_char, ModifierKind::Flight),
                    );
                    if has_modifier(mods_char, ModifierKind::Intangible) && scaled > 1 {
                        scaled = 1;
                    }
                    scaled
                });

                let move_effects = move_effects.iter().map(snapshot_effect).collect();
                (
                    Some(PyIntent {
                        kind: mv.intent.into(),
                        damage,
                        instances,
                    }),
                    move_effects,
                )
            } else {
                (
                    Some(PyIntent {
                        kind: PyIntentKind::Unknown,
                        damage: None,
                        instances: None,
                    }),
                    Vec::new(),
                )
            };

            // A replay aims at its first play's target; a draw-top play's random target stays hidden until it hits
            let replay_target = state.card_play_queue.iter().any(|card_play| {
                matches!(
                    card_play.play_source,
                    PlaySource::Replay | PlaySource::ReplayCopy { .. }
                ) && entity_requires_target(&state.entities[card_play.id_card])
                    && card_play.id_target == Some(id_monster)
            });

            PyMonster {
                id: id_monster,
                slot: slot as u8,
                name: monster.monster_name.into(),
                health: monster.vitals.health,
                health_max: monster.vitals.health_max,
                block: monster.vitals.block,
                half_dead: monster.monster_half_dead,
                modifiers: snapshot_modifiers(&monster.modifiers),
                intent,
                move_effects,
                move_current: if move_hidden {
                    None
                } else {
                    monster.monster_move_current.map(|idx| idx as u8)
                },
                move_history,
                gold_stolen: monster.monster_gold_stolen,
                ultimate_used: move_ultimate(monster.monster_name)
                    .is_some_and(|idx_move| monster.monster_move_uses[idx_move] > 0),
                replay_target,
            }
        })
        .collect()
}
