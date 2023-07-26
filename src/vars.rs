//use super::*;
//use crate::imports::imports_acmd::*;
use smash::phx::Vector3f;
/* 
pub mod link {
    pub mod instance {
        pub mod flag {
            pub const ASCEND_AIR_HOP: i32 = 0x0100;
        }
        pub mod int {
        }
        pub mod float {
            pub const ASCEND_START_Y: i32 = 0x0100;
            pub const ASCEND_TARGET_Y: i32 = 0x0101;
        }
    }
    pub mod status {
        pub mod int {
        }
    }
}*/
pub mod link {
    pub mod instance {
        pub mod flag {
            pub static mut ASCEND_AIR_HOP:[bool;8] = [false; 8];
        }
        pub mod int {
        }
        pub mod float {
            pub static mut ASCEND_START_Y:[f32;8] = [0.0; 8];
            pub static mut ASCEND_TARGET_Y:[f32;8] = [0.0; 8];
        }
    }
    pub mod status {
        pub mod int {
        }
    }
}
use {
    smash::{
        phx::*,
        app::*,
    },
    sharpsmashlinesuite::{
        VarModule::*,
    }
};
pub unsafe fn reset_vars(object: *mut BattleObject){
    let entry = get_entry_from_object(object);
    link::instance::flag::ASCEND_AIR_HOP[entry] = false;
    link::instance::float::ASCEND_START_Y[entry] = 0.0;
    link::instance::float::ASCEND_TARGET_Y[entry] = 0.0;
}