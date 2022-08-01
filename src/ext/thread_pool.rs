#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::thread_pool::*;
use clap_sys::{host::*, plugin::*};
use std::ffi::CStr;

unsafe extern "C" fn exec(plugin: *const clap_plugin, task_index: u32) {
    todo!()
}

pub trait ClapPluginThreadPoolInit {
    fn new() -> Self;
}

impl ClapPluginThreadPoolInit for clap_plugin_thread_pool {
    fn new() -> Self {
        Self { exec: Some(exec) }
    }
}

unsafe extern "C" fn request_exec(host: *const clap_host, num_tasks: u32) -> bool {
    todo!()
}

pub trait ClapHostThreadPoolInit {
    fn new() -> Self;
}

impl ClapHostThreadPoolInit for clap_host_thread_pool {
    fn new() -> Self {
        Self {
            request_exec: Some(request_exec),
        }
    }
}
