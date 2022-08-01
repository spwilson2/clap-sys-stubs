#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::entry::*;
use clap_sys::version::*;
use std::ffi::c_void;
use std::os::raw::c_char;

unsafe extern "C" fn init(plugin_path: *const c_char) -> bool {
    todo!()
}

unsafe extern "C" fn deinit() {
    todo!()
}

unsafe extern "C" fn get_factory(factory_id: *const c_char) -> *const c_void {
    todo!()
}

pub trait ClapPluginEntryInit {
    fn new() -> Self;
}

impl ClapPluginEntryInit for clap_plugin_entry {
    fn new() -> Self {
        Self {
            init: Some(init),
            deinit: Some(deinit),
            get_factory: Some(get_factory),
            clap_version: todo!(),
        }
    }
}
