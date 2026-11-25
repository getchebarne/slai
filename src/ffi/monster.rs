use pyo3::prelude::*;

use super::macros::mirror_enum;

use super::effect::id_target_shown;
use crate::consts::FACTOR_BACK_ATTACK;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::PlaySource;
use crate::entity::Entity;
use crate::entity::IntentKind;
use crate::entity::Move;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::monsters::monster_template;
use crate::monsters::move_stages;
use crate::monsters::move_ultimate;
use crate::monsters::tier_at;
use crate::types::MonsterEncounter;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::scale_attack_damage;
use crate::utils::slow_factor;
use crate::utils::vuln_factor;
use crate::utils::weak_factor;

use super::modifier::PyModifier;
use super::modifier::snapshot_modifiers;

mirror_enum!(PyMonsterKind from MonsterKind, "MonsterKind", {
    Normal, Elite, Boss,
});

mirror_enum!(PyMonsterName from MonsterName, "MonsterName", {
    AwakenedOne, CorruptHeart, Cultist, Darkling, Deca, Donu, Exploder, FungiBeast, GiantHead,
    GremlinFat, GremlinNob, GremlinThief, GremlinTsundere, GremlinWarrior, GremlinWizard, Hexaghost,
    JawWorm, JawWormHard, Lagavulin, Looter, LouseDefensive, LouseNormal, Maw, Nemesis, OrbWalker,
    Reptomancer, Repulsor, Sentry, SlaverBlue, SlaverRed, SlimeAcidLarge, SlimeAcidMedium,
    SlimeAcidSmall, SlimeBoss, SlimeSpikeLarge, SlimeSpikeMedium, SlimeSpikeSmall, SnakeDagger,
    Spiker, SpireGrowth, SpireShield, SpireSpear, TheGuardian, TimeEater, Transient, WrithingMass,
    Byrd, Centurion, Chosen, Healer, Mugger, ShelledParasite, SnakePlant, Snecko, SphericGuardian,
    BookOfStabbing, GremlinLeader, Taskmaster, BronzeAutomaton, BronzeOrb, Champ, TheCollector,
    TorchHead, BanditBear, BanditLeader, BanditPointy,
});

mirror_enum!(PyMonsterEncounter from MonsterEncounter, "MonsterEncounter", {
    Cultist, JawWorm, TwoLouse, SmallSlimes, BlueSlaver, RedSlaver, Looter, TwoFungiBeasts,
    ThreeLouse, LargeSlime, LotsOfSlimes, GremlinGang, ExordiumThugs, ExordiumWildlife, GremlinNob,
    Lagavulin, ThreeSentries, TheGuardian, Hexaghost, SlimeBoss, ThreeFungiBeasts, LagavulinEvent,
    SphericGuardian, Chosen, ShelledParasite, ThreeByrds, TwoThieves, SnakePlant,
    CenturionAndHealer, Snecko, CultistAndChosen, ThreeCultists, ShelledParasiteAndFungi,
    ChosenAndByrds, SentryAndSphere, GremlinLeader, Slavers, BookOfStabbing, BronzeAutomaton,
    TheCollector, Champ, OrbWalker, ThreeDarklings, ThreeShapes, FourShapes, JawWormHorde, Maw,
    SphereAndTwoShapes, SpireGrowth, Transient, WrithingMass, GiantHead, Nemesis, Reptomancer,
    AwakenedOne, DonuAndDeca, TimeEater, ShieldAndSpear, CorruptHeart,
});

mirror_enum!(PyIntentKind from IntentKind, "IntentKind", {
    Attack, AttackBlock, AttackBuff, AttackDebuff, Block, BlockBuff, BlockDebuff, Buff, Debuff,
    DebuffPowerful, Escape, Sleep, Stunned, Unknown,
});

#[pyclass(eq, hash, frozen, get_all, name = "Intent", module = "slai._slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyIntent {
    pub kind: PyIntentKind,
    pub damage: Option<u16>,
    pub instances: Option<u8>,
}

#[pyclass(eq, hash, frozen, get_all, name = "Monster", module = "slai._slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyMonster {
    pub id: usize,
    pub slot: u8, // The roster index: Monsters act in slot order, and a summon takes its leader's first free post
    pub name: PyMonsterName,
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub half_dead: bool, // At 0 HP, out of every Card's, hit's and Power's reach until its rebirth or its next Death
    pub modifiers: Vec<PyModifier>,
    pub intent: Option<PyIntent>, // None while Runic Dome hides it
    pub moveset: Option<u8>, // Its template's moveset index; None until a spawn-rolled move shows
    pub moves_shown: Vec<u8>, // Moveset indices of the moves showing its intent; empty while hidden
    pub move_history: Vec<u8>, // Moves before the current one; under Runic Dome only executed ones
    pub gold_stolen: u16,    // Only relevant for Looters and Muggers
    pub ultimate_used: bool, // The Monster's once-per-combat move has been used: Red Slaver's Entangle, Champ's Anger, Bronze Orb's Stasis, the Collector's Mega Debuff, Time Eater's Haste, Writhing Mass's Implant
    pub replay_target: bool, // A waiting replay aims at this Monster
}

// The DamagePhysical an attack's intent shows: its amount and hit count; None for any other move
pub(crate) fn attack_hit(effects: &[Effect]) -> Option<(u16, u8)> {
    effects.iter().find_map(|effect| match effect.kind {
        EffectKind::DamagePhysical {
            amount, instances, ..
        } => Some((amount, instances as u8)),
        _ => None,
    })
}

// The intent a move shows on screen: its kind, and an attack's scaled damage and hit count
fn intent_shown(
    state: &GameState,
    monster: &Entity,
    intent: IntentKind,
    effects: &[Effect],
) -> PyIntent {
    let (base_damage, instances) = attack_hit(effects).unzip();
    let mods_char = &state.entities[state.id_character].modifiers;
    let damage = base_damage.map(|damage| {
        let str_stacks = if has_modifier(&monster.modifiers, ModifierKind::Strength) {
            modifier_stacks(&monster.modifiers, ModifierKind::Strength)
        } else {
            0
        };
        let mut unrounded = scale_attack_damage(
            damage,
            str_stacks,
            has_modifier(&monster.modifiers, ModifierKind::DoubleDamage),
            has_modifier(&monster.modifiers, ModifierKind::PenNib),
            weak_factor(
                has_modifier(&monster.modifiers, ModifierKind::Weak),
                has_relic(&state.id_relics, RelicName::PaperKrane),
            ),
            slow_factor(modifier_stacks(mods_char, ModifierKind::Slow)),
            vuln_factor(
                has_modifier(mods_char, ModifierKind::Vulnerable),
                has_relic(&state.id_relics, RelicName::OddMushroom),
            ),
            has_modifier(mods_char, ModifierKind::Flight),
        );

        // Back Attack multiplies the unrounded value here and the truncated one in the hit, so this intent can read 1 above the hit
        if has_modifier(&monster.modifiers, ModifierKind::BackAttack) {
            unrounded *= FACTOR_BACK_ATTACK;
        }
        let mut scaled = unrounded as u16;
        if has_modifier(mods_char, ModifierKind::Intangible) && scaled > 1 {
            scaled = 1;
        }

        scaled
    });

    PyIntent {
        kind: intent.into(),
        damage,
        instances,
    }
}

// A spawn-rolled move: an entry the tier's move tables don't all hold alike
fn move_spawn_rolled(movesets: &[&[Move]], idx_move: usize) -> bool {
    movesets
        .iter()
        .any(|moveset| moveset[idx_move] != movesets[0][idx_move])
}

pub(crate) fn snapshot_monsters(state: &GameState) -> Vec<PyMonster> {
    // Runic Dome hides every Monster's upcoming move; its history shows only the executed moves
    let move_hidden = has_relic(&state.id_relics, RelicName::RunicDome);
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
            let move_history_exec = &monster.monster_move_history_exec[..len_history];
            let move_history = if move_hidden {
                // The upcoming move and any move replaced before it executed were never seen
                move_history
                    .iter()
                    .zip(move_history_exec)
                    .filter(|(_, executed)| **executed)
                    .map(|(&move_idx, _)| move_idx)
                    .collect()
            } else if move_history_exec.last() == Some(&false) {
                // The last entry is the upcoming move until it executes
                move_history[..len_history - 1].to_vec()
            } else {
                move_history.to_vec()
            };

            // The Monster's once-per-Combat move, once it has been used
            let idx_ultimate_used = move_ultimate(monster.monster_name)
                .filter(|&idx_move| monster.monster_move_uses[idx_move] > 0);

            let (intent, moves_shown) = if move_hidden {
                (None, Vec::new())
            } else if let Some(idx_current) = monster.monster_move_current {
                // The Monster's copy of its current move holds the hit as chosen
                let moves = monster.monster_moves;
                let effects_current =
                    &monster.monster_move_effects[..monster.monster_move_effects_len as usize];
                let intent =
                    intent_shown(state, monster, moves[idx_current].intent, effects_current);

                // Never listed with the current move: its other stages, a used once-per-Combat move
                let stages = move_stages(monster.monster_name)
                    .filter(|stages| stages.contains(&idx_current));
                let moves_shown = moves
                    .iter()
                    .enumerate()
                    .filter(|&(idx_move, mv)| {
                        let effects = &mv.effects[..mv.effects_len as usize];

                        idx_move == idx_current
                            || idx_ultimate_used != Some(idx_move)
                                && !stages
                                    .as_ref()
                                    .is_some_and(|stages| stages.contains(&idx_move))
                                && intent_shown(state, monster, mv.intent, effects) == intent
                    })
                    .map(|(idx_move, _)| idx_move as u8)
                    .collect();

                (Some(intent), moves_shown)
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

            // The moveset is told once a spawn-rolled move is in the history or shows as current
            let template = monster_template(monster.monster_name);
            let movesets =
                tier_at(template.move_tiers, state.ascension).expect("`move_tiers` is never empty");
            let moveset_told = movesets.len() == 1
                || move_history
                    .iter()
                    .map(|&idx_move| idx_move as usize)
                    .chain(monster.monster_move_current.filter(|_| !move_hidden))
                    .any(|idx_move| move_spawn_rolled(movesets, idx_move));
            let moveset = movesets
                .iter()
                .position(|moveset| *moveset == monster.monster_moves)
                .filter(|_| moveset_told)
                .map(|idx_moveset| idx_moveset as u8);

            // A replay aims at its first play's target; a draw-top play's random target stays hidden until it hits
            let replay_target = state.card_play_queue.iter().any(|card_play| {
                matches!(
                    card_play.play_source,
                    PlaySource::Replay | PlaySource::ReplayCopy { .. }
                ) && id_target_shown(state, card_play) == Some(id_monster)
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
                moveset,
                moves_shown,
                move_history,
                gold_stolen: monster.monster_gold_stolen,
                ultimate_used: idx_ultimate_used.is_some(),
                replay_target,
            }
        })
        .collect()
}
