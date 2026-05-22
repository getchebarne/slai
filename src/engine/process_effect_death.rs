use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;

pub fn process_effect_death(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("Death requires id_target");
    // Character death: clear pending work, mark dead, signal game over
    if id_target == state.id_character {
        state.entities[state.id_character].dead = true;
        state.game_over = true;
        state.effect_queue.clear();
        return;
    }

    let id_character = state.id_character;
    let monster = &state.entities[id_target];

    let gold_return = if monster.monster_stolen_gold > 0 {
        Some(Effect {
            kind: EffectKind::GoldGain {
                amount: monster.monster_stolen_gold,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        })
    } else {
        None
    };

    let spore_effect = if modifier_has(&monster.modifiers, ModifierKind::SporeCloud) {
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

    // CorpseExplosion: dying enemy deals damage equal to its max HP to every
    // other alive enemy. `id_source = None` so no source-side scaling and
    // Envenom can't proc; block still subtracts
    let corpse_explosion = modifier_has(&monster.modifiers, ModifierKind::CorpseExplosion)
        .then(|| monster.vitals.health_max);

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
        // Mid-combat: push to front so on-death triggers fire before any
        // suspended chain resumes
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
