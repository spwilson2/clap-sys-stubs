#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::version::*;

pub trait ClapVersionInit {
    fn new() -> Self;
}

impl ClapVersionInit for clap_version {
    fn new() -> Self {
        Self {
            major: todo!(),
            minor: todo!(),
            revision: todo!(),
        }
    }
}
