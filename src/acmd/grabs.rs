use crate::imports::imports_acmd::*;

#[acmd_script( agent = "link", script = "game_catch", category = ACMD_GAME)]
unsafe fn game_catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}
#[acmd_script( agent = "link", script = "effect_catch", category = ACMD_EFFECT)]
unsafe fn effect_catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("link_bomb_aura"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 2.5, false);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_COLOR(fighter, 0.2, 2.0, 0.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_status_all_up"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 0.25, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.2, 2.0, 0.0);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_all_up") ,false,false);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("link_bomb_aura") ,false,false);
    }
}

#[acmd_script( agent = "link", script = "sound_catch", category = ACMD_SOUND)]
unsafe fn sound_catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_special_h10"));
    }
}
#[acmd_script( agent = "link", script = "expression_catch", category = ACMD_EXPRESSION)]
unsafe fn expression_catch(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 19, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}


#[acmd_script( agent = "link", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn game_catchdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}
#[acmd_script( agent = "link", script = "effect_catchdash", category = ACMD_EFFECT)]
unsafe fn effect_catchdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("link_bomb_aura"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 2.5, false);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_COLOR(fighter, 0.2, 2.0, 0.0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_status_all_up"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 0.25, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.2, 2.0, 0.0);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_all_up") ,false,false);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("link_bomb_aura") ,false,false);
    }
}

#[acmd_script( agent = "link", script = "sound_catchdash", category = ACMD_SOUND)]
unsafe fn sound_catchdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_special_h10"));
    }
}
#[acmd_script( agent = "link", script = "expression_catchdash", category = ACMD_EXPRESSION)]
unsafe fn expression_catchdash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 19, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

#[acmd_script( agent = "link", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn game_catchturn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.0, 0.0, 7.5, -13.0, Some(0.0), Some(7.5), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}
#[acmd_script( agent = "link", script = "effect_catchturn", category = ACMD_EFFECT)]
unsafe fn effect_catchturn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("link_bomb_aura"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 2.5, false);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_COLOR(fighter, 0.2, 2.0, 0.0);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_status_all_up"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 0.25, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.2, 2.0, 0.0);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_all_up") ,false,false);
        EFFECT_OFF_KIND(fighter, Hash40::new("link_bomb_aura") ,false,false);
    }
}

#[acmd_script( agent = "link", script = "sound_catchturn", category = ACMD_SOUND)]
unsafe fn sound_catchturn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_special_h10"));
    }
}
#[acmd_script( agent = "link", script = "expression_catchturn", category = ACMD_EXPRESSION)]
unsafe fn expression_catchturn(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 19, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

#[acmd_script( agent = "link", script = "effect_catchpull", category = ACMD_EFFECT)]
unsafe fn effect_catchpull(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_all_up") ,false,false);
        EFFECT_OFF_KIND(fighter, Hash40::new("link_bomb_aura") ,false,false);
    }
}
#[acmd_script( agent = "link", script = "sound_catchpull", category = ACMD_SOUND)]
unsafe fn sound_catchpull(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        STOP_SE(fighter,Hash40::new("se_link_special_h10"));
        PLAY_SE(fighter,Hash40::new("se_link_special_h11"));
    }
}
#[acmd_script( agent = "link", script = "expression_catchpull", category = ACMD_EXPRESSION)]
unsafe fn expression_catchpull(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_catch,
        effect_catch,
        sound_catch,
        expression_catch,

        game_catchdash,
        effect_catchdash,
        sound_catchdash,
        expression_catchdash,

        game_catchturn,
        effect_catchturn,
        sound_catchturn,
        expression_catchturn,

        effect_catchpull,
        sound_catchpull,
        expression_catchpull,
    );
}