use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicExclusion;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use rand::Rng;

pub fn process_effect_mausoleum_open(state: &mut GameState) {
    // Executes in reverse: the curse first, so Omamori judges it before the Relic arrives
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RelicGrantRandom {
            tier: None,
            exclusion: RelicExclusion::Screenless,
        },
        id_source: None,
        target: Target::Direct(None),
    });
    if state.rng.random_bool(0.5) || state.ascension >= 15 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Writhe,
                card_pile: CardPile::Deck,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
