#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::draft::ambisonic::*;
use clap_sys::{host::*, plugin::*};
use std::ffi::CStr;

pub trait ClapAmbisonicInfoInit {
    fn new() -> Self;
}

impl ClapAmbisonicInfoInit for clap_ambisonic_info {
    fn new() -> Self {
        Self {
            ordering: todo!(),
            normalization: todo!(),
        }
    }
}

unsafe extern "C" fn get_info(
    plugin: *const clap_plugin,
    is_input: bool,
    port_index: u32,
    info: *mut clap_ambisonic_info,
) -> bool {
    todo!()
}

pub trait ClapPluginAmbisonicInit {
    fn new() -> Self;
}

impl ClapPluginAmbisonicInit for clap_plugin_ambisonic {
    fn new() -> Self {
        Self {
            get_info: Some(get_info),
        }
    }
}

unsafe extern "C" fn changed(host: *const clap_host) {
    todo!()
}

pub trait ClapHostAmbisonicInit {
    fn new() -> Self;
}

impl ClapHostAmbisonicInit for clap_host_ambisonic {
    fn new() -> Self {
        Self {
            changed: Some(changed),
        }
    }
}
