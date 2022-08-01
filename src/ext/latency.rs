#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::latency::*;
use clap_sys::{host::*, plugin::*};
use std::ffi::CStr;

unsafe extern "C" fn get(plugin: *const clap_plugin) -> u32 {
    todo!()
}

pub trait ClapPluginLatencyInit {
    fn new() -> Self;
}

impl ClapPluginLatencyInit for clap_plugin_latency {
    fn new() -> Self {
        Self { get: Some(get) }
    }
}

unsafe extern "C" fn changed(host: *const clap_host) {
    todo!()
}

pub trait ClapHostLatencyInit {
    fn new() -> Self;
}

impl ClapHostLatencyInit for clap_host_latency {
    fn new() -> Self {
        Self {
            changed: Some(changed),
        }
    }
}
