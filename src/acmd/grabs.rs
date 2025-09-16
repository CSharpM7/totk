use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

unsafe extern "C" fn effect_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_bomb_aura"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 2.5, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        LAST_EFFECT_SET_COLOR(agent, 0.2, 2.0, 0.0);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_status_all_up"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 0.25, false);
        LAST_EFFECT_SET_COLOR(agent, 0.2, 2.0, 0.0);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_status_all_up") ,false,false);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("link_bomb_aura") ,false,false);
    }
}

unsafe extern "C" fn sound_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_link_special_h10"));
    }
}

unsafe extern "C" fn expression_catch(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 19, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

unsafe extern "C" fn effect_catchdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_bomb_aura"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 2.5, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        LAST_EFFECT_SET_COLOR(agent, 0.2, 2.0, 0.0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_status_all_up"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 0.25, false);
        LAST_EFFECT_SET_COLOR(agent, 0.2, 2.0, 0.0);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_status_all_up") ,false,false);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("link_bomb_aura") ,false,false);
    }
}

unsafe extern "C" fn sound_catchdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_link_special_h10"));
    }
}

unsafe extern "C" fn expression_catchdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 19, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.0, 0.0, 7.5, -13.0, Some(0.0), Some(7.5), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

unsafe extern "C" fn effect_catchturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_bomb_aura"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 2.5, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        LAST_EFFECT_SET_COLOR(agent, 0.2, 2.0, 0.0);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_status_all_up"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 0.25, false);
        LAST_EFFECT_SET_COLOR(agent, 0.2, 2.0, 0.0);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_status_all_up") ,false,false);
        EFFECT_OFF_KIND(agent, Hash40::new("link_bomb_aura") ,false,false);
    }
}

unsafe extern "C" fn sound_catchturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_link_special_h10"));
    }
}

unsafe extern "C" fn expression_catchturn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 19, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

unsafe extern "C" fn effect_catchpull(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_status_all_up") ,false,false);
        EFFECT_OFF_KIND(agent, Hash40::new("link_bomb_aura") ,false,false);
    }
}

unsafe extern "C" fn sound_catchpull(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        STOP_SE(agent,Hash40::new("se_link_special_h10"));
        PLAY_SE(agent,Hash40::new("se_link_special_h11"));
    }
}

unsafe extern "C" fn expression_catchpull(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

pub fn install() {
    Agent::new("link")
        .acmd("game_catch", game_catch, Priority::Low)
        .acmd("effect_catch", effect_catch, Priority::Low)
        .acmd("sound_catch", sound_catch, Priority::Low)
        .acmd("expression_catch", expression_catch, Priority::Low)
        .acmd("game_catchdash", game_catchdash, Priority::Low)
        .acmd("effect_catchdash", effect_catchdash, Priority::Low)
        .acmd("sound_catchdash", sound_catchdash, Priority::Low)
        .acmd("expression_catchdash", expression_catchdash, Priority::Low)
        .acmd("game_catchturn", game_catchturn, Priority::Low)
        .acmd("effect_catchturn", effect_catchturn, Priority::Low)
        .acmd("sound_catchturn", sound_catchturn, Priority::Low)
        .acmd("expression_catchturn", expression_catchturn, Priority::Low)
        .acmd("effect_catchpull", effect_catchpull, Priority::Low)
        .acmd("sound_catchpull", sound_catchpull, Priority::Low)
        .acmd("expression_catchpull", expression_catchpull, Priority::Low)
        .install();
}