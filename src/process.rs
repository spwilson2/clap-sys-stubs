#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::audio_buffer::*;
use clap_sys::events::*;
use clap_sys::process::*;

pub trait ClapProcessInit {
    fn new() -> Self;
}

impl ClapProcessInit for clap_process {
    fn new() -> Self {
        Self {
            steady_time: todo!(),
            frames_count: todo!(),
            transport: todo!(),
            audio_inputs: todo!(),
            audio_outputs: todo!(),
            audio_inputs_count: todo!(),
            audio_outputs_count: todo!(),
            in_events: todo!(),
            out_events: todo!(),
        }
    }
}
