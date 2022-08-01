#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::note_ports::*;
use clap_sys::{host::*, id::*, plugin::*, string_sizes::*};
use std::ffi::CStr;
use std::os::raw::c_char;

pub trait ClapNotePortInfoInit {
    fn new() -> Self;
}

impl ClapNotePortInfoInit for clap_note_port_info {
    fn new() -> Self {
        Self {
            id: todo!(),
            supported_dialects: todo!(),
            preferred_dialect: todo!(),
            name: todo!(),
        }
    }
}

unsafe extern "C" fn count(plugin: *const clap_plugin, is_input: bool) -> u32 {
    todo!()
}

unsafe extern "C" fn get(
    plugin: *const clap_plugin,
    index: u32,
    is_input: bool,
    info: *mut clap_note_port_info,
) -> bool {
    todo!()
}

pub trait ClapPluginNotePortsInit {
    fn new() -> Self;
}

impl ClapPluginNotePortsInit for clap_plugin_note_ports {
    fn new() -> Self {
        Self {
            count: Some(count),
            get: Some(get),
        }
    }
}

unsafe extern "C" fn supported_dialects(host: *const clap_host) -> clap_note_dialect {
    todo!()
}

unsafe extern "C" fn rescan(host: *const clap_host, flags: u32) {
    todo!()
}

pub trait ClapHostNotePortsInit {
    fn new() -> Self;
}

impl ClapHostNotePortsInit for clap_host_note_ports {
    fn new() -> Self {
        Self {
            supported_dialects: Some(supported_dialects),
            rescan: Some(rescan),
        }
    }
}
