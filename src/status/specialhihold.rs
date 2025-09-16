use crate::imports::imports_agent::*;

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn specialhihold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    GroundModule::set_ignore_boss(fighter.module_accessor, true);
    GroundModule::set_passable_check(fighter.module_accessor, false);
    GroundModule::set_collidable(fighter.module_accessor, false);

    JostleModule::set_status(fighter.module_accessor, false);
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);


    0.into()
}

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn specialhihold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}
#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn specialhihold_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
    GroundModule::set_rhombus_offset(fighter.module_accessor,&Vector2f{ x: 0.0, y: 0.0});
    COL_NORMAL(fighter);
    macros::BURN_COLOR_NORMAL(fighter);
    //EFFECT_OFF_KIND(fighter, Hash40::new("sys_shield_smoke"),false,false);
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"),false,false);
    //STOP_SE(fighter,Hash40::new("se_link_special_h03"));
    0.into()
}

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn specialhihold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_hi_start"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let cbm_t_vec1 = Vector4f{ /* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 1.0};
    let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.25, /* Green */ y: 1.0, /* Blue */ z: 0.75, /* Alpha */ w: 0.5};
    ColorBlendModule::set_main_color(fighter.module_accessor, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);

    if !StopModule::is_stop(fighter.module_accessor) {
        specialhihold_substatus(fighter);
    }
    println!("Swimming Up");
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(specialhihold_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(specialhihold_main_loop as *const () as _))
}

unsafe extern "C" fn specialhihold_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    /* 
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_situation(SITUATION_KIND_AIR.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE);
    }*/
    0.into()
}

unsafe extern "C" fn specialhihold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {    
    if MotionModule::is_end(fighter.module_accessor){
        PostureModule::add_pos(fighter.module_accessor, &Vector3f{ x: 0.0, y: 11.6, z:0.0});
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_hi_hold"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }

    let current_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    let start_frame = 8.0;
    let max_frame = 10.0;
    {
        WorkModule::add_float(fighter.module_accessor, 1.0, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
        let pos_x = PostureModule::pos_x(fighter.module_accessor);
        let pos_y = PostureModule::pos_y(fighter.module_accessor);       
        let height = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);

        let target_y = VarModule::get_float(fighter.battle_object, link::instance::float::ASCEND_TARGET_Y);
        let start_y = VarModule::get_float(fighter.battle_object, link::instance::float::ASCEND_START_Y);
        let mut max_y = target_y +height+ 20.0;
        let lr = PostureModule::lr(fighter.module_accessor);   
        let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};

        //for x in (1..110).step_by(2)
        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_hi_hold")
        { 
            println!("Swimming to {target_y} from {pos_y}");

            SET_SPEED_EX(fighter,0.0,3.0,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

            if GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{ x: pos_x, y: pos_y+4.0}, &Vector2f{ x: 0.0, y: -height/1.5}, ground_hit_pos,true) == 1
            //&& pos_y >= start_y+height/1.25
            && pos_y >= max_y-(height*2.0)
            {
                println!("Confirmed ground...");
                PostureModule::set_pos(fighter.module_accessor, &Vector3f{ x: pos_x, y: ground_hit_pos.y, z:0.0});
                GroundModule::set_attach_ground(fighter.module_accessor, true);
                fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(),true.into());
                return 0.into();
            }
        }
        let verifyRate = 5.0;
		let modulo = current_frame % 10.0;
		if (modulo<1.0)
		{
            if GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: pos_x, y: target_y+5.0}, &Vector2f{ x: 0.0, y: -10.0},true) != 1
            {
                if GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{ x: pos_x, y: target_y+20.0}, &Vector2f{ x: 0.0, y: -40.0}, ground_hit_pos,true) == 1 {
                    println!("Ground is moving...");
                    VarModule::set_float(fighter.battle_object, link::instance::float::ASCEND_TARGET_Y,ground_hit_pos.y);
                    max_y = ground_hit_pos.y;
                }
                else {
                    println!("Ground moved away?");
                    max_y = -999.0;
                }
            }
        }

        if pos_y > max_y {
            println!("Ground is gone?!?");
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_damage_paralyze"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
            PLAY_SE(fighter,Hash40::new("vc_link_damage01"));
            SET_SPEED_EX(fighter,0.0,2.5,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            fighter.change_status(FIGHTER_STATUS_KIND_TREAD_FALL.into(),false.into());
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        specialhihold_pre, 
        specialhihold_exec, 
        specialhihold_exit,
        specialhihold_main
    );
}