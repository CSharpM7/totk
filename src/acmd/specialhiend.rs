use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialhiend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent,0.5);
    frame(agent.lua_state_agent, 6.0);
    FT_MOTION_RATE(agent,1.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
    }
}

unsafe extern "C" fn effect_specialhiend(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent,0.25,1.0,0.5);
        LAST_EFFECT_SET_RATE(agent,1.1);
        frame(agent.lua_state_agent, 7.0);
    }
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialhiend(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_link_special_h04"));
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_VC(agent,Hash40::new("vc_link_cliffcatch"),0.5);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_jump02"));
    }
    wait(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}

unsafe extern "C" fn expression_specialhiend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
pub fn install() {
    Agent::new("link")  
        .acmd("game_specialhiend", game_specialhiend, Priority::Low)
        .acmd("effect_specialhiend", effect_specialhiend, Priority::Low)
        .acmd("sound_specialhiend", sound_specialhiend, Priority::Low)
        .acmd("expression_specialhiend", expression_specialhiend, Priority::Low)
        .install();
}