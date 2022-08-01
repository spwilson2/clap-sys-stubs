#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::draft::surround::*;
use clap_sys::{host::*, plugin::*};
use std::ffi::CStr;

unsafe extern "C" fn get_channel_map(
    plugin: *const clap_plugin,
    is_input: bool,
    port_index: u32,
    channel_map: *mut u8,
    channel_map_capacity: u32,
) -> u32 {
    todo!()
}

unsafe extern "C" fn changed(plugin: *const clap_plugin) {
    todo!()
}

pub trait ClapPluginSurroundInit {
    fn new() -> Self;
}

impl ClapPluginSurroundInit for clap_plugin_surround {
    fn new() -> Self {
        Self {
            get_channel_map: Some(get_channel_map),
            changed: Some(changed),
        }
    }
}

unsafe extern "C" fn changed_host(host: *const clap_host) {
    todo!()
}

unsafe extern "C" fn get_preferred_channel_map(
    plugin: *const clap_host,
    channel_map: *mut u8,
    channel_map_capacity: u32,
    channel_count: *mut u32,
) {
    todo!()
}

pub trait ClapHostSurroundInit {
    fn new() -> Self;
}

impl ClapHostSurroundInit for clap_host_surround {
    fn new() -> Self {
        Self {
            changed: Some(changed_host),
            get_preferred_channel_map: Some(get_preferred_channel_map),
        }
    }
}
