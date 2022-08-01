#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::plugin::*;
use clap_sys::{process::*, version::*};
use std::ffi::c_void;
use std::os::raw::c_char;

pub trait ClapPluginDescriptorInit {
    fn new() -> Self;
}

impl ClapPluginDescriptorInit for clap_plugin_descriptor {
    fn new() -> Self {
        Self {
            clap_version: todo!(),
            id: todo!(),
            name: todo!(),
            vendor: todo!(),
            url: todo!(),
            manual_url: todo!(),
            support_url: todo!(),
            version: todo!(),
            description: todo!(),
            features: todo!(),
        }
    }
}

unsafe extern "C" fn init(plugin: *const clap_plugin) -> bool {
    todo!()
}

unsafe extern "C" fn destroy(plugin: *const clap_plugin) {
    todo!()
}

unsafe extern "C" fn activate(
    plugin: *const clap_plugin,
    sample_rate: f64,
    min_frames_count: u32,
    max_frames_count: u32,
) -> bool {
    todo!()
}

unsafe extern "C" fn deactivate(plugin: *const clap_plugin) {
    todo!()
}

unsafe extern "C" fn start_processing(plugin: *const clap_plugin) -> bool {
    todo!()
}

unsafe extern "C" fn stop_processing(plugin: *const clap_plugin) {
    todo!()
}

unsafe extern "C" fn reset(plugin: *const clap_plugin) {
    todo!()
}

unsafe extern "C" fn process(
    plugin: *const clap_plugin,
    process: *const clap_process,
) -> clap_process_status {
    todo!()
}

unsafe extern "C" fn get_extension(plugin: *const clap_plugin, id: *const c_char) -> *const c_void {
    todo!()
}

unsafe extern "C" fn on_main_thread(plugin: *const clap_plugin) {
    todo!()
}

pub trait ClapPluginInit {
    fn new() -> Self;
}

impl ClapPluginInit for clap_plugin {
    fn new() -> Self {
        Self {
            init: Some(init),
            destroy: Some(destroy),
            activate: Some(activate),
            deactivate: Some(deactivate),
            start_processing: Some(start_processing),
            stop_processing: Some(stop_processing),
            reset: Some(reset),
            process: Some(process),
            get_extension: Some(get_extension),
            on_main_thread: Some(on_main_thread),
            desc: todo!(),
            plugin_data: todo!(),
        }
    }
}
