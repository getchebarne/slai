pub mod a_thousand_cuts;
pub mod accuracy;
pub mod acrobatics;
pub mod adrenaline;
pub mod after_image;
pub mod all_out_attack;
pub mod backflip;
pub mod backstab;
pub mod blade_dance;
pub mod blur;
pub mod burst;
pub mod calculated_gamble;
pub mod cloak_and_dagger;
pub mod dagger_spray;
pub mod dagger_throw;
pub mod dash;
pub mod defend;
pub mod deflect;
pub mod die_die_die;
pub mod dodge_and_roll;
pub mod flying_knee;
pub mod footwork;
pub mod infinite_blades;
pub mod leg_sweep;
pub mod neutralize;
pub mod outmaneuver;
pub mod phantasmal_killer;
pub mod piercing_wail;
pub mod prepared;
pub mod shiv;
pub mod strike;
pub mod survivor;
pub mod terror;
pub mod well_laid_plans;

use crate::entity::Entity;
use crate::types::CardName;

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
        (CardName::AllOutAttack, false) => all_out_attack::ALL_OUT_ATTACK,
        (CardName::AllOutAttack, true) => all_out_attack::ALL_OUT_ATTACK_PLUS,
        (CardName::Backflip, false) => backflip::BACKFLIP,
        (CardName::Backflip, true) => backflip::BACKFLIP_PLUS,
        (CardName::Backstab, false) => backstab::BACKSTAB,
        (CardName::Backstab, true) => backstab::BACKSTAB_PLUS,
        (CardName::BladeDance, false) => blade_dance::BLADE_DANCE,
        (CardName::BladeDance, true) => blade_dance::BLADE_DANCE_PLUS,
        (CardName::Blur, false) => blur::BLUR,
        (CardName::Blur, true) => blur::BLUR_PLUS,
        (CardName::Burst, false) => burst::BURST,
        (CardName::Burst, true) => burst::BURST_PLUS,
        (CardName::CalculatedGamble, false) => calculated_gamble::CALCULATED_GAMBLE,
        (CardName::CalculatedGamble, true) => calculated_gamble::CALCULATED_GAMBLE_PLUS,
        (CardName::CloakAndDagger, false) => cloak_and_dagger::CLOAK_AND_DAGGER,
        (CardName::CloakAndDagger, true) => cloak_and_dagger::CLOAK_AND_DAGGER_PLUS,
        (CardName::DaggerSpray, false) => dagger_spray::DAGGER_SPRAY,
        (CardName::DaggerSpray, true) => dagger_spray::DAGGER_SPRAY_PLUS,
        (CardName::DaggerThrow, false) => dagger_throw::DAGGER_THROW,
        (CardName::DaggerThrow, true) => dagger_throw::DAGGER_THROW_PLUS,
        (CardName::Dash, false) => dash::DASH,
        (CardName::Dash, true) => dash::DASH_PLUS,
        (CardName::Defend, false) => defend::DEFEND,
        (CardName::Defend, true) => defend::DEFEND_PLUS,
        (CardName::Deflect, false) => deflect::DEFLECT,
        (CardName::Deflect, true) => deflect::DEFLECT_PLUS,
        (CardName::DieDieDie, false) => die_die_die::DIE_DIE_DIE,
        (CardName::DieDieDie, true) => die_die_die::DIE_DIE_DIE_PLUS,
        (CardName::DodgeAndRoll, false) => dodge_and_roll::DODGE_AND_ROLL,
        (CardName::DodgeAndRoll, true) => dodge_and_roll::DODGE_AND_ROLL_PLUS,
        (CardName::FlyingKnee, false) => flying_knee::FLYING_KNEE,
        (CardName::FlyingKnee, true) => flying_knee::FLYING_KNEE_PLUS,
        (CardName::Footwork, false) => footwork::FOOTWORK,
        (CardName::Footwork, true) => footwork::FOOTWORK_PLUS,
        (CardName::InfiniteBlades, false) => infinite_blades::INFINITE_BLADES,
        (CardName::InfiniteBlades, true) => infinite_blades::INFINITE_BLADES_PLUS,
        (CardName::LegSweep, false) => leg_sweep::LEG_SWEEP,
        (CardName::LegSweep, true) => leg_sweep::LEG_SWEEP_PLUS,
        (CardName::Neutralize, false) => neutralize::NEUTRALIZE,
        (CardName::Neutralize, true) => neutralize::NEUTRALIZE_PLUS,
        (CardName::Outmaneuver, false) => outmaneuver::OUTMANEUVER,
        (CardName::Outmaneuver, true) => outmaneuver::OUTMANEUVER_PLUS,
        (CardName::PhantasmalKiller, false) => phantasmal_killer::PHANTASMAL_KILLER,
        (CardName::PhantasmalKiller, true) => phantasmal_killer::PHANTASMAL_KILLER_PLUS,
        (CardName::PiercingWail, false) => piercing_wail::PIERCING_WAIL,
        (CardName::PiercingWail, true) => piercing_wail::PIERCING_WAIL_PLUS,
        (CardName::Prepared, false) => prepared::PREPARED,
        (CardName::Prepared, true) => prepared::PREPARED_PLUS,
        (CardName::Shiv, false) => shiv::SHIV,
        (CardName::Shiv, true) => shiv::SHIV_PLUS,
        (CardName::Strike, false) => strike::STRIKE,
        (CardName::Strike, true) => strike::STRIKE_PLUS,
        (CardName::Survivor, false) => survivor::SURVIVOR,
        (CardName::Survivor, true) => survivor::SURVIVOR_PLUS,
        (CardName::Terror, false) => terror::TERROR,
        (CardName::Terror, true) => terror::TERROR_PLUS,
        (CardName::WellLaidPlans, false) => well_laid_plans::WELL_LAID_PLANS,
        (CardName::WellLaidPlans, true) => well_laid_plans::WELL_LAID_PLANS_PLUS,
    }
}

// All card names eligible for card rewards (excludes Basic and Special)
pub const REWARD_POOL_COMMON: &[CardName] = &[
    CardName::Acrobatics,
    CardName::Backflip,
    CardName::BladeDance,
    CardName::CloakAndDagger,
    CardName::DaggerSpray,
    CardName::DaggerThrow,
    CardName::Deflect,
    CardName::DodgeAndRoll,
    CardName::FlyingKnee,
    CardName::Outmaneuver,
    CardName::PiercingWail,
    CardName::Prepared,
];

pub const REWARD_POOL_UNCOMMON: &[CardName] = &[
    CardName::Accuracy,
    CardName::AllOutAttack,
    CardName::Backstab,
    CardName::Blur,
    CardName::CalculatedGamble,
    CardName::Dash,
    CardName::Footwork,
    CardName::InfiniteBlades,
    CardName::LegSweep,
    CardName::Terror,
    CardName::WellLaidPlans,
];

pub const REWARD_POOL_RARE: &[CardName] = &[
    CardName::AThousandCuts,
    CardName::Adrenaline,
    CardName::AfterImage,
    CardName::Burst,
    CardName::DieDieDie,
    CardName::PhantasmalKiller,
];
