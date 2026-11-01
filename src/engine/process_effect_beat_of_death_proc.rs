use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_stacks;

// Beat of Death answers a Card play, even for a holder the play killed: a hit of its stacks on the Character behind everything queued
pub fn process_effect_beat_of_death_proc(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("BeatOfDeathProc requires id_target");
    let stacks = modifier_stacks(
        &state.entities[id_target].modifiers,
        ModifierKind::BeatOfDeath,
    );

    // An unscaled hit with no source, so Torii, Thorns, Plated Armor and Painful Stabs ignore it
    state.effect_queue.push_back(Effect {
        kind: EffectKind::DamageDeal {
            amount: stacks as u16,
            lifesteal: false,
        },
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    });
}
