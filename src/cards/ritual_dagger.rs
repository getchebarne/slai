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

// On fatal, permanently gains damage (+3, +5 upgraded)
pub static RITUAL_DAGGER: CardTemplate = make_card_template(
    CardName::RitualDagger,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Special,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 15,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::RitualDaggerProc { bump: 3 },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded: only the on-kill bump grows (3 -> 5); base damage is unchanged
pub static RITUAL_DAGGER_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = RITUAL_DAGGER.effects;
        effects[1].kind = EffectKind::RitualDaggerProc { bump: 5 };
        effects
    },
    ..RITUAL_DAGGER
};
