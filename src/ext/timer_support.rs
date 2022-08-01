#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::timer_support::*;
use clap_sys::{host::*, id::*, plugin::*};
use std::ffi::CStr;

unsafe extern "C" fn on_timer(plugin: *const clap_plugin, timer_id: clap_id) {
    todo!()
}

pub trait ClapPluginTimerSupportInit {
    fn new() -> Self;
}

impl ClapPluginTimerSupportInit for clap_plugin_timer_support {
    fn new() -> Self {
        Self {
            on_timer: Some(on_timer),
        }
    }
}

unsafe extern "C" fn register_timer(
    host: *const clap_host,
    period_ms: u32,
    timer_id: *mut clap_id,
) -> bool {
    todo!()
}

unsafe extern "C" fn unregister_timer(host: *const clap_host, timer_id: clap_id) -> bool {
    todo!()
}

pub trait ClapHostTimerSupportInit {
    fn new() -> Self;
}

impl ClapHostTimerSupportInit for clap_host_timer_support {
    fn new() -> Self {
        Self {
            register_timer: Some(register_timer),
            unregister_timer: Some(unregister_timer),
        }
    }
}
