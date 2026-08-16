use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventKind;
use crate::events::bake_options;
use crate::events::make_entity_event_option;
use crate::game::GameState;
use crate::relics::iter_owned_relics;
use crate::types::RelicName;
use rand::Rng;

// Two distinct owned Relics rolled at spawn; the draw gate guarantees at least two
// Rolls two distinct owned Relics and bakes them into the trade options
pub fn spawn_event_nloth(state: &mut GameState) -> (EventKind, Vec<usize>) {
    let owned: Vec<_> = iter_owned_relics(&state.id_relics)
        .map(|(name, _)| name)
        .collect();
    let idx = state.rng.random_range(0..owned.len());
    let mut jdx = state.rng.random_range(0..owned.len() - 1);
    if jdx >= idx {
        jdx += 1;
    }
    let relic_first = owned[idx];
    let relic_second = owned[jdx];
    let id_event_options = bake_options(state, &bake(relic_first, relic_second));
    (
        EventKind::Nloth {
            relic_first,
            relic_second,
        },
        id_event_options,
    )
}

fn option_trade(label: &'static str, name: RelicName) -> Entity {
    make_entity_event_option(
        label,
        &[
            Effect {
                kind: EffectKind::RelicLose { name },
                id_source: None,
                target: Target::Direct(None),
            },
            Effect {
                kind: EffectKind::RelicGrantSpecific {
                    name: RelicName::NlothsGift,
                    fallback_circlet: true,
                },
                id_source: None,
                target: Target::Direct(None),
            },
            EVENT_CONSUME_EFFECT,
        ],
    )
}

// The rolled Relics are spawn-time state, so the trades bake straight into the options
pub fn bake(relic_first: RelicName, relic_second: RelicName) -> [Entity; 3] {
    [
        option_trade("[Offer the first Relic] Obtain N'loth's Gift.", relic_first),
        option_trade(
            "[Offer the second Relic] Obtain N'loth's Gift.",
            relic_second,
        ),
        make_entity_event_option("[Refuse] Nothing happens.", &[EVENT_CONSUME_EFFECT]),
    ]
}
