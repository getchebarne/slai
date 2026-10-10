use crate::consts::MODE_SHIFT_INCREASE_PER_CYCLE;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::effect::effect_accuracy_resync;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::has_modifier;
use crate::modifier::modifier_apply;
use crate::modifier::modifier_def;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::monsters::byrd;
use crate::monsters::shelled_parasite;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::scale_block_gain;

pub fn process_effect_modifier_delta(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    kind: ModifierKind,
    stacks: i16,
) {
    let id_target = id_target.expect("ModifierDelta requires id_target");

    // A corpse takes no Powers, and its hooks (Snecko Skull, Sadistic Nature) must not fire
    if state.entities[id_target].dead {
        return;
    }

    // Dodge and Roll freezes the Dex/Frail-adjusted block at play time; No Block freezes 0
    let stacks = if kind == ModifierKind::NextTurnBlock
        && stacks > 0
        && id_source.is_some_and(|id| state.entities[id].kind == EntityKind::Card)
    {
        let mods = &state.entities[state.id_character].modifiers;
        if has_modifier(mods, ModifierKind::NoBlock) {
            0
        } else {
            let dex_stacks = modifier_stacks(mods, ModifierKind::Dexterity);
            let is_frail = has_modifier(mods, ModifierKind::Frail);
            scale_block_gain(stacks as u16, dex_stacks, is_frail) as i16
        }
    } else {
        stacks
    };

    // Accuracy resets every Shiv in the hand and the draw, discard and exhaust piles; one held by Stasis or Nightmare keeps its damage
    if kind == ModifierKind::Accuracy && stacks != 0 && id_target == state.id_character {
        for candidate_pool in [
            CandidatePool::Hand,
            CandidatePool::PileDraw,
            CandidatePool::PileDiscard,
            CandidatePool::PileExhaust,
        ] {
            state
                .effect_queue
                .push_front(effect_accuracy_resync(candidate_pool));
        }
    }

    // Ginger / Turnip: negate the application outright, before Artifact is consumed
    if id_target == state.id_character
        && stacks > 0
        && ((kind == ModifierKind::Weak && has_relic(&state.id_relics, RelicName::Ginger))
            || (kind == ModifierKind::Frail && has_relic(&state.id_relics, RelicName::Turnip)))
    {
        return;
    }

    // No Draw on a target already under it is refused outright, before Artifact is consumed
    if kind == ModifierKind::NoDraw
        && has_modifier(&state.entities[id_target].modifiers, ModifierKind::NoDraw)
    {
        return;
    }

    // A debuff is the Character's unless a Monster's own effect applies it
    let from_monster = id_source.is_some_and(|id| state.entities[id].kind == EntityKind::Monster);

    // Get mutable target reference
    let target = &mut state.entities[id_target];

    // Snecko Skull: +1 to any positive Poison the Character applies to a Monster
    let stacks = if kind == ModifierKind::Poison
        && stacks > 0
        && !from_monster
        && matches!(target.kind, EntityKind::Monster)
        && has_relic(&state.id_relics, RelicName::SneckoSkull)
    {
        stacks.saturating_add(1)
    } else {
        stacks
    };

    // ModeShift has special scaling logic
    if kind == ModifierKind::ModeShift && target.kind == EntityKind::Monster {
        return process_mode_shift_gain(&mut target.modifiers, stacks, target.monster_cycle_count);
    }

    // Artifact
    let modifiers = &mut target.modifiers;
    let is_debuff_attempt = (stacks > 0 && !modifier_def(kind).is_buff)
        || (stacks < 0 && (kind == ModifierKind::Dexterity || kind == ModifierKind::Strength));

    if is_debuff_attempt && has_modifier(modifiers, ModifierKind::Artifact) {
        let stacks_new = modifier_stacks(modifiers, ModifierKind::Artifact) - 1;
        if stacks_new < modifier_def(ModifierKind::Artifact).stacks_min {
            modifier_remove(modifiers, ModifierKind::Artifact);
        } else {
            modifiers.stacks[ModifierKind::Artifact as usize] = stacks_new;
        }
        return;
    }

    // Apply the delta
    let had_kind = has_modifier(modifiers, kind);
    let had_flight = has_modifier(modifiers, ModifierKind::Flight);
    let had_plated_armor = has_modifier(modifiers, ModifierKind::PlatedArmor);
    modifier_apply(modifiers, kind, stacks);

    // A Modifier that appears takes the next stamp, so one removed and applied again goes last
    if !had_kind && has_modifier(modifiers, kind) {
        modifiers.seq[kind as usize] = state.combat.modifier_seq_next;
        state.combat.modifier_seq_next += 1;
    }

    // Shelled Parasite: stripping the last Plated Armor stack breaks the shell and stuns; a strip queued past it does nothing
    if kind == ModifierKind::PlatedArmor
        && had_plated_armor
        && !has_modifier(modifiers, ModifierKind::PlatedArmor)
        && state.entities[id_target].monster_name == MonsterName::ShelledParasite
    {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::MoveUpdate {
                move_override: Some(shelled_parasite::IDX_MOVE_STUNNED),
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    // Byrd: losing its last Flight stack grounds it and stuns
    if kind == ModifierKind::Flight
        && had_flight
        && !has_modifier(&state.entities[id_target].modifiers, ModifierKind::Flight)
        && state.entities[id_target].monster_name == MonsterName::Byrd
    {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::MoveUpdate {
                move_override: Some(byrd::IDX_MOVE_STUNNED),
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    // Sadistic Nature: player-applied debuffs landing on a Monster proc THORNS-type damage
    if is_debuff_attempt
        && kind != ModifierKind::Shackled
        && !from_monster
        && state.entities[id_target].kind == EntityKind::Monster
    {
        let mods_char = &state.entities[state.id_character].modifiers;
        if has_modifier(mods_char, ModifierKind::SadisticNature) {
            let dmg = modifier_stacks(mods_char, ModifierKind::SadisticNature);

            // The hit waits behind everything queued
            state.effect_queue.push_back(Effect {
                kind: EffectKind::DamageDeal {
                    amount: dmg.max(0) as u16,
                    lifesteal: false,
                },
                id_source: None,
                target: Target::Direct(Some(id_target)),
            });
        }
    }
}

fn process_mode_shift_gain(modifiers: &mut Modifiers, stacks: i16, monster_cycle_count: u8) {
    modifier_apply(
        modifiers,
        ModifierKind::ModeShift,
        stacks + MODE_SHIFT_INCREASE_PER_CYCLE * monster_cycle_count as i16,
    );
    modifiers.is_new[ModifierKind::ModeShift as usize] = false;
}
