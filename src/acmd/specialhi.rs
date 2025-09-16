use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, &mut link::instance::flag::ASCEND_AIR_HOP) && StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_AIR {
            let lr = PostureModule::lr(agent.module_accessor);   
            let speedx = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)*lr;
            SET_SPEED_EX(agent,speedx,1.5,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        VarModule::off_flag(agent.battle_object, &mut link::instance::flag::ASCEND_AIR_HOP);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 13, 0, -90, 0, 0, 0.625, true, *EF_FLIP_YZ);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.25, 1.00, 0.5);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_vector"), Hash40::new("sys_vector"), Hash40::new("top"), 0, 25, 0, -90, 0, 0, 2.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(agent,0,2,0.5);
        LAST_EFFECT_SET_SCALE_W(agent,1.0,3.25,1);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("sys_vector"),false,false);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_link_jump02"));
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_link_special_h01"));
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        PLAY_VC(agent,Hash40::new("vc_link_ottotto"),0.5);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_link_squat"));
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
}

pub fn install() {
    Agent::new("link")
        .acmd("game_specialhi", game_specialhi, Priority::Low)
        .acmd("game_specialairhi", game_specialhi, Priority::Low)
        .acmd("effect_specialhi", effect_specialhi, Priority::Low)
        .acmd("effect_specialairhi", effect_specialhi, Priority::Low)
        .acmd("sound_specialhi", sound_specialhi, Priority::Low)
        .acmd("sound_specialairhi", sound_specialhi, Priority::Low)
        .acmd("expression_specialhi", expression_specialhi, Priority::Low)
        .acmd("expression_specialairhi", expression_specialhi, Priority::Low)
        .install();
}