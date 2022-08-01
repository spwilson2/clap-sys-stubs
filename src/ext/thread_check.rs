#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::thread_check::*;
use clap_sys::host::*;
use std::ffi::CStr;

unsafe extern "C" fn is_main_thread(host: *const clap_host) -> bool {
    todo!()
}

unsafe extern "C" fn is_audio_thread(host: *const clap_host) -> bool {
    todo!()
}

pub trait ClapHostThreadCheckInit {
    fn new() -> Self;
}

impl ClapHostThreadCheckInit for clap_host_thread_check {
    fn new() -> Self {
        Self {
            is_main_thread: Some(is_main_thread),
            is_audio_thread: Some(is_audio_thread),
        }
    }
}
