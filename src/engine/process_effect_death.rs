use crate::effect::Amount;
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
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::PotionName;
use crate::types::RelicName;
use crate::utils::has_relic;

pub fn process_effect_death(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("Death requires id_target");

    // Character death: clear pending work, mark dead, signal game over.
    // Event damage can kill outside combat, so this path is mode-agnostic
    if id_target == state.id_character {
        // Fairy in a Bottle: consumed to revive at 30% max HP; checked before Lizard Tail
        if let Some(id_potion) = state
            .id_potions
            .iter()
            .flatten()
            .copied()
            .find(|&id| state.entities[id].potion_name == PotionName::FairyPotion)
        {
            remove_potion(&mut state.id_potions, id_potion);
            let vitals = &mut state.entities[state.id_character].vitals;
            vitals.health = ((vitals.health_max as f32) * 0.30) as u16;
            vitals.health = vitals.health.max(1);
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

        // Mark Character as dead, set `game_over` flag, and clear the effect queue
        state.entities[state.id_character].dead = true;
        state.game_over = true;
        state.effect_queue.clear();
        return;
    }

    // Monster-death path
    let Some(Mode::Combat { id_monsters, .. }) = state.mode_stack.last_mut() else {
        unreachable!("Monster death outside Combat mode")
    };
    let id_character = state.id_character;

    // Mark the corpse dead, drop it from the live roster, and check if combat continues
    state.entities[id_target].dead = true;
    if let Some(slot) = id_monsters.iter().position(|s| *s == Some(id_target)) {
        id_monsters[slot] = None; // Clear from `id_monsters` Vec
    }

    // Calculate if there're any monsters left alive
    let any_alive = id_monsters.iter().any(|s| s.is_some());

    // Return stolen gold. Only relevant for "Looter"s in practice
    let stolen_gold = state.entities[id_target].monster_stolen_gold;
    let gold_return = if stolen_gold > 0 {
        Some(Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(stolen_gold),
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        })
    } else {
        None
    };

    if !any_alive {
        // Combat ends. Replace pending effects with on-death triggers then CombatEnd
        state.effect_queue.clear();
        if let Some(e) = gold_return {
            state.effect_queue.push_back(e);
        }
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CombatEnd {
                escaped_character: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
        return;
    }

    let target = &state.entities[id_target];

    // Spore Cloud: Character gains 2 stacks of Vulnerable
    let spore_effect = if has_modifier(&target.modifiers, ModifierKind::SporeCloud) {
        let stacks = modifier_stacks(&target.modifiers, ModifierKind::SporeCloud);
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
    let corpse_explosion = has_modifier(&target.modifiers, ModifierKind::CorpseExplosion)
        .then(|| target.vitals.health_max);

    // The Specimen: the corpse's Poison moves to a random survivor
    let specimen_poison = (has_relic(&state.id_relics, RelicName::TheSpecimen)
        && has_modifier(&target.modifiers, ModifierKind::Poison))
    .then(|| modifier_stacks(&target.modifiers, ModifierKind::Poison));

    // Mid-combat: push to front so on-death triggers fire before suspended chain
    // (corpse already left id_monsters, so Monsters{All} resolves to survivors only)
    // Executes in reverse:
    //     1. GoldDelta (stolen-gold return)
    //     2. DamageDeal per survivor (Corpse Explosion)
    //     3. ModifierGain Vulnerable (Spore Cloud)
    //     4. EnergyDelta then CardDraw (Gremlin Horn)
    //     5. ModifierGain Poison (The Specimen)
    if let Some(stacks) = specimen_poison {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Poison,
                stacks,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Monsters,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::Random { count: 1 },
            },
        });
    }
    // Gremlin Horn: a monster's death grants 1 energy and draws 1
    if has_relic(&state.id_relics, RelicName::GremlinHorn) {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::EnergyDelta {
                sign: DeltaSign::Gain,
                amount: 1,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
    if let Some(e) = spore_effect {
        state.effect_queue.push_front(e);
    }
    if let Some(dmg) = corpse_explosion {
        for slot in id_monsters.iter().rev() {
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
