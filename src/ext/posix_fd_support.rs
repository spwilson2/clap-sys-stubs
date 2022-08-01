#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::posix_fd_support::*;
use clap_sys::{host::*, plugin::*};
use std::ffi::CStr;

unsafe extern "C" fn on_fd(plugin: *const clap_plugin, fd: i32, flags: clap_posix_fd_flags) {
    todo!()
}

pub trait ClapPluginPosixFdSupportInit {
    fn new() -> Self;
}

impl ClapPluginPosixFdSupportInit for clap_plugin_posix_fd_support {
    fn new() -> Self {
        Self { on_fd: Some(on_fd) }
    }
}

unsafe extern "C" fn register_fd(
    host: *const clap_host,
    fd: i32,
    flags: clap_posix_fd_flags,
) -> bool {
    todo!()
}

unsafe extern "C" fn modify_fd(
    host: *const clap_host,
    fd: i32,
    flags: clap_posix_fd_flags,
) -> bool {
    todo!()
}

unsafe extern "C" fn unregister_fd(host: *const clap_host, fd: i32) -> bool {
    todo!()
}

pub trait ClapHostPosixFdSupportInit {
    fn new() -> Self;
}

impl ClapHostPosixFdSupportInit for clap_host_posix_fd_support {
    fn new() -> Self {
        Self {
            register_fd: Some(register_fd),
            modify_fd: Some(modify_fd),
            unregister_fd: Some(unregister_fd),
        }
    }
}
