use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicExclusion;
use crate::effect::RelicRoll;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Focus;
use crate::utils::context_focus;
use crate::utils::relic_dig_cuts;

// Dig spends the site and leaves it at once: leaving the dug Relic's reward leaves the Room
pub fn process_effect_rest_dig(state: &mut GameState) {
    assert!(
        context_focus(state) == Focus::RestSite,
        "RestDig outside the RestSite context"
    );
    state.rest_site.active = false;

    // The tier roll is always drawn; Act 4's cuts make it always Uncommon
    let (th_common, th_uncommon) = relic_dig_cuts(state.act);
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RewardRollRelic {
            roll: RelicRoll::Thresholds {
                th_common,
                th_uncommon,
            },
            exclusion: RelicExclusion::Unfiltered,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
