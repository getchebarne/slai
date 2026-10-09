use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

const BOUNCE: Effect = Effect {
    kind: EffectKind::ModifierDelta {
        kind: ModifierKind::Poison,
        stacks: 3,
    },
    id_source: None,
    target: Target::Resolve {
        candidate_pool: CandidatePool::Monsters,
        filters: &[],
        selection_kind: SelectionKind::Random { count: 1 },
    },
};

pub static BOUNCING_FLASK: CardTemplate = make_card_template(
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
pub static BOUNCING_FLASK_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects_play: {
        let mut effects = BOUNCING_FLASK.effects_play;
        effects[3] = BOUNCE; // +1 bounce
        effects
    },
    effects_play_len: 4,
    ..BOUNCING_FLASK
};
