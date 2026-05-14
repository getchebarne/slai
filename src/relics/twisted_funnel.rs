use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::modifier::ModifierKind;
use crate::types::RelicName;
use crate::types::RelicTier;

pub static TWISTED_FUNNEL: Entity = make_entity_relic(
    RelicName::TwistedFunnel,
    RelicTier::Shop,
    0,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 4,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
);
