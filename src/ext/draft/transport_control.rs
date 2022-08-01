#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::draft::transport_control::*;
use clap_sys::{fixedpoint::*, host::*};
use std::ffi::CStr;

unsafe extern "C" fn request_start(host: *const clap_host) {
    todo!()
}

unsafe extern "C" fn request_stop(host: *const clap_host) {
    todo!()
}

unsafe extern "C" fn request_continue(host: *const clap_host) {
    todo!()
}

unsafe extern "C" fn request_pause(host: *const clap_host) {
    todo!()
}

unsafe extern "C" fn request_toggle_play(host: *const clap_host) {
    todo!()
}

unsafe extern "C" fn request_jump(host: *const clap_host, position: clap_beattime) {
    todo!()
}

unsafe extern "C" fn request_loop_region(
    host: *const clap_host,
    start: clap_beattime,
    duration: clap_beattime,
) {
    todo!()
}

unsafe extern "C" fn request_toggle_loop(host: *const clap_host) {
    todo!()
}

unsafe extern "C" fn request_enable_loop(host: *const clap_host, is_enabled: bool) {
    todo!()
}

unsafe extern "C" fn request_record(host: *const clap_host, is_recording: bool) {
    todo!()
}

unsafe extern "C" fn request_toggle_record(host: *const clap_host) {
    todo!()
}

pub trait ClapHostTransportControlInit {
    fn new() -> Self;
}

impl ClapHostTransportControlInit for clap_host_transport_control {
    fn new() -> Self {
        Self {
            request_start: Some(request_start),
            request_stop: Some(request_stop),
            request_continue: Some(request_continue),
            request_pause: Some(request_pause),
            request_toggle_play: Some(request_toggle_play),
            request_jump: Some(request_jump),
            request_loop_region: Some(request_loop_region),
            request_toggle_loop: Some(request_toggle_loop),
            request_enable_loop: Some(request_enable_loop),
            request_record: Some(request_record),
            request_toggle_record: Some(request_toggle_record),
        }
    }
}
