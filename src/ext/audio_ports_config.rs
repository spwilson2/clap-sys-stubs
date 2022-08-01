#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::audio_ports_config::*;
use clap_sys::{host::*, id::*, plugin::*, string_sizes::*};
use std::ffi::CStr;
use std::os::raw::c_char;

pub trait ClapAudioPortsConfigInit {
    fn new() -> Self;
}

impl ClapAudioPortsConfigInit for clap_audio_ports_config {
    fn new() -> Self {
        Self {
            id: todo!(),
            name: todo!(),
            input_port_count: todo!(),
            output_port_count: todo!(),
            has_main_input: todo!(),
            main_input_channel_count: todo!(),
            main_input_port_type: todo!(),
            has_main_output: todo!(),
            main_output_channel_count: todo!(),
            main_output_port_type: todo!(),
        }
    }
}

unsafe extern "C" fn count(plugin: *const clap_plugin) -> u32 {
    todo!()
}

unsafe extern "C" fn get(
    plugin: *const clap_plugin,
    index: u32,
    config: *mut clap_audio_ports_config,
) -> bool {
    todo!()
}

unsafe extern "C" fn select(plugin: *const clap_plugin, config_id: clap_id) -> bool {
    todo!()
}

pub trait ClapPluginAudioPortsConfigInit {
    fn new() -> Self;
}

impl ClapPluginAudioPortsConfigInit for clap_plugin_audio_ports_config {
    fn new() -> Self {
        Self {
            count: Some(count),
            get: Some(get),
            select: Some(select),
        }
    }
}

unsafe extern "C" fn rescan(host: *const clap_host) {
    todo!()
}

pub trait ClapHostAudioPortsConfigInit {
    fn new() -> Self;
}

impl ClapHostAudioPortsConfigInit for clap_host_audio_ports_config {
    fn new() -> Self {
        Self {
            rescan: Some(rescan),
        }
    }
}
