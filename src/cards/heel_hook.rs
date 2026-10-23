use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_SELECTED;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static HEEL_HOOK: CardTemplate = make_card_template(
    CardName::HeelHook,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::HeelHookProc { damage: 5 },
        id_source: None,
        target: TARGET_MONSTER_SELECTED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static HEEL_HOOK_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects_play: {
        let mut effects = HEEL_HOOK.effects_play;
        effects[0].kind = EffectKind::HeelHookProc { damage: 8 }; // +3 damage
        effects
    },
    ..HEEL_HOOK
};
