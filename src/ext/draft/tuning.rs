#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::draft::tuning::*;
use clap_sys::{events::*, host::*, id::*, plugin::*, string_sizes::*};
use std::ffi::CStr;
use std::os::raw::c_char;

pub trait ClapEventTuningInit {
    fn new() -> Self;
}

impl ClapEventTuningInit for clap_event_tuning {
    fn new() -> Self {
        Self {
            header: todo!(),
            port_index: todo!(),
            channel: todo!(),
            tunning_id: todo!(),
        }
    }
}

pub trait ClapTuningInfoInit {
    fn new() -> Self;
}

impl ClapTuningInfoInit for clap_tuning_info {
    fn new() -> Self {
        Self {
            tuning_id: todo!(),
            name: todo!(),
            is_dynamic: todo!(),
        }
    }
}

unsafe extern "C" fn changed(plugin: *const clap_plugin) {
    todo!()
}

pub trait ClapPluginTuningTInit {
    fn new() -> Self;
}

impl ClapPluginTuningTInit for clap_plugin_tuning_t {
    fn new() -> Self {
        Self {
            changed: Some(changed),
        }
    }
}

unsafe extern "C" fn get_relative(
    host: *const clap_host,
    tuning_id: clap_id,
    channel: i32,
    key: i32,
    sample_offset: u32,
) -> f64 {
    todo!()
}

unsafe extern "C" fn should_play(
    host: *const clap_host,
    tuning_id: clap_id,
    channel: i32,
    key: i32,
) -> bool {
    todo!()
}

unsafe extern "C" fn get_tuning_count(host: *const clap_host) -> u32 {
    todo!()
}

unsafe extern "C" fn get_info(
    host: *const clap_host,
    tuning_index: u32,
    info: *mut clap_tuning_info,
) -> bool {
    todo!()
}

pub trait ClapHostTuningInit {
    fn new() -> Self;
}

impl ClapHostTuningInit for clap_host_tuning {
    fn new() -> Self {
        Self {
            get_relative: Some(get_relative),
            should_play: Some(should_play),
            get_tuning_count: Some(get_tuning_count),
            get_info: Some(get_info),
        }
    }
}
