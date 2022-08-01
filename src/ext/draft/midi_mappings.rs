#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::draft::midi_mappings::*;
use clap_sys::{host::*, id::*, plugin::*};
use std::ffi::CStr;

pub trait ClapMidiMappingInit {
    fn new() -> Self;
}

impl ClapMidiMappingInit for clap_midi_mapping {
    fn new() -> Self {
        Self {
            channel: todo!(),
            number: todo!(),
            param_id: todo!(),
        }
    }
}

unsafe extern "C" fn count(plugin: *const clap_plugin) -> u32 {
    todo!()
}

unsafe extern "C" fn get(
    plugin: *const clap_plugin,
    index: u32,
    mapping: *mut clap_midi_mapping,
) -> bool {
    todo!()
}

pub trait ClapPluginMidiMappingsInit {
    fn new() -> Self;
}

impl ClapPluginMidiMappingsInit for clap_plugin_midi_mappings {
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

pub trait ClapHostMidiMappingsInit {
    fn new() -> Self;
}

impl ClapHostMidiMappingsInit for clap_host_midi_mappings {
    fn new() -> Self {
        Self {
            changed: Some(changed),
        }
    }
}
