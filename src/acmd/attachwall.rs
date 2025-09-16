use crate::imports::imports_acmd::*;

unsafe extern "C" fn expression_attachwall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

unsafe extern "C" fn sound_attachwall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_link_step_left_s_ft"));
        macros::STOP_SE(agent, Hash40::new("se_link_step_right_s_ft"));
    }
}

unsafe extern "C" fn expression_attachwallclimb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 1.0);
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

unsafe extern "C" fn sound_attachwallclimb(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_link_step_right_s_ft"));
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_link_step_left_s_ft"));
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

pub fn install() {
    Agent::new("link")
        .acmd("expression_attachwall", expression_attachwall, Priority::Low)
        .acmd("sound_attachwall", sound_attachwall, Priority::Low)
        .acmd("expression_attachwallclimb", expression_attachwallclimb, Priority::Low)
        .acmd("sound_attachwallclimb", sound_attachwallclimb, Priority::Low)
        .install();
}