use crate::imports::imports_agent::*;

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;

    if (&[*SITUATION_KIND_GROUND,*SITUATION_KIND_CLIFF]).contains(&StatusModule::situation_kind(boma))
    || is_damage_status(boma) 
    {
        VarModule::on_flag(fighter.battle_object, &mut link::instance::flag::ASCEND_AIR_HOP)
    }
    true.into()
}

unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_LINK {
        return;
    }
    //GetVarManager::reset_var_module(fighter.battle_object,false);
    reset_vars(fighter.battle_object);
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}

#[smashline::fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}
#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(
        agent_init
    );
    install_agent_resets!(
        agent_reset
    );
}