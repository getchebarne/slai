use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static CORPSE_EXPLOSION: CardTemplate = make_card_template(
    CardName::CorpseExplosion,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Poison,
                stacks: 6,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::CorpseExplosion,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static CORPSE_EXPLOSION_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = CORPSE_EXPLOSION.effects;
        effects[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 9, // +3 poison
        };
        effects
    },
    ..CORPSE_EXPLOSION
};
