pub mod a_thousand_cuts;
pub mod accuracy;
pub mod acrobatics;
pub mod adrenaline;
pub mod after_image;
pub mod all_out_attack;
pub mod backflip;
pub mod backstab;
pub mod bane;
pub mod blade_dance;
pub mod blur;
pub mod bouncing_flask;
pub mod bullet_time;
pub mod burn;
pub mod burst;
pub mod calculated_gamble;
pub mod caltrops;
pub mod catalyst;
pub mod choke;
pub mod cloak_and_dagger;
pub mod concentrate;
pub mod corpse_explosion;
pub mod crippling_poison;
pub mod dagger_spray;
pub mod dagger_throw;
pub mod dash;
pub mod dazed;
pub mod deadly_poison;
pub mod defend;
pub mod deflect;
pub mod die_die_die;
pub mod distraction;
pub mod dodge_and_roll;
pub mod doppelganger;
pub mod endless_agony;
pub mod envenom;
pub mod escape_plan;
pub mod eviscerate;
pub mod expertise;
pub mod finisher;
pub mod flechettes;
pub mod flying_knee;
pub mod footwork;
pub mod glass_knife;
pub mod grand_finale;
pub mod heel_hook;
pub mod infinite_blades;
pub mod leg_sweep;
pub mod malaise;
pub mod masterful_stab;
pub mod neutralize;
pub mod nightmare;
pub mod noxious_fumes;
pub mod outmaneuver;
pub mod phantasmal_killer;
pub mod piercing_wail;
pub mod poisoned_stab;
pub mod predator;
pub mod prepared;
pub mod quick_slash;
pub mod reflex;
pub mod riddle_with_holes;
pub mod setup;
pub mod shiv;
pub mod skewer;
pub mod slice;
pub mod slimed;
pub mod sneaky_strike;
pub mod storm_of_steel;
pub mod strike;
pub mod sucker_punch;
pub mod survivor;
pub mod tactician;
pub mod terror;
pub mod tools_of_the_trade;
pub mod unload;
pub mod well_laid_plans;
pub mod wraith_form;

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
        (CardName::Bane, false) => bane::BANE,
        (CardName::Bane, true) => bane::BANE_PLUS,
        (CardName::BladeDance, false) => blade_dance::BLADE_DANCE,
        (CardName::BladeDance, true) => blade_dance::BLADE_DANCE_PLUS,
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
        // Dazed (Status): no upgrade variant — both branches return the same entity.
        (CardName::Dazed, _) => dazed::DAZED,
        (CardName::DeadlyPoison, false) => deadly_poison::DEADLY_POISON,
        (CardName::DeadlyPoison, true) => deadly_poison::DEADLY_POISON_PLUS,
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
        (CardName::Finisher, false) => finisher::FINISHER,
        (CardName::Finisher, true) => finisher::FINISHER_PLUS,
        (CardName::Flechettes, false) => flechettes::FLECHETTES,
        (CardName::Flechettes, true) => flechettes::FLECHETTES_PLUS,
        (CardName::FlyingKnee, false) => flying_knee::FLYING_KNEE,
        (CardName::FlyingKnee, true) => flying_knee::FLYING_KNEE_PLUS,
        (CardName::Footwork, false) => footwork::FOOTWORK,
        (CardName::Footwork, true) => footwork::FOOTWORK_PLUS,
        (CardName::GlassKnife, false) => glass_knife::GLASS_KNIFE,
        (CardName::GlassKnife, true) => glass_knife::GLASS_KNIFE_PLUS,
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
        (CardName::MasterfulStab, false) => masterful_stab::MASTERFUL_STAB,
        (CardName::MasterfulStab, true) => masterful_stab::MASTERFUL_STAB_PLUS,
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
        // Slimed (Status): no upgrade variant.
        (CardName::Slimed, _) => slimed::SLIMED,
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
    }
}

// All card names eligible for card rewards (excludes Basic and Special)
pub const REWARD_POOL_COMMON: &[CardName] = &[
    CardName::Acrobatics,
    CardName::Backflip,
    CardName::Bane,
    CardName::BladeDance,
    CardName::CloakAndDagger,
    CardName::DaggerSpray,
    CardName::DaggerThrow,
    CardName::DeadlyPoison,
    CardName::Deflect,
    CardName::DodgeAndRoll,
    CardName::FlyingKnee,
    CardName::Outmaneuver,
    CardName::PiercingWail,
    CardName::PoisonedStab,
    CardName::Prepared,
    CardName::QuickSlash,
    CardName::Slice,
    CardName::SneakyStrike,
    CardName::SuckerPunch,
];

pub const REWARD_POOL_UNCOMMON: &[CardName] = &[
    CardName::Accuracy,
    CardName::AllOutAttack,
    CardName::Backstab,
    CardName::Blur,
    CardName::BouncingFlask,
    CardName::CalculatedGamble,
    CardName::Caltrops,
    CardName::Catalyst,
    CardName::Choke,
    CardName::Concentrate,
    CardName::CripplingPoison,
    CardName::Dash,
    CardName::Distraction,
    CardName::EndlessAgony,
    CardName::EscapePlan,
    CardName::Eviscerate,
    CardName::Expertise,
    CardName::Finisher,
    CardName::Flechettes,
    CardName::Footwork,
    CardName::HeelHook,
    CardName::InfiniteBlades,
    CardName::LegSweep,
    CardName::MasterfulStab,
    CardName::NoxiousFumes,
    CardName::Predator,
    CardName::Reflex,
    CardName::RiddleWithHoles,
    CardName::Setup,
    CardName::Skewer,
    CardName::Tactician,
    CardName::Terror,
    CardName::WellLaidPlans,
];

pub const REWARD_POOL_RARE: &[CardName] = &[
    CardName::AThousandCuts,
    CardName::Adrenaline,
    CardName::AfterImage,
    CardName::BulletTime,
    CardName::Burst,
    CardName::CorpseExplosion,
    CardName::DieDieDie,
    CardName::Doppelganger,
    CardName::Envenom,
    CardName::GlassKnife,
    CardName::GrandFinale,
    CardName::Malaise,
    CardName::Nightmare,
    CardName::PhantasmalKiller,
    CardName::StormOfSteel,
    CardName::ToolsOfTheTrade,
    CardName::Unload,
    CardName::WraithForm,
];
