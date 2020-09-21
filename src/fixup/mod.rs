#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
#![allow(dead_code)]

pub use gio_sys::*;
pub use gobject_sys::*;
pub use glib_sys::*;

pub use libc::{
    size_t, c_int, c_uint, c_char, c_uchar,
    sockaddr,
    timeval
};

pub type gsize = usize;
pub type gssize = isize;
pub type guint64 = u64;
pub type guint32 = u32;
pub type guint16 = u16;
pub type guint8 = u8;
pub type guint = c_uint;
pub type gint = c_int;
pub type guchar = c_uchar;
pub type gchar = c_char;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;