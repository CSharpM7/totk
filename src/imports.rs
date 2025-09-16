pub mod imports_acmd {
    pub use {
        crate::vars::*,
        
        sharpsmashlinesuite::{
            ext::*,
            util::{self, *},
            *,
        },

        smash::{
            app::{
                self,
                lua_bind::*,
                sv_animcmd::{frame, wait},
                *,
            },
            hash40,
            lib::lua_const::*,
            lua2cpp::*,
            phx::*,
        },
        smash_script::{macros::*, *},
        smashline::*,
    };
    pub unsafe extern "C" fn empty_acmd(agent: &mut L2CAgentBase) {}
}

pub mod imports_agent {
    pub use {
        crate::vars::*,

        sharpsmashlinesuite::{
            ext::*,
            util::{self, *},
            *,
        },

        smash::{
            app::{self, lua_bind::*, *},
            hash40,
            lib::{lua_const::*, L2CAgent, L2CValue},
            lua2cpp::*,
            phx::*,
        },
        smash_script::{macros::*, *},
        smashline::{Main, *},
    };
    pub unsafe extern "C" fn empty_status(agent: &mut L2CAgentBase) -> L2CValue {
        0.into()
    }
}
pub mod imports_status {
    pub use {
        crate::imports::imports_agent::*,
        smashline::{End, Exec, Init, Main, Pre, *},
    };
}
