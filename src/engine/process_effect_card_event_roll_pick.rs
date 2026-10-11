use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

// The event's rolled Cards clear; the picked one, if any, joins the deck
pub fn process_effect_card_event_roll_pick(id_target: Option<usize>, state: &mut GameState) {
    assert!(state.event.active, "CardEventRollPick outside an event");
    state.event.id_card_pile_event_roll.clear();

    // A skipped pick adds nothing
    let Some(id_card) = id_target else {
        return;
    };
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardAdopt { landing: false },
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}
