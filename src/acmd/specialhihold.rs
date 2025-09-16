use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent,0.5);
    if macros::is_excute(agent) {       
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
}

unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {        
        let target_y = VarModule::get_float(agent.battle_object, &mut link::instance::float::ASCEND_TARGET_Y);
        let pos_y = PostureModule::pos_y(agent.module_accessor);
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, target_y-pos_y+5.0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent,0.25,1.0,0.5);

        EFFECT(agent, Hash40::new("sys_assist_out"), Hash40::new("top"), 0, -2.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent,0.25,1,0.5);
        LAST_EFFECT_SET_RATE(agent,1.25);
        EFFECT_FOLLOW(agent, Hash40::new("sys_aura_light"), Hash40::new("hip"), 0, 0.0, 0, 0, 0, 0, 3.0, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {       
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent,0.25,1.0,0.5);
        LAST_EFFECT_SET_RATE(agent,1.1);

    }
}

unsafe extern "C" fn sound_specialhistart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {        
        STOP_SE(agent,Hash40::new("se_link_special_h01"));
        PLAY_SE(agent,Hash40::new("se_link_special_h02"));
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_VC(agent,Hash40::new("vc_link_jump01"),1.5);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_link_escapeair"));
    }
}

unsafe extern "C" fn expression_specialhistart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {  
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32); 
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);  
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

unsafe extern "C" fn game_specialhihold(agent: &mut L2CAgentBase) {

}

unsafe extern "C" fn effect_specialhihold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {        
        let target_y = VarModule::get_float(agent.battle_object, &mut link::instance::float::ASCEND_TARGET_Y);
        let pos_y = PostureModule::pos_y(agent.module_accessor);
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, target_y-pos_y+5.0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent,0.25,1.0,0.5);

        EFFECT_FOLLOW(agent, Hash40::new("sys_aura_light"), Hash40::new("hip"), 0, 0.0, 0, 0, 0, 0, 3.0, false);
    }

    for _ in 0..i32::MAX {
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -4, 0, -90, 0, 0, 1.25, true, *EF_FLIP_YZ);
            macros::LAST_PARTICLE_SET_COLOR(agent, 0.25, 1.0, 0.5);
            macros::FLASH(agent, 0.25, 1.0, 0.5, 0.0);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.25, 1, 0.5, 0.05);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.25, 1, 0.5, 0.3);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.25, 1, 0.5, 0.3);
        }
    }
}

unsafe extern "C" fn sound_specialhihold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {        
        PLAY_STATUS(agent,Hash40::new("se_link_special_h03"));
    }
}

unsafe extern "C" fn expression_specialhihold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}
pub fn install() {
    Agent::new("link")
        .acmd("game_specialhistart", game_specialhistart, Priority::Low)
        .acmd("effect_specialhistart", effect_specialhistart, Priority::Low)
        .acmd("sound_specialhistart", sound_specialhistart, Priority::Low)
        .acmd("expression_specialhistart", expression_specialhistart, Priority::Low)
        .acmd("game_specialhihold", game_specialhihold, Priority::Low)
        .acmd("effect_specialhihold", effect_specialhihold, Priority::Low)
        .acmd("sound_specialhihold", sound_specialhihold, Priority::Low)
        .acmd("expression_specialhihold", expression_specialhihold, Priority::Low)
        .install();
}