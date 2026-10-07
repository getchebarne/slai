use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_set_not_new;
use crate::modifier::modifier_stacks;
use crate::modifier::modifier_tick;

pub fn process_effect_modifier_tick(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("ModifierTick requires id_target");
    let modifiers = &mut state.entities[id_target].modifiers;

    if id_target != state.id_character {
        // Ritual: a Monster gains `stacks` Strength each round end, skipping the round it gained Ritual
        if has_modifier(modifiers, ModifierKind::Ritual)
            && !modifiers.is_new[ModifierKind::Ritual as usize]
        {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Strength,
                    stacks: modifier_stacks(modifiers, ModifierKind::Ritual),
                },
                id_source: None,
                target: Target::Direct(Some(id_target)),
            });
        }

        // A Monster's Modifiers stop being new here; only debuffs on the Character skip a round end
        modifier_set_not_new(modifiers);
    }
    modifier_tick(modifiers);
}
