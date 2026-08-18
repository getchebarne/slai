use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::effect::Target;
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
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::EndlessAgony,
            pile: CardPile::Hand,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    PlayRestriction::Always,
);
// Upgraded
pub static ENDLESS_AGONY_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = ENDLESS_AGONY.effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 6,
            lifesteal: false,
        }; // +2 damage
        effects
    },
    effects_on_draw: &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::EndlessAgony,
            pile: CardPile::Hand,
            count: 1,
            upgraded: true,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    ..ENDLESS_AGONY
};
