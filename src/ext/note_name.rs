#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::note_name::*;
use clap_sys::{host::*, plugin::*, string_sizes::*};
use std::ffi::CStr;
use std::os::raw::c_char;

pub trait ClapNoteNameInit {
    fn new() -> Self;
}

impl ClapNoteNameInit for clap_note_name {
    fn new() -> Self {
        Self {
            name: todo!(),
            port: todo!(),
            key: todo!(),
            channel: todo!(),
        }
    }
}

unsafe extern "C" fn count(plugin: *const clap_plugin) -> u32 {
    todo!()
}

unsafe extern "C" fn get(
    plugin: *const clap_plugin,
    index: u32,
    note_name: *mut clap_note_name,
) -> bool {
    todo!()
}

pub trait ClapPluginNoteNameInit {
    fn new() -> Self;
}

impl ClapPluginNoteNameInit for clap_plugin_note_name {
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

pub trait ClapHostNoteNameInit {
    fn new() -> Self;
}

impl ClapHostNoteNameInit for clap_host_note_name {
    fn new() -> Self {
        Self {
            changed: Some(changed),
        }
    }
}
