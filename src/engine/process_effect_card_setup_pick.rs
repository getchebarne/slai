use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Combat;
use crate::types::CostScope;

pub fn process_effect_card_setup_pick(
    id_target: Option<usize>,
    state: &mut GameState,
    free: bool,
    bottom: bool,
) {
    assert!(
        state.combat.active,
        "process_effect_card_setup_pick outside the Combat frame"
    );
    let Combat {
        id_card_hand,
        id_card_draw,
        ..
    } = &mut state.combat;
    let id_target = id_target.expect("CardSetupPick requires id_target");
    if free {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::SetCostOverride {
                amount: 0,
                only_reduce: false,
                random: false,
                scope: CostScope::UntilPlayed,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }
    if let Some(pos) = id_card_hand.iter().position(|&id| id == id_target) {
        id_card_hand.remove(pos);
    }
    // Top of the draw pile (Setup) is the vec's end; bottom (Forethought) is index 0
    if bottom {
        id_card_draw.insert(0, id_target);
    } else {
        id_card_draw.push(id_target);
    }
}
