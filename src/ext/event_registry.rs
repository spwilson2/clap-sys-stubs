#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::event_registry::*;
use clap_sys::host::*;
use std::ffi::CStr;
use std::os::raw::c_char;

unsafe extern "C" fn query(
    host: *const clap_host,
    space_name: *const c_char,
    space_id: *mut u16,
) -> bool {
    todo!()
}

pub trait ClapHostEventRegistryInit {
    fn new() -> Self;
}

impl ClapHostEventRegistryInit for clap_host_event_registry {
    fn new() -> Self {
        Self { query: Some(query) }
    }
}
