#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::log::*;
use clap_sys::host::*;
use std::ffi::CStr;
use std::os::raw::c_char;

unsafe extern "C" fn log(host: *const clap_host, severity: clap_log_severity, msg: *const c_char) {
    todo!()
}

pub trait ClapHostLogInit {
    fn new() -> Self;
}

impl ClapHostLogInit for clap_host_log {
    fn new() -> Self {
        Self { log: Some(log) }
    }
}
