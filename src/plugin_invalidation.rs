#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::plugin_invalidation::*;
use std::ffi::CStr;
use std::os::raw::c_char;

pub trait ClapPluginInvalidationSourceInit {
    fn new() -> Self;
}

impl ClapPluginInvalidationSourceInit for clap_plugin_invalidation_source {
    fn new() -> Self {
        Self {
            directory: todo!(),
            filename_glob: todo!(),
            recursive_scan: todo!(),
        }
    }
}

unsafe extern "C" fn count(factory: *const clap_plugin_invalidation_factory) -> u32 {
    todo!()
}

unsafe extern "C" fn get(
    factory: *const clap_plugin_invalidation_factory,
    index: u32,
) -> *const clap_plugin_invalidation_source {
    todo!()
}

unsafe extern "C" fn refresh(factory: *const clap_plugin_invalidation_factory) -> bool {
    todo!()
}

pub trait ClapPluginInvalidationFactoryInit {
    fn new() -> Self;
}

impl ClapPluginInvalidationFactoryInit for clap_plugin_invalidation_factory {
    fn new() -> Self {
        Self {
            count: Some(count),
            get: Some(get),
            refresh: Some(refresh),
        }
    }
}
