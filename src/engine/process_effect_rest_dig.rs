use crate::consts::RELIC_TIER_TH_COMMON;
use crate::consts::RELIC_TIER_TH_UNCOMMON;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicExclusion;
use crate::effect::RelicPick;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Focus;
use crate::utils::context_focus;

// Dig spends the site and leaves it at once: leaving the dug Relic's reward leaves the Room
pub fn process_effect_rest_dig(state: &mut GameState) {
    assert!(
        context_focus(state) == Focus::RestSite,
        "RestDig outside the RestSite context"
    );
    state.rest_site.active = false;
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RewardRollRelic {
            pick: RelicPick::Thresholds {
                th_common: RELIC_TIER_TH_COMMON,
                th_uncommon: RELIC_TIER_TH_UNCOMMON,
            },
            exclusion: RelicExclusion::Unfiltered,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
