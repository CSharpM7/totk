#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused,
    static_mut_refs
)]
#![deny(
    deprecated
)]
#[macro_use]
extern crate lazy_static;

mod imports;
pub mod vars;

mod acmd;
mod status;
mod agent;

#[no_mangle]
pub fn smashline_install() {
    //println!("[smashline_totk::main] Reloading...");
    install();
}

pub fn install() {    
    acmd::install();
    status::install();
    agent::install();
}

#[skyline::main(name = "smashline_totk")]
pub fn main() {
    #[cfg(feature = "dev")]{
        smashline_install();
        return;
    }
    #[cfg(feature = "devhook")]{
        return;
    }

    println!("[smashline_totk::main] Loading...");
    install();
    println!("[smashline_totk::main] Loaded!");
}