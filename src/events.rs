#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::events::*;
use clap_sys::{fixedpoint::*, id::*};
use std::ffi::c_void;

pub trait ClapEventHeaderInit {
    fn new() -> Self;
}

impl ClapEventHeaderInit for clap_event_header {
    fn new() -> Self {
        Self {
            size: todo!(),
            time: todo!(),
            space_id: todo!(),
            type_: todo!(),
            flags: todo!(),
        }
    }
}

pub trait ClapEventNoteInit {
    fn new() -> Self;
}

impl ClapEventNoteInit for clap_event_note {
    fn new() -> Self {
        Self {
            header: todo!(),
            note_id: todo!(),
            port_index: todo!(),
            channel: todo!(),
            key: todo!(),
            velocity: todo!(),
        }
    }
}

pub trait ClapEventNoteExpressionInit {
    fn new() -> Self;
}

impl ClapEventNoteExpressionInit for clap_event_note_expression {
    fn new() -> Self {
        Self {
            header: todo!(),
            expression_id: todo!(),
            note_id: todo!(),
            port_index: todo!(),
            channel: todo!(),
            key: todo!(),
            value: todo!(),
        }
    }
}

pub trait ClapEventParamValueInit {
    fn new() -> Self;
}

impl ClapEventParamValueInit for clap_event_param_value {
    fn new() -> Self {
        Self {
            header: todo!(),
            param_id: todo!(),
            cookie: todo!(),
            note_id: todo!(),
            port_index: todo!(),
            channel: todo!(),
            key: todo!(),
            value: todo!(),
        }
    }
}

pub trait ClapEventParamModInit {
    fn new() -> Self;
}

impl ClapEventParamModInit for clap_event_param_mod {
    fn new() -> Self {
        Self {
            header: todo!(),
            param_id: todo!(),
            cookie: todo!(),
            note_id: todo!(),
            port_index: todo!(),
            channel: todo!(),
            key: todo!(),
            amount: todo!(),
        }
    }
}

pub trait ClapEventParamGestureInit {
    fn new() -> Self;
}

impl ClapEventParamGestureInit for clap_event_param_gesture {
    fn new() -> Self {
        Self {
            header: todo!(),
            param_id: todo!(),
        }
    }
}

pub trait ClapEventTransportInit {
    fn new() -> Self;
}

impl ClapEventTransportInit for clap_event_transport {
    fn new() -> Self {
        Self {
            header: todo!(),
            flags: todo!(),
            song_pos_beats: todo!(),
            song_pos_seconds: todo!(),
            tempo: todo!(),
            tempo_inc: todo!(),
            loop_start_beats: todo!(),
            loop_end_beats: todo!(),
            loop_start_seconds: todo!(),
            loop_end_seconds: todo!(),
            bar_start: todo!(),
            bar_number: todo!(),
            tsig_num: todo!(),
            tsig_denom: todo!(),
        }
    }
}

pub trait ClapEventMidiInit {
    fn new() -> Self;
}

impl ClapEventMidiInit for clap_event_midi {
    fn new() -> Self {
        Self {
            header: todo!(),
            port_index: todo!(),
            data: todo!(),
        }
    }
}

pub trait ClapEventMidiSysexInit {
    fn new() -> Self;
}

impl ClapEventMidiSysexInit for clap_event_midi_sysex {
    fn new() -> Self {
        Self {
            header: todo!(),
            port_index: todo!(),
            buffer: todo!(),
            size: todo!(),
        }
    }
}

pub trait ClapEventMidi2Init {
    fn new() -> Self;
}

impl ClapEventMidi2Init for clap_event_midi2 {
    fn new() -> Self {
        Self {
            header: todo!(),
            port_index: todo!(),
            data: todo!(),
        }
    }
}

unsafe extern "C" fn size(list: *const clap_input_events) -> u32 {
    todo!()
}

unsafe extern "C" fn get(list: *const clap_input_events, index: u32) -> *const clap_event_header {
    todo!()
}

pub trait ClapInputEventsInit {
    fn new() -> Self;
}

impl ClapInputEventsInit for clap_input_events {
    fn new() -> Self {
        Self {
            size: Some(size),
            get: Some(get),
            ctx: todo!(),
        }
    }
}

unsafe extern "C" fn try_push(
    list: *const clap_output_events,
    event: *const clap_event_header,
) -> bool {
    todo!()
}

pub trait ClapOutputEventsInit {
    fn new() -> Self;
}

impl ClapOutputEventsInit for clap_output_events {
    fn new() -> Self {
        Self {
            try_push: Some(try_push),
            ctx: todo!(),
        }
    }
}
