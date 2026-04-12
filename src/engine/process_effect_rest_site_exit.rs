use crate::consts::MAP_HEIGHT;
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::state::Map;

pub fn process_effect_rest_site_exit(map: &mut Map) -> ProcessEffectResult {
    if map.y_current == Some(MAP_HEIGHT - 1) {
        // Final row — advance into the boss room and let RoomEnter do the setup
        map.y_current = Some(MAP_HEIGHT);
        map.x_current = Some(0);
        ProcessEffectResult::AddAndContinue {
            top: vec![Effect {
                kind: EffectKind::RoomEnter,
                source: None,
                target: Target::Direct(None),
            }],
            bot: Vec::new(),
        }
    } else {
        // Non-final row — halt and wait for the player to pick the next map node
        ProcessEffectResult::AddAndContinue {
            top: vec![crate::effect::Effect {
                kind: crate::effect::EffectKind::SelectMapNode,
                source: None,
                target: crate::effect::Target::Resolve {
                    candidates: crate::effect::CandidatePool::MapNodeNextRow,
                    selection: crate::effect::SelectionKind::Input { count: 1 },
                },
            }],
            bot: Vec::new(),
        }
    }
}
