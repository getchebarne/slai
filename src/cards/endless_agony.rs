use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::effect::TARGET_SOURCE;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CardRarity;

pub static ENDLESS_AGONY: CardTemplate = make_card_template(
    CardName::EndlessAgony,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 4,
            instances: 1,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    // Drawing it adds a copy of it, as drawn, to the hand
    &[Effect {
        kind: EffectKind::CardDuplicate {
            pile: CardPile::Hand,
        },
        id_source: None,
        target: TARGET_SOURCE,
    }],
    PlayRestriction::Always,
);
// Upgraded
pub static ENDLESS_AGONY_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects_play: {
        let mut effects = ENDLESS_AGONY.effects_play;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 6,
            instances: 1,
            lifesteal: false,
        }; // +2 damage
        effects
    },
    ..ENDLESS_AGONY
};
