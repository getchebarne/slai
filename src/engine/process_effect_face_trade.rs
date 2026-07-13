use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::RelicName;
use crate::utils::pick_from_pool;

const FACE_POOL: &[RelicName] = &[
    RelicName::CultistHeadpiece,
    RelicName::FaceOfCleric,
    RelicName::GremlinVisage,
    RelicName::NlothsHungryFace,
    RelicName::SsserpentHead,
];

// Face Trader: a random unowned face relic; Circlet once all five are owned
pub fn process_effect_face_trade(state: &mut GameState) {
    let name =
        pick_from_pool(FACE_POOL, &state.id_relics, &mut state.rng).unwrap_or(RelicName::Circlet);
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RelicGrantSpecific {
            name,
            fallback_circlet: false,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
