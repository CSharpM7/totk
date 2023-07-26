use crate::imports::imports_acmd::*;

#[acmd_script( agent = "link", scripts = ["game_specialhistart"], category = ACMD_GAME)]
unsafe fn game_specialhistart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter,0.5);
    if macros::is_excute(fighter) {       
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
}

#[acmd_script( agent = "link", scripts = ["effect_specialhistart"], category = ACMD_EFFECT)]
unsafe fn effect_specialhistart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {        
        let target_y = VarModule::get_float(fighter.battle_object, &mut link::instance::float::ASCEND_TARGET_Y);
        let pos_y = PostureModule::pos_y(fighter.module_accessor);
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, target_y-pos_y+5.0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter,0.25,1.0,0.5);

        EFFECT(fighter, Hash40::new("sys_assist_out"), Hash40::new("top"), 0, -2.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter,0.25,1,0.5);
        LAST_EFFECT_SET_RATE(fighter,1.25);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("hip"), 0, 0.0, 0, 0, 0, 0, 3.0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {       
        EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter,0.25,1.0,0.5);
        LAST_EFFECT_SET_RATE(fighter,1.1);

    }
}
#[acmd_script( agent = "link", scripts = ["sound_specialhistart"], category = ACMD_SOUND)]
unsafe fn sound_specialhistart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {        
        STOP_SE(fighter,Hash40::new("se_link_special_h01"));
        PLAY_SE(fighter,Hash40::new("se_link_special_h02"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_VC(fighter,Hash40::new("vc_link_jump01"),1.5);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_escapeair"));
    }
}
#[acmd_script( agent = "link", scripts = ["expression_specialhistart"], category = ACMD_EXPRESSION)]
unsafe fn expression_specialhistart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {  
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32); 
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);  
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

#[acmd_script( agent = "link", scripts = ["game_specialhihold"], category = ACMD_GAME)]
unsafe fn game_specialhihold(fighter: &mut L2CAgentBase) {

}
#[acmd_script( agent = "link", scripts = ["effect_specialhihold"], category = ACMD_EFFECT)]
unsafe fn effect_specialhihold(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {        
        let target_y = VarModule::get_float(fighter.battle_object, &mut link::instance::float::ASCEND_TARGET_Y);
        let pos_y = PostureModule::pos_y(fighter.module_accessor);
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, target_y-pos_y+5.0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter,0.25,1.0,0.5);

        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("hip"), 0, 0.0, 0, 0, 0, 0, 3.0, false);
    }

    for _ in 0..i32::MAX {
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -4, 0, -90, 0, 0, 1.25, true, *EF_FLIP_YZ);
            macros::LAST_PARTICLE_SET_COLOR(fighter, 0.25, 1.0, 0.5);
            macros::FLASH(fighter, 0.25, 1.0, 0.5, 0.0);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            //macros::FLASH(fighter, 0.25, 1, 0.5, 0.05);
        }
        /* 
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0.25, 1, 0.5, 0.3);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0.25, 1, 0.5, 0.3);
        }*/
    }
}
#[acmd_script( agent = "link", scripts = ["sound_specialhihold"], category = ACMD_SOUND)]
unsafe fn sound_specialhihold(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {        
        PLAY_STATUS(fighter,Hash40::new("se_link_special_h03"));
    }
}

#[acmd_script( agent = "link", scripts = ["expression_specialhihold"], category = ACMD_EXPRESSION)]
unsafe fn expression_specialhihold(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}
pub fn install() {
    install_acmd_scripts!(
        game_specialhistart,
        effect_specialhistart,
        sound_specialhistart,
        expression_specialhistart,

        game_specialhihold,
        effect_specialhihold,
        sound_specialhihold,
        expression_specialhihold,
    );
}