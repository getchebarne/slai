use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static GRAND_FINALE: CardTemplate = make_card_template(
    CardName::GrandFinale,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 50,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
    &[],
    &[],
    PlayRestriction::DrawPileEmpty,
);
// Upgraded
pub static GRAND_FINALE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = GRAND_FINALE.effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 60,
            lifesteal: false,
        }; // +10 damage
        effects
    },
    ..GRAND_FINALE
};
