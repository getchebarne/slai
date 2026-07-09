use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::types::DeltaSign;
use crate::types::RelicName;

pub fn process_effect_death(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("Death requires id_target");
    // Character death: clear pending work, mark dead, signal game over
    if id_target == state.id_character {
        // Lizard Tail: once per run, survive at half max HP instead
        if let Some(id_relic) = state.id_relics[RelicName::LizardTail as usize]
            && !state.entities[id_relic].relic_used_up
        {
            state.entities[id_relic].relic_used_up = true;
            let vitals = &mut state.entities[state.id_character].vitals;
            vitals.health = (vitals.health_max / 2).max(1);
            return;
        }
        state.entities[state.id_character].dead = true;
        state.game_over = true;
        state.effect_queue.clear();
        return;
    }

    let id_character = state.id_character;
    let monster = &state.entities[id_target];

    let gold_return = if monster.monster_stolen_gold > 0 {
        Some(Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(monster.monster_stolen_gold),
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        })
    } else {
        None
    };

    let spore_effect = if has_modifier(&monster.modifiers, ModifierKind::SporeCloud) {
        let stacks = modifier_stacks(&monster.modifiers, ModifierKind::SporeCloud);
        Some(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Vulnerable,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        })
    } else {
        None
    };

    // CorpseExplosion: max_health to others; no source scaling, no Envenom proc
    let corpse_explosion = has_modifier(&monster.modifiers, ModifierKind::CorpseExplosion)
        .then(|| monster.vitals.health_max);

    // The Specimen: the corpse's Poison moves to a random survivor
    let specimen_poison = (state.id_relics[RelicName::TheSpecimen as usize].is_some()
        && has_modifier(&monster.modifiers, ModifierKind::Poison))
    .then(|| modifier_stacks(&monster.modifiers, ModifierKind::Poison));

    state.entities[id_target].dead = true;
    if let Some(slot) = state.id_monsters.iter().position(|s| *s == Some(id_target)) {
        state.id_monsters[slot] = None;
    }

    let any_alive = state.id_monsters.iter().any(|s| s.is_some());

    if !any_alive {
        // Combat ends. Replace pending effects with on-death triggers then CombatEnd
        state.effect_queue.clear();
        if let Some(e) = gold_return {
            state.effect_queue.push_back(e);
        }
        if let Some(dmg) = corpse_explosion {
            for slot in state.id_monsters.iter() {
                if let Some(id) = *slot
                    && id != id_target
                {
                    state.effect_queue.push_back(Effect {
                        kind: EffectKind::DamageDeal { amount: dmg },
                        id_source: None,
                        target: Target::Direct(Some(id)),
                    });
                }
            }
        }
        if let Some(e) = spore_effect {
            state.effect_queue.push_back(e);
        }
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CombatEnd,
            id_source: None,
            target: Target::Direct(None),
        });
    } else {
        // Mid-combat: push to front so on-death triggers fire before suspended chain
        // (corpse already left id_monsters, so Monsters{All} resolves to survivors only)
        if let Some(stacks) = specimen_poison {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Poison,
                    stacks,
                },
                id_source: None,
                target: Target::Resolve {
                    candidate_pool: CandidatePool::Monsters {
                        filter: CandidatePoolMonstersFilter::All,
                    },
                    selection_kind: SelectionKind::Random { count: 1 },
                },
            });
        }
        if state.id_relics[RelicName::GremlinHorn as usize].is_some() {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardDraw { count: 1 },
                id_source: None,
                target: Target::Direct(None),
            });
            state.effect_queue.push_front(Effect {
                kind: EffectKind::EnergyGain { amount: 1 },
                id_source: None,
                target: Target::Direct(None),
            });
        }
        if let Some(e) = spore_effect {
            state.effect_queue.push_front(e);
        }
        if let Some(dmg) = corpse_explosion {
            for slot in state.id_monsters.iter().rev() {
                if let Some(id) = *slot
                    && id != id_target
                {
                    state.effect_queue.push_front(Effect {
                        kind: EffectKind::DamageDeal { amount: dmg },
                        id_source: None,
                        target: Target::Direct(Some(id)),
                    });
                }
            }
        }
        if let Some(e) = gold_return {
            state.effect_queue.push_front(e);
        }
    }
}
