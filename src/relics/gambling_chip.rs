use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// At combat start, discard any number of cards, then draw that many
// (combat-start effects drain after the turn-1 chain, matching the source's post-draw timing)
// See:
//    - `process_effect_gambling_chip_proc.rs`
pub static GAMBLING_CHIP: Entity = make_entity_relic(
    RelicName::GamblingChip,
    RelicTier::Rare,
    0,
    &[Effect {
        kind: EffectKind::GamblingChipProc {
            discards_before: None,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
);
