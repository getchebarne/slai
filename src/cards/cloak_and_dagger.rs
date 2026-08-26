use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CardRarity;

pub static CLOAK_AND_DAGGER: CardTemplate = make_card_template(
    CardName::CloakAndDagger,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Shiv,
                pile: CardPile::Hand,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static CLOAK_AND_DAGGER_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = CLOAK_AND_DAGGER.effects;
        effects[1].kind = EffectKind::CardAdd {
            card_name: CardName::Shiv,
            pile: CardPile::Hand,
            count: 2, // +1 shiv
            upgraded: false,
        };
        effects
    },
    ..CLOAK_AND_DAGGER
};
