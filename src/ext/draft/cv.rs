#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::draft::cv::*;
use clap_sys::{host::*, plugin::*};
use std::ffi::CStr;

unsafe extern "C" fn get_channel_type(
    plugin: *const clap_plugin,
    is_input: bool,
    port_index: u32,
    channel_index: u32,
    channel_type: *mut u32,
) -> bool {
    todo!()
}

pub trait ClapPluginCvInit {
    fn new() -> Self;
}

impl ClapPluginCvInit for clap_plugin_cv {
    fn new() -> Self {
        Self {
            get_channel_type: Some(get_channel_type),
        }
    }
}

unsafe extern "C" fn changed(host: *const clap_host) {
    todo!()
}

pub trait ClapHostCvInit {
    fn new() -> Self;
}

impl ClapHostCvInit for clap_host_cv {
    fn new() -> Self {
        Self {
            changed: Some(changed),
        }
    }
}
