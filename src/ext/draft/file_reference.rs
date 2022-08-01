#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::draft::file_reference::*;
use clap_sys::{host::*, id::*, plugin::*};
use std::ffi::CStr;
use std::os::raw::c_char;

pub trait ClapFileReferenceInit {
    fn new() -> Self;
}

impl ClapFileReferenceInit for clap_file_reference {
    fn new() -> Self {
        Self {
            resource_id: todo!(),
            belongs_to_plugin_collection: todo!(),
            path_capacity: todo!(),
            path_size: todo!(),
            path: todo!(),
        }
    }
}

unsafe extern "C" fn count(plugin: *const clap_plugin) -> u32 {
    todo!()
}

unsafe extern "C" fn get(
    plugin: *const clap_plugin,
    index: u32,
    file_reference: *mut clap_file_reference,
) -> bool {
    todo!()
}

unsafe extern "C" fn get_blake3_digest(
    plugin: *const clap_plugin,
    resource_id: clap_id,
    digest: *mut u8,
) -> bool {
    todo!()
}

unsafe extern "C" fn get_file_size(
    plugin: *const clap_plugin,
    resource_id: clap_id,
    size: *mut u64,
) -> bool {
    todo!()
}

unsafe extern "C" fn update_path(
    plugin: *const clap_plugin,
    resource_id: clap_id,
    path: *const c_char,
) -> bool {
    todo!()
}

unsafe extern "C" fn save_resources(plugin: *const clap_plugin) -> bool {
    todo!()
}

pub trait ClapPluginFileReferenceInit {
    fn new() -> Self;
}

impl ClapPluginFileReferenceInit for clap_plugin_file_reference {
    fn new() -> Self {
        Self {
            count: Some(count),
            get: Some(get),
            get_blake3_digest: Some(get_blake3_digest),
            get_file_size: Some(get_file_size),
            update_path: Some(update_path),
            save_resources: Some(save_resources),
        }
    }
}

unsafe extern "C" fn changed(host: *const clap_host) {
    todo!()
}

unsafe extern "C" fn set_dirty(host: *const clap_host, resource_id: clap_id) {
    todo!()
}

pub trait ClapHostFileReferenceInit {
    fn new() -> Self;
}

impl ClapHostFileReferenceInit for clap_host_file_reference {
    fn new() -> Self {
        Self {
            changed: Some(changed),
            set_dirty: Some(set_dirty),
        }
    }
}
