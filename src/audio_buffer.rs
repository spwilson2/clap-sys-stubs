#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::audio_buffer::*;

pub trait ClapAudioBufferInit {
    fn new() -> Self;
}

impl ClapAudioBufferInit for clap_audio_buffer {
    fn new() -> Self {
        Self {
            data32: todo!(),
            data64: todo!(),
            channel_count: todo!(),
            latency: todo!(),
            constant_mask: todo!(),
        }
    }
}
