use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

const HIT: Effect = Effect {
    kind: EffectKind::DamagePhysical {
        amount: 3,
        lifesteal: false,
    },
    id_source: None,
    target: TARGET_MONSTER_PICKED,
};
const HIT_PLUS: Effect = Effect {
    kind: EffectKind::DamagePhysical {
        amount: 4,
        lifesteal: false,
    }, // +1 damage
    id_source: None,
    target: TARGET_MONSTER_PICKED,
};

pub static RIDDLE_WITH_HOLES: CardTemplate = make_card_template(
    CardName::RiddleWithHoles,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[HIT, HIT, HIT, HIT, HIT],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static RIDDLE_WITH_HOLES_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = RIDDLE_WITH_HOLES.effects;
        effects[0] = HIT_PLUS;
        effects[1] = HIT_PLUS;
        effects[2] = HIT_PLUS;
        effects[3] = HIT_PLUS;
        effects[4] = HIT_PLUS;
        effects
    },
    ..RIDDLE_WITH_HOLES
};
