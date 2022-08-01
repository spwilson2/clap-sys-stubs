#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::host::*;
use clap_sys::version::*;
use std::ffi::c_void;
use std::os::raw::c_char;

unsafe extern "C" fn get_extension(
    host: *const clap_host,
    extension_id: *const c_char,
) -> *const c_void {
    todo!()
}

unsafe extern "C" fn request_restart(host: *const clap_host) {
    todo!()
}

unsafe extern "C" fn request_process(host: *const clap_host) {
    todo!()
}

unsafe extern "C" fn request_callback(host: *const clap_host) {
    todo!()
}

pub trait ClapHostInit {
    fn new() -> Self;
}

impl ClapHostInit for clap_host {
    fn new() -> Self {
        Self {
            get_extension: Some(get_extension),
            request_restart: Some(request_restart),
            request_process: Some(request_process),
            request_callback: Some(request_callback),
            clap_version: todo!(),
            host_data: todo!(),
            name: todo!(),
            vendor: todo!(),
            url: todo!(),
            version: todo!(),
        }
    }
}
