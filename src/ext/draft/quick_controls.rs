#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::draft::quick_controls::*;
use clap_sys::{host::*, id::*, plugin::*, string_sizes::*};
use std::ffi::CStr;
use std::os::raw::c_char;

pub trait ClapQuickControlsPageInit {
    fn new() -> Self;
}

impl ClapQuickControlsPageInit for clap_quick_controls_page {
    fn new() -> Self {
        Self {
            id: todo!(),
            name: todo!(),
            param_ids: todo!(),
        }
    }
}

unsafe extern "C" fn count(plugin: *const clap_plugin) -> u32 {
    todo!()
}

unsafe extern "C" fn get(
    plugin: *const clap_plugin,
    page_index: u32,
    page: *mut clap_quick_controls_page,
) -> bool {
    todo!()
}

pub trait ClapPluginQuickControlsInit {
    fn new() -> Self;
}

impl ClapPluginQuickControlsInit for clap_plugin_quick_controls {
    fn new() -> Self {
        Self {
            count: Some(count),
            get: Some(get),
        }
    }
}

unsafe extern "C" fn changed(host: *const clap_host) {
    todo!()
}

unsafe extern "C" fn suggest_page(host: *const clap_host, page_id: clap_id) {
    todo!()
}

pub trait ClapHostQuickControlsInit {
    fn new() -> Self;
}

impl ClapHostQuickControlsInit for clap_host_quick_controls {
    fn new() -> Self {
        Self {
            changed: Some(changed),
            suggest_page: Some(suggest_page),
        }
    }
}
