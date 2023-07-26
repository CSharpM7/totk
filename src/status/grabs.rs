use crate::imports::imports_agent::*;

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_CATCH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn catch_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_all_up") ,false,false);
    EFFECT_OFF_KIND(fighter, Hash40::new("link_bomb_aura") ,false,false);
    return original!(fighter);
}


pub fn install() {
    install_status_scripts!(
        catch_end,
    );
}