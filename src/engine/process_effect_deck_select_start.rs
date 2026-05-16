use crate::events::deck_filter_any;
use crate::events::deck_filter_non_basic_non_curse;
use crate::events::deck_filter_purgeable;
use crate::events::deck_filter_upgradable;
use crate::game::GameState;
use crate::types::DeckSelectKind;
use crate::types::Phase;

pub fn process_effect_deck_select_start(
    kind: DeckSelectKind,
    state: &GameState,
) -> Option<Phase> {
    let id_options = match kind {
        DeckSelectKind::Remove => deck_filter_purgeable(state),
        DeckSelectKind::UpgradeAny => deck_filter_upgradable(state),
        DeckSelectKind::TransformOne => deck_filter_non_basic_non_curse(state),
        DeckSelectKind::DuplicateAny => deck_filter_any(state),
    };
    if id_options.is_empty() {
        return None;
    }
    Some(Phase::AwaitDeckSelect { kind, id_options })
}
