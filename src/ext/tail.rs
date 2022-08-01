#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::tail::*;
use clap_sys::{host::*, plugin::*};
use std::ffi::CStr;

unsafe extern "C" fn get(plugin: *const clap_plugin) -> u32 {
    todo!()
}

pub trait ClapPluginTailInit {
    fn new() -> Self;
}

impl ClapPluginTailInit for clap_plugin_tail {
    fn new() -> Self {
        Self { get: Some(get) }
    }
}

unsafe extern "C" fn changed(host: *const clap_host) {
    todo!()
}

pub trait ClapHostTailInit {
    fn new() -> Self;
}

impl ClapHostTailInit for clap_host_tail {
    fn new() -> Self {
        Self {
            changed: Some(changed),
        }
    }
}
