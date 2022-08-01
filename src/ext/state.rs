#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::state::*;
use clap_sys::{host::*, plugin::*, stream::*};
use std::ffi::CStr;

unsafe extern "C" fn save(plugin: *const clap_plugin, stream: *const clap_ostream) -> bool {
    todo!()
}

unsafe extern "C" fn load(plugin: *const clap_plugin, stream: *const clap_istream) -> bool {
    todo!()
}

pub trait ClapPluginStateInit {
    fn new() -> Self;
}

impl ClapPluginStateInit for clap_plugin_state {
    fn new() -> Self {
        Self {
            save: Some(save),
            load: Some(load),
        }
    }
}

unsafe extern "C" fn mark_dirty(host: *const clap_host) {
    todo!()
}

pub trait ClapHostStateInit {
    fn new() -> Self;
}

impl ClapHostStateInit for clap_host_state {
    fn new() -> Self {
        Self {
            mark_dirty: Some(mark_dirty),
        }
    }
}
