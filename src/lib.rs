#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros)]
mod agent;
#[skyline::main(name = "upTauntFs")]
pub fn main() {
    agent::install();
}

