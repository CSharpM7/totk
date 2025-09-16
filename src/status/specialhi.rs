use crate::imports::imports_agent::*;

unsafe extern "C" fn specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut kinetic = *FIGHTER_KINETIC_TYPE_UNIQ;
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        kinetic, 
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
        *FIGHTER_STATUS_ATTR_START_TURN as u32, 
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 
        0
    );
    0.into()
}

unsafe extern "C" fn specialhi_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    //WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL);
    fighter.sub_change_motion_by_situation(Hash40::new("special_hi").into(), Hash40::new("special_air_hi").into(), false.into());
    let mot_g = hash40("special_hi");
    let mot_a = hash40("special_air_hi");
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let motion = if situation == *SITUATION_KIND_GROUND {mot_g} else {mot_a};
    WorkModule::set_int64(fighter.module_accessor, mot_g as i64,*FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, mot_a as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);

    //let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    //WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        specialhi_substatus(fighter);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(specialhi_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_main_loop as *const () as _))
}

unsafe extern "C" fn specialhi_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
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

unsafe extern "C" fn find_ascendable_ground(module_accessor: *mut BattleObjectModuleAccessor, pos_x: f32, min_pos_y: f32, pos_y: f32, height: f32) -> f32 {
    let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
    //println!("Cast from: {}", pos_y);
    if GroundModule::ray_check_hit_pos(module_accessor, &smash::phx::Vector2f{ x: pos_x, y: pos_y}, &Vector2f{ x: 0.0, y: -100.0}, ground_hit_pos, true)
    {
        if ground_hit_pos.y < min_pos_y {
            return pos_y;
        }
        //println!("Ground: {}!", ground_hit_pos.y);
        return find_ascendable_ground(module_accessor, pos_x, min_pos_y, ground_hit_pos.y-height, height);
    }
    else {
        return pos_y;
    }
}

unsafe extern "C" fn specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_hi").into(), Hash40::new("special_air_hi").into(), true.into());
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_KEEP));
            KineticModule::clear_speed_all(fighter.module_accessor);
        }
        else{
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    //fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_UNIQ.into());
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X){
        //println!("Searching for ground...");
        let pos_x = PostureModule::pos_x(fighter.module_accessor);
        let pos_y = PostureModule::pos_y(fighter.module_accessor);     
        let height = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);  
        let mut min_pos_y = pos_y;
        let lr = PostureModule::lr(fighter.module_accessor);   

        let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
        if GroundModule::ray_check_hit_pos(fighter.module_accessor, &smash::phx::Vector2f{ x: pos_x, y: pos_y}, &Vector2f{ x: 0.0, y: 100.0}, ground_hit_pos, true) {
            min_pos_y = ground_hit_pos.y;
            //println!("Ceiling at {pos_y}");
        }
        let ground = find_ascendable_ground(fighter.module_accessor, pos_x, min_pos_y+height, pos_y+100.0, height);
        if pos_y < ground && ground < pos_y+100.0
        {
            //println!("New ground y: {}!", ground);
            VarModule::set_float(fighter.battle_object, &mut link::instance::float::ASCEND_START_Y, pos_y);
            VarModule::set_float(fighter.battle_object, &mut link::instance::float::ASCEND_TARGET_Y, ground+5.0);

            fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD.into(), true.into());
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor){
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("link")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, specialhi_pre)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, specialhi_exec)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, specialhi_main)
        .install();
}