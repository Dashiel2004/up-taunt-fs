#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros)]
mod fs;
#[skyline::main(name = "up_taunt_fs")]
pub fn main() {
    fs::install();
}
