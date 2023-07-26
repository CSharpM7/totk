use crate::imports::imports_acmd::*;

#[acmd_script( agent = "link", scripts = ["game_specialhi","game_specialairhi"], category = ACMD_GAME)]
unsafe fn game_specialhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, &mut link::instance::flag::ASCEND_AIR_HOP)
        && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR
        {
            let lr = PostureModule::lr(fighter.module_accessor);   
            let speedx = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)*lr;
            SET_SPEED_EX(fighter,speedx,1.5,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        VarModule::off_flag(fighter.battle_object, &mut link::instance::flag::ASCEND_AIR_HOP);
        
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "link", scripts = ["effect_specialhi","effect_specialairhi"], category = ACMD_EFFECT)]
unsafe fn effect_specialhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 13, 0, -90, 0, 0, 0.625, true, *EF_FLIP_YZ);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.25, 1.00, 0.5);

        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_vector"), Hash40::new("sys_vector"), Hash40::new("top"), 0, 25, 0, -90, 0, 0, 2.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter,0,2,0.5);
        LAST_EFFECT_SET_SCALE_W(fighter,1.0,3.25,1);
    }
    frame(fighter.lua_state_agent, 14.0);
    {
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_vector"),false,false);
    }
}
#[acmd_script( agent = "link", scripts = ["sound_specialhi","sound_specialairhi"], category = ACMD_SOUND)]
unsafe fn sound_specialhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_jump02"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_special_h01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_VC(fighter,Hash40::new("vc_link_ottotto"),0.5);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_squat"));
    }
}

#[acmd_script( agent = "link", scripts = ["expression_specialhi","expression_specialairhi"], category = ACMD_EXPRESSION)]
unsafe fn expression_specialhi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
}
pub fn install() {
    install_acmd_scripts!(
        game_specialhi,
        effect_specialhi,
        sound_specialhi,
        expression_specialhi,
    );
}