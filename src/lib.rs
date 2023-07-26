#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![deny(
    deprecated
)]

#[macro_use]
extern crate lazy_static;

pub mod imports;

mod acmd;
mod frame;
mod status;
mod agent;
pub mod data;
//mod custom_vars;
pub mod vars;
use data::gamemode::*;
//pub mod util;
//use util::*;

#[skyline::main(name = "smashline_totk")]
pub fn main() {
    println!("[smashline_totk::main] Loading...");
    //custom_vars::install();

    //data::install();
    data::gamemode::set_gamemode();
    acmd::install();
    //frame::install();
    status::install();
    agent::install();
    println!("[smashline_totk::main] Loaded!");
}