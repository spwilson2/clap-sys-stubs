#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::draft::track_info::*;
use clap_sys::{color::*, host::*, id::*, plugin::*, string_sizes::*};
use std::ffi::CStr;
use std::os::raw::c_char;

pub trait ClapTrackInfoInit {
    fn new() -> Self;
}

impl ClapTrackInfoInit for clap_track_info {
    fn new() -> Self {
        Self {
            id: todo!(),
            index: todo!(),
            name: todo!(),
            path: todo!(),
            channel_count: todo!(),
            audio_port_type: todo!(),
            color: todo!(),
            is_return_track: todo!(),
        }
    }
}

unsafe extern "C" fn changed(plugin: *const clap_plugin) {
    todo!()
}

pub trait ClapPluginTrackInfoInit {
    fn new() -> Self;
}

impl ClapPluginTrackInfoInit for clap_plugin_track_info {
    fn new() -> Self {
        Self {
            changed: Some(changed),
        }
    }
}

unsafe extern "C" fn get(host: *const clap_host, info: *mut clap_track_info) -> bool {
    todo!()
}

pub trait ClapHostTrackInfoInit {
    fn new() -> Self;
}

impl ClapHostTrackInfoInit for clap_host_track_info {
    fn new() -> Self {
        Self { get: Some(get) }
    }
}
