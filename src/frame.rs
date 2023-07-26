use crate::imports::imports_agent::*;

#[fighter_frame( agent = FIGHTER_KIND_LINK )]
fn link_update(fighter: &mut L2CFighterCommon) {
    unsafe {

    }
}


pub fn install() {
    smashline::install_agent_frames!(
        link_update
    );
}