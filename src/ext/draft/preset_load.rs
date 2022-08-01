#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::draft::preset_load::*;
use clap_sys::plugin::*;
use std::ffi::CStr;
use std::os::raw::c_char;

unsafe extern "C" fn from_file(plugin: *const clap_plugin, path: *const c_char) -> bool {
    todo!()
}

pub trait ClapPluginPresetLoadInit {
    fn new() -> Self;
}

impl ClapPluginPresetLoadInit for clap_plugin_preset_load {
    fn new() -> Self {
        Self {
            from_file: Some(from_file),
        }
    }
}
