#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]
use clap_sys::ext::gui::*;
use clap_sys::{host::*, plugin::*};
use std::ffi::{c_void, CStr};
use std::fmt::Debug;
use std::os::raw::{c_char, c_ulong};

pub trait ClapWindowInit {
    fn new() -> Self;
}

impl ClapWindowInit for clap_window {
    fn new() -> Self {
        Self {
            api: todo!(),
            specific: todo!(),
        }
    }
}

pub trait ClapGuiResizeHintsInit {
    fn new() -> Self;
}

impl ClapGuiResizeHintsInit for clap_gui_resize_hints {
    fn new() -> Self {
        Self {
            can_resize_horizontally: todo!(),
            can_resize_vertically: todo!(),
            preserve_aspect_ratio: todo!(),
            aspect_ratio_width: todo!(),
            aspect_ratio_height: todo!(),
        }
    }
}

unsafe extern "C" fn is_api_supported(
    plugin: *const clap_plugin,
    api: *const c_char,
    is_floating: bool,
) -> bool {
    todo!()
}

unsafe extern "C" fn get_preferred_api(
    plugin: *const clap_plugin,
    api: *mut *const c_char,
    is_floating: *mut bool,
) -> bool {
    todo!()
}

unsafe extern "C" fn create(
    plugin: *const clap_plugin,
    api: *const c_char,
    is_floating: bool,
) -> bool {
    todo!()
}

unsafe extern "C" fn destroy(plugin: *const clap_plugin) {
    todo!()
}

unsafe extern "C" fn set_scale(plugin: *const clap_plugin, scale: f64) -> bool {
    todo!()
}

unsafe extern "C" fn get_size(
    plugin: *const clap_plugin,
    width: *mut u32,
    height: *mut u32,
) -> bool {
    todo!()
}

unsafe extern "C" fn can_resize(plugin: *const clap_plugin) -> bool {
    todo!()
}

unsafe extern "C" fn get_resize_hints(
    plugin: *const clap_plugin,
    hints: *mut clap_gui_resize_hints,
) -> bool {
    todo!()
}

unsafe extern "C" fn adjust_size(
    plugin: *const clap_plugin,
    width: *mut u32,
    height: *mut u32,
) -> bool {
    todo!()
}

unsafe extern "C" fn set_size(plugin: *const clap_plugin, width: u32, height: u32) -> bool {
    todo!()
}

unsafe extern "C" fn set_parent(plugin: *const clap_plugin, window: *const clap_window) -> bool {
    todo!()
}

unsafe extern "C" fn set_transient(plugin: *const clap_plugin, window: *const clap_window) -> bool {
    todo!()
}

unsafe extern "C" fn suggest_title(plugin: *const clap_plugin, title: *const c_char) {
    todo!()
}

unsafe extern "C" fn show(plugin: *const clap_plugin) -> bool {
    todo!()
}

unsafe extern "C" fn hide(plugin: *const clap_plugin) -> bool {
    todo!()
}

pub trait ClapPluginGuiInit {
    fn new() -> Self;
}

impl ClapPluginGuiInit for clap_plugin_gui {
    fn new() -> Self {
        Self {
            is_api_supported: Some(is_api_supported),
            get_preferred_api: Some(get_preferred_api),
            create: Some(create),
            destroy: Some(destroy),
            set_scale: Some(set_scale),
            get_size: Some(get_size),
            can_resize: Some(can_resize),
            get_resize_hints: Some(get_resize_hints),
            adjust_size: Some(adjust_size),
            set_size: Some(set_size),
            set_parent: Some(set_parent),
            set_transient: Some(set_transient),
            suggest_title: Some(suggest_title),
            show: Some(show),
            hide: Some(hide),
        }
    }
}

unsafe extern "C" fn resize_hints_changed(host: *const clap_host) {
    todo!()
}

unsafe extern "C" fn request_resize(host: *const clap_host, width: u32, height: u32) -> bool {
    todo!()
}

unsafe extern "C" fn request_show(host: *const clap_host) -> bool {
    todo!()
}

unsafe extern "C" fn request_hide(host: *const clap_host) -> bool {
    todo!()
}

unsafe extern "C" fn closed(host: *const clap_host, was_destroyed: bool) {
    todo!()
}

pub trait ClapHostGuiInit {
    fn new() -> Self;
}

impl ClapHostGuiInit for clap_host_gui {
    fn new() -> Self {
        Self {
            resize_hints_changed: Some(resize_hints_changed),
            request_resize: Some(request_resize),
            request_show: Some(request_show),
            request_hide: Some(request_hide),
            closed: Some(closed),
        }
    }
}
