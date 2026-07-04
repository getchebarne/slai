use crate::consts::MODE_SHIFT_INCREASE_PER_CYCLE;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::types::RelicName;
use crate::modifier::Modifiers;
use crate::modifier::modifier_apply;
use crate::modifier::modifier_def;
use crate::modifier::modifier_has;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;

pub fn process_effect_modifier_gain(
    id_target: Option<usize>,
    state: &mut GameState,
    kind: ModifierKind,
    stacks: i16,
) {
    let id_target = id_target.expect("ModifierGain requires id_target");

    // Ginger / Turnip: negate the application outright, before Artifact is consumed
    if id_target == state.id_character
        && stacks > 0
        && ((kind == ModifierKind::Weak
            && state.id_relics[RelicName::Ginger as usize].is_some())
            || (kind == ModifierKind::Frail
                && state.id_relics[RelicName::Turnip as usize].is_some()))
    {
        return;
    }

    let entity = &mut state.entities[id_target];

    // Snecko Skull: +1 to any positive Poison application on a monster
    let stacks = if kind == ModifierKind::Poison
        && stacks > 0
        && matches!(entity.kind, EntityKind::Monster)
        && state.id_relics[RelicName::SneckoSkull as usize].is_some()
    {
        stacks + 1
    } else {
        stacks
    };

    let monster_cycle_count = match entity.kind {
        EntityKind::Monster => Some(entity.monster_cycle_count),
        _ => None,
    };
    let modifiers = &mut entity.modifiers;

    // ModeShift has special scaling logic
    if kind == ModifierKind::ModeShift {
        if let Some(cc) = monster_cycle_count {
            return process_mode_shift_gain(modifiers, stacks, cc);
        }
    }

    // Check artifact
    let is_debuff_attempt = (stacks > 0 && !modifier_def(kind).is_buff)
        || (stacks < 0 && (kind == ModifierKind::Dexterity || kind == ModifierKind::Strength));
    if is_debuff_attempt && modifier_has(modifiers, ModifierKind::Artifact) {
        let stacks_new = modifier_stacks(modifiers, ModifierKind::Artifact) - 1;
        if stacks_new < modifier_def(ModifierKind::Artifact).stacks_min {
            modifier_remove(modifiers, ModifierKind::Artifact);
        } else {
            modifiers.stacks[ModifierKind::Artifact as usize] = stacks_new;
        }
        return;
    }

    // Negative stacks reduce existing modifier, removing if below minimum
    if stacks < 0 {
        if modifier_has(modifiers, kind) {
            let idx = kind as usize;
            modifiers.stacks[idx] += stacks;
            let mod_def = modifier_def(kind);
            if modifiers.stacks[idx] < mod_def.stacks_min {
                modifier_remove(modifiers, kind);
            }
        } else if modifier_def(kind).stacks_min < 0 {
            modifier_apply(modifiers, kind, stacks);
        }
        return;
    }

    modifier_apply(modifiers, kind, stacks);
}

fn process_mode_shift_gain(modifiers: &mut Modifiers, stacks: i16, monster_cycle_count: u8) {
    let increase = MODE_SHIFT_INCREASE_PER_CYCLE * monster_cycle_count as i16;
    modifier_apply(modifiers, ModifierKind::ModeShift, stacks + increase);
    modifiers.is_new[ModifierKind::ModeShift as usize] = false;
}

#[cfg(test)]
mod tests {
    use crate::effect::Effect;
    use crate::effect::EffectKind;
    use crate::effect::Target;
    use crate::engine::process_effect_queue;
    use crate::engine::test_support::char_modifier;
    use crate::engine::test_support::combat_with_relic;
    use crate::engine::test_support::first_monster;
    use crate::game::GameState;
    use crate::modifier::ModifierKind;
    use crate::modifier::modifier_apply;
    use crate::modifier::modifier_has;
    use crate::modifier::modifier_stacks;
    use crate::types::MonsterName;
    use crate::types::RelicName;

    fn gain(state: &mut GameState, id_target: usize, kind: ModifierKind, stacks: i16) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierGain { kind, stacks },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
        process_effect_queue(state);
    }

    #[test]
    fn snecko_skull_adds_one_poison_to_monsters() {
        let mut state = combat_with_relic(RelicName::SneckoSkull, MonsterName::JawWorm);
        let id_monster = first_monster(&state);
        gain(&mut state, id_monster, ModifierKind::Poison, 2);
        let mods = &state.entities[id_monster].modifiers;
        assert_eq!(modifier_stacks(mods, ModifierKind::Poison), 3);
    }

    #[test]
    fn ginger_negates_weak_without_consuming_artifact() {
        let mut state = combat_with_relic(RelicName::Ginger, MonsterName::JawWorm);
        let id_character = state.id_character;
        modifier_apply(
            &mut state.entities[id_character].modifiers,
            ModifierKind::Artifact,
            1,
        );
        gain(&mut state, id_character, ModifierKind::Weak, 1);
        assert!(!modifier_has(
            &state.entities[id_character].modifiers,
            ModifierKind::Weak
        ));
        assert_eq!(char_modifier(&state, ModifierKind::Artifact), 1);
    }

    #[test]
    fn turnip_negates_frail() {
        let mut state = combat_with_relic(RelicName::Turnip, MonsterName::JawWorm);
        let id_character = state.id_character;
        gain(&mut state, id_character, ModifierKind::Frail, 2);
        assert!(!modifier_has(
            &state.entities[id_character].modifiers,
            ModifierKind::Frail
        ));
    }
}
