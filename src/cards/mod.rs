mod a_thousand_cuts;
mod accuracy;
mod acrobatics;
mod adrenaline;
mod after_image;
mod alchemize;
mod all_out_attack;
mod ascenders_bane;
mod backflip;
mod backstab;
mod bandage_up;
mod bane;
mod blade_dance;
mod blind;
mod blur;
mod bouncing_flask;
mod bullet_time;
mod burn;
mod burst;
mod calculated_gamble;
mod caltrops;
mod catalyst;
mod choke;
mod cloak_and_dagger;
mod concentrate;
mod corpse_explosion;
mod crippling_poison;
mod dagger_spray;
mod dagger_throw;
mod dash;
mod dazed;
mod deadly_poison;
mod decay;
mod deep_breath;
mod defend;
mod deflect;
mod die_die_die;
mod distraction;
mod dodge_and_roll;
mod doppelganger;
mod doubt;
mod endless_agony;
mod envenom;
mod escape_plan;
mod eviscerate;
mod expertise;
mod finesse;
mod finisher;
mod flash_of_steel;
mod flechettes;
mod flying_knee;
mod footwork;
mod glass_knife;
mod good_instincts;
mod grand_finale;
mod heel_hook;
mod infinite_blades;
mod injury;
mod leg_sweep;
mod malaise;
mod master_of_strategy;
mod masterful_stab;
mod mind_blast;
mod neutralize;
mod nightmare;
mod normality;
mod noxious_fumes;
mod outmaneuver;
mod pain;
mod parasite;
mod phantasmal_killer;
mod piercing_wail;
mod poisoned_stab;
mod predator;
mod prepared;
mod quick_slash;
mod reflex;
mod regret;
mod riddle_with_holes;
mod setup;
mod shame;
mod shiv;
mod skewer;
mod slice;
mod slimed;
mod sneaky_strike;
mod storm_of_steel;
mod strike;
mod sucker_punch;
mod survivor;
mod swift_strike;
mod tactician;
mod terror;
mod tools_of_the_trade;
mod unload;
mod well_laid_plans;
mod wraith_form;
mod writhe;

use crate::entity::Entity;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::utils::shuffle;
use strum::EnumCount;

pub fn get_card(name: CardName, upgraded: bool) -> Entity {
    match (name, upgraded) {
        (CardName::AThousandCuts, false) => a_thousand_cuts::A_THOUSAND_CUTS,
        (CardName::AThousandCuts, true) => a_thousand_cuts::A_THOUSAND_CUTS_PLUS,
        (CardName::Accuracy, false) => accuracy::ACCURACY,
        (CardName::Accuracy, true) => accuracy::ACCURACY_PLUS,
        (CardName::Acrobatics, false) => acrobatics::ACROBATICS,
        (CardName::Acrobatics, true) => acrobatics::ACROBATICS_PLUS,
        (CardName::Adrenaline, false) => adrenaline::ADRENALINE,
        (CardName::Adrenaline, true) => adrenaline::ADRENALINE_PLUS,
        (CardName::AfterImage, false) => after_image::AFTER_IMAGE,
        (CardName::AfterImage, true) => after_image::AFTER_IMAGE_PLUS,
        (CardName::Alchemize, false) => alchemize::ALCHEMIZE,
        (CardName::Alchemize, true) => alchemize::ALCHEMIZE_PLUS,
        (CardName::AllOutAttack, false) => all_out_attack::ALL_OUT_ATTACK,
        (CardName::AllOutAttack, true) => all_out_attack::ALL_OUT_ATTACK_PLUS,
        (CardName::Backflip, false) => backflip::BACKFLIP,
        (CardName::Backflip, true) => backflip::BACKFLIP_PLUS,
        (CardName::Backstab, false) => backstab::BACKSTAB,
        (CardName::Backstab, true) => backstab::BACKSTAB_PLUS,
        (CardName::BandageUp, false) => bandage_up::BANDAGE_UP,
        (CardName::BandageUp, true) => bandage_up::BANDAGE_UP_PLUS,
        (CardName::Bane, false) => bane::BANE,
        (CardName::Bane, true) => bane::BANE_PLUS,
        (CardName::BladeDance, false) => blade_dance::BLADE_DANCE,
        (CardName::BladeDance, true) => blade_dance::BLADE_DANCE_PLUS,
        (CardName::Blind, false) => blind::BLIND,
        (CardName::Blind, true) => blind::BLIND_PLUS,
        (CardName::Blur, false) => blur::BLUR,
        (CardName::Blur, true) => blur::BLUR_PLUS,
        (CardName::BouncingFlask, false) => bouncing_flask::BOUNCING_FLASK,
        (CardName::BouncingFlask, true) => bouncing_flask::BOUNCING_FLASK_PLUS,
        (CardName::BulletTime, false) => bullet_time::BULLET_TIME,
        (CardName::BulletTime, true) => bullet_time::BULLET_TIME_PLUS,
        (CardName::Burn, false) => burn::BURN,
        (CardName::Burn, true) => burn::BURN_UPGRADED,
        (CardName::Burst, false) => burst::BURST,
        (CardName::Burst, true) => burst::BURST_PLUS,
        (CardName::CalculatedGamble, false) => calculated_gamble::CALCULATED_GAMBLE,
        (CardName::CalculatedGamble, true) => calculated_gamble::CALCULATED_GAMBLE_PLUS,
        (CardName::Caltrops, false) => caltrops::CALTROPS,
        (CardName::Caltrops, true) => caltrops::CALTROPS_PLUS,
        (CardName::Catalyst, false) => catalyst::CATALYST,
        (CardName::Catalyst, true) => catalyst::CATALYST_PLUS,
        (CardName::Choke, false) => choke::CHOKE,
        (CardName::Choke, true) => choke::CHOKE_PLUS,
        (CardName::CloakAndDagger, false) => cloak_and_dagger::CLOAK_AND_DAGGER,
        (CardName::CloakAndDagger, true) => cloak_and_dagger::CLOAK_AND_DAGGER_PLUS,
        (CardName::Concentrate, false) => concentrate::CONCENTRATE,
        (CardName::Concentrate, true) => concentrate::CONCENTRATE_PLUS,
        (CardName::CorpseExplosion, false) => corpse_explosion::CORPSE_EXPLOSION,
        (CardName::CorpseExplosion, true) => corpse_explosion::CORPSE_EXPLOSION_PLUS,
        (CardName::CripplingPoison, false) => crippling_poison::CRIPPLING_POISON,
        (CardName::CripplingPoison, true) => crippling_poison::CRIPPLING_POISON_PLUS,
        (CardName::DaggerSpray, false) => dagger_spray::DAGGER_SPRAY,
        (CardName::DaggerSpray, true) => dagger_spray::DAGGER_SPRAY_PLUS,
        (CardName::DaggerThrow, false) => dagger_throw::DAGGER_THROW,
        (CardName::DaggerThrow, true) => dagger_throw::DAGGER_THROW_PLUS,
        (CardName::Dash, false) => dash::DASH,
        (CardName::Dash, true) => dash::DASH_PLUS,
        (CardName::Dazed, _) => dazed::DAZED, // No upgraded variant
        (CardName::DeadlyPoison, false) => deadly_poison::DEADLY_POISON,
        (CardName::DeadlyPoison, true) => deadly_poison::DEADLY_POISON_PLUS,
        (CardName::DeepBreath, false) => deep_breath::DEEP_BREATH,
        (CardName::DeepBreath, true) => deep_breath::DEEP_BREATH_PLUS,
        (CardName::Defend, false) => defend::DEFEND,
        (CardName::Defend, true) => defend::DEFEND_PLUS,
        (CardName::Deflect, false) => deflect::DEFLECT,
        (CardName::Deflect, true) => deflect::DEFLECT_PLUS,
        (CardName::DieDieDie, false) => die_die_die::DIE_DIE_DIE,
        (CardName::DieDieDie, true) => die_die_die::DIE_DIE_DIE_PLUS,
        (CardName::Distraction, false) => distraction::DISTRACTION,
        (CardName::Distraction, true) => distraction::DISTRACTION_PLUS,
        (CardName::DodgeAndRoll, false) => dodge_and_roll::DODGE_AND_ROLL,
        (CardName::DodgeAndRoll, true) => dodge_and_roll::DODGE_AND_ROLL_PLUS,
        (CardName::Doppelganger, false) => doppelganger::DOPPELGANGER,
        (CardName::Doppelganger, true) => doppelganger::DOPPELGANGER_PLUS,
        (CardName::EndlessAgony, false) => endless_agony::ENDLESS_AGONY,
        (CardName::EndlessAgony, true) => endless_agony::ENDLESS_AGONY_PLUS,
        (CardName::Envenom, false) => envenom::ENVENOM,
        (CardName::Envenom, true) => envenom::ENVENOM_PLUS,
        (CardName::EscapePlan, false) => escape_plan::ESCAPE_PLAN,
        (CardName::EscapePlan, true) => escape_plan::ESCAPE_PLAN_PLUS,
        (CardName::Eviscerate, false) => eviscerate::EVISCERATE,
        (CardName::Eviscerate, true) => eviscerate::EVISCERATE_PLUS,
        (CardName::Expertise, false) => expertise::EXPERTISE,
        (CardName::Expertise, true) => expertise::EXPERTISE_PLUS,
        (CardName::Finesse, false) => finesse::FINESSE,
        (CardName::Finesse, true) => finesse::FINESSE_PLUS,
        (CardName::Finisher, false) => finisher::FINISHER,
        (CardName::Finisher, true) => finisher::FINISHER_PLUS,
        (CardName::FlashOfSteel, false) => flash_of_steel::FLASH_OF_STEEL,
        (CardName::FlashOfSteel, true) => flash_of_steel::FLASH_OF_STEEL_PLUS,
        (CardName::Flechettes, false) => flechettes::FLECHETTES,
        (CardName::Flechettes, true) => flechettes::FLECHETTES_PLUS,
        (CardName::FlyingKnee, false) => flying_knee::FLYING_KNEE,
        (CardName::FlyingKnee, true) => flying_knee::FLYING_KNEE_PLUS,
        (CardName::Footwork, false) => footwork::FOOTWORK,
        (CardName::Footwork, true) => footwork::FOOTWORK_PLUS,
        (CardName::GlassKnife, false) => glass_knife::GLASS_KNIFE,
        (CardName::GlassKnife, true) => glass_knife::GLASS_KNIFE_PLUS,
        (CardName::GoodInstincts, false) => good_instincts::GOOD_INSTINCTS,
        (CardName::GoodInstincts, true) => good_instincts::GOOD_INSTINCTS_PLUS,
        (CardName::GrandFinale, false) => grand_finale::GRAND_FINALE,
        (CardName::GrandFinale, true) => grand_finale::GRAND_FINALE_PLUS,
        (CardName::HeelHook, false) => heel_hook::HEEL_HOOK,
        (CardName::HeelHook, true) => heel_hook::HEEL_HOOK_PLUS,
        (CardName::InfiniteBlades, false) => infinite_blades::INFINITE_BLADES,
        (CardName::InfiniteBlades, true) => infinite_blades::INFINITE_BLADES_PLUS,
        (CardName::LegSweep, false) => leg_sweep::LEG_SWEEP,
        (CardName::LegSweep, true) => leg_sweep::LEG_SWEEP_PLUS,
        (CardName::Malaise, false) => malaise::MALAISE,
        (CardName::Malaise, true) => malaise::MALAISE_PLUS,
        (CardName::MasterOfStrategy, false) => master_of_strategy::MASTER_OF_STRATEGY,
        (CardName::MasterOfStrategy, true) => master_of_strategy::MASTER_OF_STRATEGY_PLUS,
        (CardName::MasterfulStab, false) => masterful_stab::MASTERFUL_STAB,
        (CardName::MasterfulStab, true) => masterful_stab::MASTERFUL_STAB_PLUS,
        (CardName::MindBlast, false) => mind_blast::MIND_BLAST,
        (CardName::MindBlast, true) => mind_blast::MIND_BLAST_PLUS,
        (CardName::Neutralize, false) => neutralize::NEUTRALIZE,
        (CardName::Neutralize, true) => neutralize::NEUTRALIZE_PLUS,
        (CardName::Nightmare, false) => nightmare::NIGHTMARE,
        (CardName::Nightmare, true) => nightmare::NIGHTMARE_PLUS,
        (CardName::NoxiousFumes, false) => noxious_fumes::NOXIOUS_FUMES,
        (CardName::NoxiousFumes, true) => noxious_fumes::NOXIOUS_FUMES_PLUS,
        (CardName::Outmaneuver, false) => outmaneuver::OUTMANEUVER,
        (CardName::Outmaneuver, true) => outmaneuver::OUTMANEUVER_PLUS,
        (CardName::PhantasmalKiller, false) => phantasmal_killer::PHANTASMAL_KILLER,
        (CardName::PhantasmalKiller, true) => phantasmal_killer::PHANTASMAL_KILLER_PLUS,
        (CardName::PiercingWail, false) => piercing_wail::PIERCING_WAIL,
        (CardName::PiercingWail, true) => piercing_wail::PIERCING_WAIL_PLUS,
        (CardName::PoisonedStab, false) => poisoned_stab::POISONED_STAB,
        (CardName::PoisonedStab, true) => poisoned_stab::POISONED_STAB_PLUS,
        (CardName::Predator, false) => predator::PREDATOR,
        (CardName::Predator, true) => predator::PREDATOR_PLUS,
        (CardName::Prepared, false) => prepared::PREPARED,
        (CardName::Prepared, true) => prepared::PREPARED_PLUS,
        (CardName::QuickSlash, false) => quick_slash::QUICK_SLASH,
        (CardName::QuickSlash, true) => quick_slash::QUICK_SLASH_PLUS,
        (CardName::Reflex, false) => reflex::REFLEX,
        (CardName::Reflex, true) => reflex::REFLEX_PLUS,
        (CardName::RiddleWithHoles, false) => riddle_with_holes::RIDDLE_WITH_HOLES,
        (CardName::RiddleWithHoles, true) => riddle_with_holes::RIDDLE_WITH_HOLES_PLUS,
        (CardName::Setup, false) => setup::SETUP,
        (CardName::Setup, true) => setup::SETUP_PLUS,
        (CardName::Shiv, false) => shiv::SHIV,
        (CardName::Shiv, true) => shiv::SHIV_PLUS,
        (CardName::Skewer, false) => skewer::SKEWER,
        (CardName::Skewer, true) => skewer::SKEWER_PLUS,
        (CardName::Slice, false) => slice::SLICE,
        (CardName::Slice, true) => slice::SLICE_PLUS,
        (CardName::Slimed, _) => slimed::SLIMED, // No upgraded variant
        (CardName::SneakyStrike, false) => sneaky_strike::SNEAKY_STRIKE,
        (CardName::SneakyStrike, true) => sneaky_strike::SNEAKY_STRIKE_PLUS,
        (CardName::StormOfSteel, false) => storm_of_steel::STORM_OF_STEEL,
        (CardName::StormOfSteel, true) => storm_of_steel::STORM_OF_STEEL_PLUS,
        (CardName::Strike, false) => strike::STRIKE,
        (CardName::Strike, true) => strike::STRIKE_PLUS,
        (CardName::SuckerPunch, false) => sucker_punch::SUCKER_PUNCH,
        (CardName::SuckerPunch, true) => sucker_punch::SUCKER_PUNCH_PLUS,
        (CardName::Survivor, false) => survivor::SURVIVOR,
        (CardName::Survivor, true) => survivor::SURVIVOR_PLUS,
        (CardName::SwiftStrike, false) => swift_strike::SWIFT_STRIKE,
        (CardName::SwiftStrike, true) => swift_strike::SWIFT_STRIKE_PLUS,
        (CardName::Tactician, false) => tactician::TACTICIAN,
        (CardName::Tactician, true) => tactician::TACTICIAN_PLUS,
        (CardName::Terror, false) => terror::TERROR,
        (CardName::Terror, true) => terror::TERROR_PLUS,
        (CardName::ToolsOfTheTrade, false) => tools_of_the_trade::TOOLS_OF_THE_TRADE,
        (CardName::ToolsOfTheTrade, true) => tools_of_the_trade::TOOLS_OF_THE_TRADE_PLUS,
        (CardName::Unload, false) => unload::UNLOAD,
        (CardName::Unload, true) => unload::UNLOAD_PLUS,
        (CardName::WellLaidPlans, false) => well_laid_plans::WELL_LAID_PLANS,
        (CardName::WellLaidPlans, true) => well_laid_plans::WELL_LAID_PLANS_PLUS,
        (CardName::WraithForm, false) => wraith_form::WRAITH_FORM,
        (CardName::WraithForm, true) => wraith_form::WRAITH_FORM_PLUS,
        (CardName::AscendersBane, _) => ascenders_bane::ASCENDERS_BANE,
        (CardName::Regret, _) => regret::REGRET,
        (CardName::Pain, _) => pain::PAIN,
        (CardName::Doubt, _) => doubt::DOUBT,
        (CardName::Decay, _) => decay::DECAY,
        (CardName::Injury, _) => injury::INJURY,
        (CardName::Shame, _) => shame::SHAME,
        (CardName::Writhe, _) => writhe::WRITHE,
        (CardName::Parasite, _) => parasite::PARASITE,
        (CardName::Normality, _) => normality::NORMALITY,
    }
}

pub const ALL_CARDS: &[&'static Entity] = &[
    &a_thousand_cuts::A_THOUSAND_CUTS,
    &accuracy::ACCURACY,
    &acrobatics::ACROBATICS,
    &adrenaline::ADRENALINE,
    &after_image::AFTER_IMAGE,
    &alchemize::ALCHEMIZE,
    &all_out_attack::ALL_OUT_ATTACK,
    &backflip::BACKFLIP,
    &backstab::BACKSTAB,
    &bandage_up::BANDAGE_UP,
    &bane::BANE,
    &blade_dance::BLADE_DANCE,
    &blind::BLIND,
    &blur::BLUR,
    &bouncing_flask::BOUNCING_FLASK,
    &bullet_time::BULLET_TIME,
    &burn::BURN,
    &burst::BURST,
    &calculated_gamble::CALCULATED_GAMBLE,
    &caltrops::CALTROPS,
    &catalyst::CATALYST,
    &choke::CHOKE,
    &cloak_and_dagger::CLOAK_AND_DAGGER,
    &concentrate::CONCENTRATE,
    &corpse_explosion::CORPSE_EXPLOSION,
    &crippling_poison::CRIPPLING_POISON,
    &dagger_spray::DAGGER_SPRAY,
    &dagger_throw::DAGGER_THROW,
    &dash::DASH,
    &dazed::DAZED,
    &deadly_poison::DEADLY_POISON,
    &deep_breath::DEEP_BREATH,
    &defend::DEFEND,
    &deflect::DEFLECT,
    &die_die_die::DIE_DIE_DIE,
    &distraction::DISTRACTION,
    &dodge_and_roll::DODGE_AND_ROLL,
    &doppelganger::DOPPELGANGER,
    &endless_agony::ENDLESS_AGONY,
    &envenom::ENVENOM,
    &escape_plan::ESCAPE_PLAN,
    &eviscerate::EVISCERATE,
    &expertise::EXPERTISE,
    &finesse::FINESSE,
    &finisher::FINISHER,
    &flash_of_steel::FLASH_OF_STEEL,
    &flechettes::FLECHETTES,
    &flying_knee::FLYING_KNEE,
    &footwork::FOOTWORK,
    &glass_knife::GLASS_KNIFE,
    &good_instincts::GOOD_INSTINCTS,
    &grand_finale::GRAND_FINALE,
    &heel_hook::HEEL_HOOK,
    &infinite_blades::INFINITE_BLADES,
    &leg_sweep::LEG_SWEEP,
    &malaise::MALAISE,
    &master_of_strategy::MASTER_OF_STRATEGY,
    &masterful_stab::MASTERFUL_STAB,
    &mind_blast::MIND_BLAST,
    &neutralize::NEUTRALIZE,
    &nightmare::NIGHTMARE,
    &noxious_fumes::NOXIOUS_FUMES,
    &outmaneuver::OUTMANEUVER,
    &phantasmal_killer::PHANTASMAL_KILLER,
    &piercing_wail::PIERCING_WAIL,
    &poisoned_stab::POISONED_STAB,
    &predator::PREDATOR,
    &prepared::PREPARED,
    &quick_slash::QUICK_SLASH,
    &reflex::REFLEX,
    &riddle_with_holes::RIDDLE_WITH_HOLES,
    &setup::SETUP,
    &shiv::SHIV,
    &skewer::SKEWER,
    &slice::SLICE,
    &slimed::SLIMED,
    &sneaky_strike::SNEAKY_STRIKE,
    &storm_of_steel::STORM_OF_STEEL,
    &strike::STRIKE,
    &sucker_punch::SUCKER_PUNCH,
    &survivor::SURVIVOR,
    &swift_strike::SWIFT_STRIKE,
    &tactician::TACTICIAN,
    &terror::TERROR,
    &tools_of_the_trade::TOOLS_OF_THE_TRADE,
    &unload::UNLOAD,
    &well_laid_plans::WELL_LAID_PLANS,
    &wraith_form::WRAITH_FORM,
    &ascenders_bane::ASCENDERS_BANE,
    &regret::REGRET,
    &pain::PAIN,
    &doubt::DOUBT,
    &decay::DECAY,
    &injury::INJURY,
    &shame::SHAME,
    &writhe::WRITHE,
    &parasite::PARASITE,
    &normality::NORMALITY,
];
// Assert all cards are included without duplicates
const _: () = assert!(ALL_CARDS.len() == CardName::COUNT);
const _: () = {
    let mut seen = [false; CardName::COUNT];
    let mut idx_all = 0;
    while idx_all < ALL_CARDS.len() {
        let idx_card = ALL_CARDS[idx_all].card_name as usize;
        assert!(!seen[idx_card], "ALL_CARDS contains a duplicate CardName");
        seen[idx_card] = true;
        idx_all += 1;
    }
};

const fn card_rarity_eq(lhs: CardRarity, rhs: CardRarity) -> bool {
    matches!(
        (lhs, rhs),
        (CardRarity::Basic, CardRarity::Basic)
            | (CardRarity::Common, CardRarity::Common)
            | (CardRarity::Uncommon, CardRarity::Uncommon)
            | (CardRarity::Rare, CardRarity::Rare)
            | (CardRarity::Special, CardRarity::Special)
            | (CardRarity::Curse, CardRarity::Curse)
    )
}

const fn card_color_eq(lhs: CardColor, rhs: CardColor) -> bool {
    matches!(
        (lhs, rhs),
        (CardColor::Green, CardColor::Green)
            | (CardColor::Colorless, CardColor::Colorless)
            | (CardColor::Curse, CardColor::Curse)
    )
}

const fn is_rewardable_kind(kind: CardKind) -> bool {
    matches!(
        kind,
        CardKind::Attack | CardKind::Skill | CardKind::Power | CardKind::Curse
    )
}

const fn count_pool(rarity: CardRarity, color: CardColor) -> usize {
    let mut count = 0;
    let mut idx = 0;
    while idx < ALL_CARDS.len() {
        let card = ALL_CARDS[idx];
        if card_rarity_eq(card.card_rarity, rarity)
            && card_color_eq(card.card_color, color)
            && is_rewardable_kind(card.card_kind)
        {
            // AscendersBane is Curse-rarity but Neow-only; skip
            if matches!(rarity, CardRarity::Curse)
                && matches!(card.card_name, CardName::AscendersBane)
            {
                idx += 1;
                continue;
            }
            count += 1;
        }
        idx += 1;
    }
    count
}

const fn build_pool<const N: usize>(rarity: CardRarity, color: CardColor) -> [CardName; N] {
    let mut buf = [CardName::Strike; N];
    let mut idx_pool = 0;
    let mut idx_all = 0;
    while idx_all < ALL_CARDS.len() {
        let card = ALL_CARDS[idx_all];
        if card_rarity_eq(card.card_rarity, rarity)
            && card_color_eq(card.card_color, color)
            && (matches!(rarity, CardRarity::Curse) || is_rewardable_kind(card.card_kind))
        {
            // AscendersBane is Curse-rarity but Neow-only; skip
            if matches!(card.card_name, CardName::AscendersBane) {
                idx_all += 1;
                continue;
            }
            buf[idx_pool] = card.card_name;
            idx_pool += 1;
        }
        idx_all += 1;
    }
    buf
}

// Pool sizes by (rarity, color) — Green
const NUM_COMMON_GREEN: usize = count_pool(CardRarity::Common, CardColor::Green);
const NUM_UNCOMMON_GREEN: usize = count_pool(CardRarity::Uncommon, CardColor::Green);
const NUM_RARE_GREEN: usize = count_pool(CardRarity::Rare, CardColor::Green);

// Colorless
const NUM_UNCOMMON_COLORLESS: usize = count_pool(CardRarity::Uncommon, CardColor::Colorless);
const NUM_RARE_COLORLESS: usize = count_pool(CardRarity::Rare, CardColor::Colorless);

// Curse
const NUM_CURSE: usize = count_pool(CardRarity::Curse, CardColor::Curse);

// Pools by (rarity, color) — Green
pub const POOL_COMMON_GREEN_CARD: &[CardName] =
    &build_pool::<NUM_COMMON_GREEN>(CardRarity::Common, CardColor::Green);
pub const POOL_UNCOMMON_GREEN_CARD: &[CardName] =
    &build_pool::<NUM_UNCOMMON_GREEN>(CardRarity::Uncommon, CardColor::Green);
pub const POOL_RARE_GREEN_CARD: &[CardName] =
    &build_pool::<NUM_RARE_GREEN>(CardRarity::Rare, CardColor::Green);

// Colorless
pub const POOL_UNCOMMON_COLORLESS_CARD: &[CardName] =
    &build_pool::<NUM_UNCOMMON_COLORLESS>(CardRarity::Uncommon, CardColor::Colorless);
pub const POOL_RARE_COLORLESS_CARD: &[CardName] =
    &build_pool::<NUM_RARE_COLORLESS>(CardRarity::Rare, CardColor::Colorless);

// Curse
pub const POOL_CURSE_CARD: &[CardName] =
    &build_pool::<NUM_CURSE>(CardRarity::Curse, CardColor::Curse);

// Pick `count` distinct cards from the full set, filtered by color and (when given) kind/rarity
pub fn get_random_cards(
    color: CardColor,
    kind: Option<CardKind>,
    rarity: Option<CardRarity>,
    exclude: &[CardName],
    count: usize,
    rng: &mut impl rand::Rng,
) -> Vec<Entity> {
    let mut pool: Vec<Entity> = ALL_CARDS
        .iter()
        .filter(|c| c.card_color == color)
        .filter(|c| kind.is_none_or(|k| c.card_kind == k))
        .filter(|c| rarity.is_none_or(|r| c.card_rarity == r))
        .filter(|c| !exclude.contains(&c.card_name))
        .map(|c| **c)
        .collect();

    shuffle(&mut pool, rng);
    pool.truncate(count);
    pool
}
