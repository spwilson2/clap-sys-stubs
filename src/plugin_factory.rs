#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::plugin_factory::*;
use clap_sys::{host::*, plugin::*};
use std::ffi::CStr;
use std::os::raw::c_char;

unsafe extern "C" fn get_plugin_count(factory: *const clap_plugin_factory) -> u32 {
    todo!()
}

unsafe extern "C" fn get_plugin_descriptor(
    factory: *const clap_plugin_factory,
    index: u32,
) -> *const clap_plugin_descriptor {
    todo!()
}

unsafe extern "C" fn create_plugin(
    factory: *const clap_plugin_factory,
    host: *const clap_host,
    plugin_id: *const c_char,
) -> *const clap_plugin {
    todo!()
}

pub trait ClapPluginFactoryInit {
    fn new() -> Self;
}

impl ClapPluginFactoryInit for clap_plugin_factory {
    fn new() -> Self {
        Self {
            get_plugin_count: Some(get_plugin_count),
            get_plugin_descriptor: Some(get_plugin_descriptor),
            create_plugin: Some(create_plugin),
        }
    }
}
