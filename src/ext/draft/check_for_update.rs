#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::draft::check_for_update::*;
use clap_sys::{host::*, plugin::*};
use std::ffi::CStr;
use std::os::raw::c_char;

pub trait ClapCheckForUpdateInfoInit {
    fn new() -> Self;
}

impl ClapCheckForUpdateInfoInit for clap_check_for_update_info {
    fn new() -> Self {
        Self {
            version: todo!(),
            release_date: todo!(),
            url: todo!(),
            is_preview: todo!(),
        }
    }
}

unsafe extern "C" fn check(plugin: *const clap_plugin, include_preview: bool) {
    todo!()
}

pub trait ClapPluginCheckForUpdateInit {
    fn new() -> Self;
}

impl ClapPluginCheckForUpdateInit for clap_plugin_check_for_update {
    fn new() -> Self {
        Self { check: Some(check) }
    }
}

unsafe extern "C" fn on_new_version(
    host: *const clap_host,
    update_info: *const clap_check_for_update_info,
) {
    todo!()
}

pub trait ClapHostCheckForUpdateInit {
    fn new() -> Self;
}

impl ClapHostCheckForUpdateInit for clap_host_check_for_update {
    fn new() -> Self {
        Self {
            on_new_version: Some(on_new_version),
        }
    }
}
