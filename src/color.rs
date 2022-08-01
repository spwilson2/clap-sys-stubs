#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::color::*;

pub trait ClapColorInit {
    fn new() -> Self;
}

impl ClapColorInit for clap_color {
    fn new() -> Self {
        Self {
            alpha: todo!(),
            red: todo!(),
            green: todo!(),
            blue: todo!(),
        }
    }
}
