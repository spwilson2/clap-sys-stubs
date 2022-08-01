#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::draft::voice_info::*;
use clap_sys::{host::*, plugin::*};
use std::ffi::CStr;

pub trait ClapVoiceInfoInit {
    fn new() -> Self;
}

impl ClapVoiceInfoInit for clap_voice_info {
    fn new() -> Self {
        Self {
            voice_count: todo!(),
            voice_capacity: todo!(),
            flags: todo!(),
        }
    }
}

unsafe extern "C" fn get(plugin: *const clap_plugin, info: *mut clap_voice_info) -> bool {
    todo!()
}

pub trait ClapPluginVoiceInfoInit {
    fn new() -> Self;
}

impl ClapPluginVoiceInfoInit for clap_plugin_voice_info {
    fn new() -> Self {
        Self { get: Some(get) }
    }
}

unsafe extern "C" fn changed(host: *const clap_host) {
    todo!()
}

pub trait ClapHostVoiceInfoInit {
    fn new() -> Self;
}

impl ClapHostVoiceInfoInit for clap_host_voice_info {
    fn new() -> Self {
        Self {
            changed: Some(changed),
        }
    }
}
