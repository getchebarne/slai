use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::utils::flush_effects_from_buf_to_queue_front;

// The Character turns to its target: Back Attack leaves the target at once and lands on the other living Monsters; queued only while Surrounded holds
pub fn process_effect_face_monster(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("FaceMonster requires id_target");
    assert!(
        has_modifier(
            &state.entities[state.id_character].modifiers,
            ModifierKind::Surrounded
        ),
        "FaceMonster requires a Surrounded Character"
    );
    state.effect_buf.clear();
    state.effect_buf.push(Effect {
        kind: EffectKind::ModifierRemove {
            kind: ModifierKind::BackAttack,
        },
        id_source: None,
        target: Target::Direct(Some(id_target)),
    });
    state.effect_buf.push(Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::BackAttack,
            stacks: 1,
        },
        id_source: Some(id_target),
        target: Target::Resolve {
            candidate_pool: CandidatePool::Monsters,
            filters: &[CandidateFilter::NotSource],
            selection_kind: SelectionKind::All,
        },
    });
    flush_effects_from_buf_to_queue_front(state);
}
