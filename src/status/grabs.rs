use crate::imports::imports_agent::*;

unsafe extern "C" fn catch_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_all_up") ,false,false);
    EFFECT_OFF_KIND(fighter, Hash40::new("link_bomb_aura") ,false,false);
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_CATCH)(fighter)
}

pub fn install() {
    Agent::new("link")
        .status(End, *FIGHTER_STATUS_KIND_CATCH, catch_end)
        .install();
}