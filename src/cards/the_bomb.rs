use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::consts::BOMB_FUSE_TURNS;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static THE_BOMB: CardTemplate = make_card_template(
    CardName::TheBomb,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::BombArm {
            turns: BOMB_FUSE_TURNS,
            damage: 40,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static THE_BOMB_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = THE_BOMB.effects;
        effects[0].kind = EffectKind::BombArm {
            turns: BOMB_FUSE_TURNS,
            damage: 50, // +10 damage
        };
        effects
    },
    ..THE_BOMB
};
