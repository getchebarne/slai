use crate::cards::make_entity_card;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

const BOUNCE: Effect = Effect {
    kind: EffectKind::ModifierGain {
        kind: ModifierKind::Poison,
        stacks: 3,
    },
    id_source: None,
    target: Target::Resolve {
        candidate_pool: CandidatePool::Monsters,
        filter: CandidateFilter::Any,
        selection_kind: SelectionKind::Random { count: 1 },
    },
};

pub static BOUNCING_FLASK: Entity = make_entity_card(
    CardName::BouncingFlask,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[BOUNCE, BOUNCE, BOUNCE],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded: one more bounce
pub static BOUNCING_FLASK_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = BOUNCING_FLASK.card_effects;
        effects[3] = BOUNCE; // +1 bounce
        effects
    },
    card_effects_len: 4,
    ..BOUNCING_FLASK
};
