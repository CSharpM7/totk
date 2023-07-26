use crate::imports::imports_acmd::*;

#[acmd_script( agent = "link", scripts = ["game_specialhiend"], category = ACMD_GAME)]
unsafe fn game_specialhiend(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter,0.5);
    frame(fighter.lua_state_agent, 6.0);
    FT_MOTION_RATE(fighter,1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
    }
}

#[acmd_script( agent = "link", scripts = ["effect_specialhiend"], category = ACMD_EFFECT)]
unsafe fn effect_specialhiend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter,0.25,1.0,0.5);
        LAST_EFFECT_SET_RATE(fighter,1.1);
        frame(fighter.lua_state_agent, 7.0);
    }
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        //macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "link", scripts = ["sound_specialhiend"], category = ACMD_SOUND)]
unsafe fn sound_specialhiend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_special_h04"));
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_VC(fighter,Hash40::new("vc_link_cliffcatch"),0.5);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_link_jump02"));
    }
    wait(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_05"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_05"));
    }
}

#[acmd_script( agent = "link", scripts = ["expression_specialhiend"], category = ACMD_EXPRESSION)]
unsafe fn expression_specialhiend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
pub fn install() {
    install_acmd_scripts!(
        game_specialhiend,
        effect_specialhiend,
        sound_specialhiend,
        expression_specialhiend,
    );
}