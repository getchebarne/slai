use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ScaledAt;
use crate::effect::TARGET_MONSTER_SELECTED;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static QUICK_SLASH: CardTemplate = make_card_template(
    CardName::QuickSlash,
    CardKind::Attack,
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
            kind: EffectKind::DamagePhysical {
                amount: 8,
                instances: 1,
                lifesteal: false,
                if_poisoned: false,
                scaled_at: ScaledAt::Now,
            },
            id_source: None,
            target: TARGET_MONSTER_SELECTED,
        },
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static QUICK_SLASH_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects_play: {
        let mut effects = QUICK_SLASH.effects_play;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 12,
            instances: 1,
            lifesteal: false,
            if_poisoned: false,
            scaled_at: ScaledAt::Now,
        }; // +4 damage
        effects
    },
    ..QUICK_SLASH
};
