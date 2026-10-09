use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::potions::remove_potion;
use crate::types::Combat;
use crate::types::DeltaSign;
use crate::types::MonsterName;
use crate::types::PotionName;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;
use crate::utils::release_stasis_card;
use crate::utils::resolve_health_fraction;

pub fn process_effect_death(id_target: Option<usize>, state: &mut GameState, with_leader: bool) {
    let id_target = id_target.expect("Death requires id_target");

    // Death is not re-entrant: a second pass would re-fire the on-death triggers
    if state.entities[id_target].dead {
        return;
    }

    // Character death: clear pending work, mark dead, signal game over
    if id_target == state.id_character {
        // Fairy in a Bottle: consumed to revive by its own heal (doubled with Sacred Bark); checked before Lizard Tail
        if let Some(id_potion) = state
            .id_potions
            .iter()
            .copied()
            .find(|&id| state.entities[id].potion_name == PotionName::Fairy)
        {
            remove_potion(&mut state.id_potions, id_potion);
            let EffectKind::HealthDelta { amount, .. } =
                state.entities[id_potion].potion_effects[0].kind
            else {
                unreachable!("Fairy in a Bottle revives through its HealthDelta");
            };
            let vitals = &mut state.entities[state.id_character].vitals;
            vitals.health =
                resolve_health_fraction(vitals.health_max, state.event.health_max_at_open, amount);
            return;
        }

        // Lizard Tail: once per run, survive at half max HP instead
        if let Some(id_relic) = state.id_relics[RelicName::LizardTail as usize]
            && !state.entities[id_relic].relic_used_up
        {
            // Mark as used up
            state.entities[id_relic].relic_used_up = true;

            // Set HP to half-of-max
            let vitals = &mut state.entities[state.id_character].vitals;
            vitals.health = (vitals.health_max / 2).max(1);
            return;
        }

        // Mark Character as dead, set `game_over` flag, and clear the queued work: effects, waiting Card plays, turn phases
        state.entities[state.id_character].dead = true;
        state.game_over = true;
        state.effect_queue.clear();
        state.card_play_queue.clear();
        state.phase_queue.clear();
        return;
    }

    // Monster-death path
    assert!(
        state.combat.active,
        "Monster death outside the Combat frame"
    );
    let Combat {
        id_monsters,
        id_card_hand,
        id_card_stasis,
        gold_stolen: gold_stolen_total,
        this_combat_monster_died,
        ..
    } = &mut state.combat;
    *this_combat_monster_died = true;
    let id_character = state.id_character;

    // Mark the corpse dead, drop it from the live roster, and check if combat continues
    state.entities[id_target].dead = true;
    let slot = id_monsters.iter().position(|slot| *slot == Some(id_target));
    if let Some(slot) = slot {
        id_monsters[slot] = None; // Clear from `id_monsters` Vec
    }

    // Calculate if there're any Monsters left alive
    let any_alive = id_monsters.iter().any(|slot| slot.is_some());

    // Stolen gold is staged as its own reward item; claimed at reward screen; skips Golden Idol
    *gold_stolen_total += state.entities[id_target].monster_gold_stolen;
    state.entities[id_target].monster_gold_stolen = 0;

    if !any_alive {
        // The last kill ends the combat: no waiting Card play or turn phase starts, and only the queued heals, block and damage resolve, Hand of Greed and Ritual Dagger with them
        state.card_play_queue.clear();
        state.phase_queue.clear();
        state.effect_queue.retain(|e| {
            matches!(
                e.kind,
                EffectKind::HealthDelta { .. }
                    | EffectKind::LifestealHeal
                    | EffectKind::BlockGain { .. }
                    | EffectKind::DamageDeal { .. }
                    | EffectKind::DamagePhysical { .. }
                    | EffectKind::DamagePhysicalIfPoisoned { .. }
                    | EffectKind::DamageFinisher { .. }
                    | EffectKind::DamageFlechettes { .. }
                    | EffectKind::DamageMindBlast { .. }
                    | EffectKind::PoisonTick { .. }
                    | EffectKind::HandOfGreedProc { .. }
                    | EffectKind::RitualDaggerProc { .. }
            )
        });

        // The combat closes once that work and all it sets off have resolved
        state.phase_queue.push_back(Effect {
            kind: EffectKind::CombatEnd {
                escaped_character: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
        return;
    }

    let target = &state.entities[id_target];

    // A leader's death drains its minions: survivors that are all Minions escape or die
    let drains_minions = !has_modifier(&target.modifiers, ModifierKind::Minion)
        && id_monsters
            .iter()
            .flatten()
            .all(|&id| has_modifier(&state.entities[id].modifiers, ModifierKind::Minion));
    let gremlins_flee = drains_minions && target.monster_name == MonsterName::GremlinLeader;
    let minions_fall = drains_minions && !gremlins_flee;

    // A leader other than the Gremlin Leader takes its minions down at once
    if minions_fall {
        for id_monster in id_monsters.iter() {
            if let Some(id) = *id_monster {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::Death { with_leader: true },
                    id_source: None,
                    target: Target::Direct(Some(id)),
                });
            }
        }
    }

    // The other on-death effects, staged in order
    state.effect_buf.clear();

    // Stasis: the hostage comes back ahead of the Relics' effects; a death that ends the combat leaves it to the combat reset
    if let Some(slot) = slot {
        release_stasis_card(
            slot,
            id_card_stasis,
            id_card_hand,
            &state.entities,
            &mut state.effect_buf,
        );
    }

    // CorpseExplosion: max_health to each survivor; no source scaling, no Envenom proc
    if has_modifier(&target.modifiers, ModifierKind::CorpseExplosion) {
        let damage = target.vitals.health_max.saturating_mul(
            modifier_stacks(&target.modifiers, ModifierKind::CorpseExplosion).max(0) as u16,
        );
        for id_monster in id_monsters.iter() {
            if let Some(id) = *id_monster {
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal {
                        amount: damage,
                        lifesteal: false,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id)),
                });
            }
        }
    }

    // Gremlin Horn: a Monster's death grants 1 energy and draws 1
    if has_relic(&state.id_relics, RelicName::GremlinHorn) {
        state.effect_buf.push(Effect {
            kind: EffectKind::EnergyDelta {
                sign: DeltaSign::Gain,
                amount: 1,
            },
            id_source: None,
            target: Target::Direct(None),
        });
        state.effect_buf.push(Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // The Specimen: the corpse's Poison moves to a random survivor
    if has_relic(&state.id_relics, RelicName::TheSpecimen)
        && has_modifier(&target.modifiers, ModifierKind::Poison)
    {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Poison,
                stacks: modifier_stacks(&target.modifiers, ModifierKind::Poison),
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Monsters,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::Random { count: 1 },
            },
        });
    }

    // Gremlin Leader: its gremlins flee last; minions escaping here don't skip rewards because of `RoomKind::CombatElite`
    if gremlins_flee {
        for id_monster in id_monsters.iter() {
            if let Some(id) = *id_monster {
                state.effect_buf.push(Effect {
                    kind: EffectKind::MonsterEscape,
                    id_source: None,
                    target: Target::Direct(Some(id)),
                });
            }
        }
    }

    // The staged effects land behind everything queued; while minions fall with their leader, its death and theirs land them at once
    if minions_fall || with_leader {
        flush_effects_from_buf_to_queue_front(state);
    } else {
        state.effect_queue.extend(state.effect_buf.drain(..));
    }

    // Spore Cloud: the Character gains Vulnerable at once
    let modifiers = &state.entities[id_target].modifiers;
    if has_modifier(modifiers, ModifierKind::SporeCloud) {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Vulnerable,
                stacks: modifier_stacks(modifiers, ModifierKind::SporeCloud),
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }
}
