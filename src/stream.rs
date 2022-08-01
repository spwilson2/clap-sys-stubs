#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::stream::*;
use std::ffi::c_void;

unsafe extern "C" fn read(stream: *const clap_istream, buffer: *mut c_void, size: u64) -> i64 {
    todo!()
}

pub trait ClapIstreamInit {
    fn new() -> Self;
}

impl ClapIstreamInit for clap_istream {
    fn new() -> Self {
        Self {
            read: Some(read),
            ctx: todo!(),
        }
    }
}

unsafe extern "C" fn write(stream: *const clap_ostream, buffer: *const c_void, size: u64) -> i64 {
    todo!()
}

pub trait ClapOstreamInit {
    fn new() -> Self;
}

impl ClapOstreamInit for clap_ostream {
    fn new() -> Self {
        Self {
            write: Some(write),
            ctx: todo!(),
        }
    }
}
