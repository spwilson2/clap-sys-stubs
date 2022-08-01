#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::render::*;
use clap_sys::plugin::*;
use std::ffi::CStr;

unsafe extern "C" fn has_hard_realtime_requirement(plugin: *const clap_plugin) -> bool {
    todo!()
}

unsafe extern "C" fn set(plugin: *const clap_plugin, mode: clap_plugin_render_mode) -> bool {
    todo!()
}

pub trait ClapPluginRenderInit {
    fn new() -> Self;
}

impl ClapPluginRenderInit for clap_plugin_render {
    fn new() -> Self {
        Self {
            has_hard_realtime_requirement: Some(has_hard_realtime_requirement),
            set: Some(set),
        }
    }
}
