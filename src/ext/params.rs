#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::params::*;
use clap_sys::{events::*, host::*, id::*, plugin::*, string_sizes::*};
use std::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;

pub trait ClapParamInfoInit {
    fn new() -> Self;
}

impl ClapParamInfoInit for clap_param_info {
    fn new() -> Self {
        Self {
            id: todo!(),
            flags: todo!(),
            cookie: todo!(),
            name: todo!(),
            module: todo!(),
            min_value: todo!(),
            max_value: todo!(),
            default_value: todo!(),
        }
    }
}

unsafe extern "C" fn count(plugin: *const clap_plugin) -> u32 {
    todo!()
}

unsafe extern "C" fn get_info(
    plugin: *const clap_plugin,
    param_index: u32,
    param_info: *mut clap_param_info,
) -> bool {
    todo!()
}

unsafe extern "C" fn get_value(
    plugin: *const clap_plugin,
    param_id: clap_id,
    value: *mut f64,
) -> bool {
    todo!()
}

unsafe extern "C" fn value_to_text(
    plugin: *const clap_plugin,
    param_id: clap_id,
    value: f64,
    display: *mut c_char,
    size: u32,
) -> bool {
    todo!()
}

unsafe extern "C" fn text_to_value(
    plugin: *const clap_plugin,
    param_id: clap_id,
    display: *const c_char,
    value: *mut f64,
) -> bool {
    todo!()
}

unsafe extern "C" fn flush(
    plugin: *const clap_plugin,
    in_: *const clap_input_events,
    out: *const clap_output_events,
) {
    todo!()
}

pub trait ClapPluginParamsInit {
    fn new() -> Self;
}

impl ClapPluginParamsInit for clap_plugin_params {
    fn new() -> Self {
        Self {
            count: Some(count),
            get_info: Some(get_info),
            get_value: Some(get_value),
            value_to_text: Some(value_to_text),
            text_to_value: Some(text_to_value),
            flush: Some(flush),
        }
    }
}

unsafe extern "C" fn rescan(host: *const clap_host, flags: clap_param_rescan_flags) {
    todo!()
}

unsafe extern "C" fn clear(
    host: *const clap_host,
    param_id: clap_id,
    flags: clap_param_clear_flags,
) {
    todo!()
}

unsafe extern "C" fn request_flush(host: *const clap_host) {
    todo!()
}

pub trait ClapHostParamsInit {
    fn new() -> Self;
}

impl ClapHostParamsInit for clap_host_params {
    fn new() -> Self {
        Self {
            rescan: Some(rescan),
            clear: Some(clear),
            request_flush: Some(request_flush),
        }
    }
}
