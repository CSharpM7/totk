use crate::imports::imports_agent::*;

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn specialhiend_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_AIR_STOP,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
    GroundModule::set_collidable(fighter.module_accessor, true);
    GroundModule::set_gr_collision_mode(fighter.module_accessor, true);
    JostleModule::set_status(fighter.module_accessor, true);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
    KineticModule::clear_speed_all(fighter.module_accessor);

    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    0.into()
}

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn specialhiend_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}
#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn specialhiend_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("Finishing up special");
    let mut motion = Hash40::new("special_hi_end");
    PostureModule::add_pos(fighter.module_accessor, &Vector3f{ x: 0.0, y: 0.0, z:0.0});
    MotionModule::change_motion(
        fighter.module_accessor,
        motion,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("rslash_landing_frame"), 0);
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

    if !StopModule::is_stop(fighter.module_accessor) {
        specialhiend_substatus(fighter);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(specialhiend_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(specialhiend_main_loop as *const () as _))
}

unsafe extern "C" fn specialhiend_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST){
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL);
        SET_SPEED_EX(fighter,0.0,2.0,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
    }
    0.into()
}

unsafe extern "C" fn specialhiend_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {

    if MotionModule::is_end(fighter.module_accessor){
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), FIGHTER_STATUS_KIND_FALL_AERIAL.into(),false.into());
    }
    0.into()
}

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn specialhiend_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}


pub fn install() {
    install_status_scripts!(
        specialhiend_pre, 
        specialhiend_exec, 
        specialhiend_end, 
        specialhiend_main
    );
}