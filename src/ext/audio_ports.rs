#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::audio_ports::*;
use clap_sys::{host::*, id::*, plugin::*, string_sizes::*};
use std::ffi::CStr;
use std::os::raw::c_char;

pub trait ClapAudioPortInfoInit {
    fn new() -> Self;
}

impl ClapAudioPortInfoInit for clap_audio_port_info {
    fn new() -> Self {
        Self {
            id: todo!(),
            name: todo!(),
            flags: todo!(),
            channel_count: todo!(),
            port_type: todo!(),
            in_place_pair: todo!(),
        }
    }
}

unsafe extern "C" fn count(plugin: *const clap_plugin, is_input: bool) -> u32 {
    todo!()
}

unsafe extern "C" fn get(
    plugin: *const clap_plugin,
    index: u32,
    is_input: bool,
    info: *mut clap_audio_port_info,
) -> bool {
    todo!()
}

pub trait ClapPluginAudioPortsInit {
    fn new() -> Self;
}

impl ClapPluginAudioPortsInit for clap_plugin_audio_ports {
    fn new() -> Self {
        Self {
            count: Some(count),
            get: Some(get),
        }
    }
}

unsafe extern "C" fn is_rescan_flag_supported(host: *const clap_host, flag: u32) -> bool {
    todo!()
}

unsafe extern "C" fn rescan(host: *const clap_host, flags: u32) {
    todo!()
}

pub trait ClapHostAudioPortsInit {
    fn new() -> Self;
}

impl ClapHostAudioPortsInit for clap_host_audio_ports {
    fn new() -> Self {
        Self {
            is_rescan_flag_supported: Some(is_rescan_flag_supported),
            rescan: Some(rescan),
        }
    }
}
