#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use gio_sys::*;
use gobject_sys::*;
use glib_sys::*;
use libc::*;

type gsize = usize;
type gssize = isize;
type guint64 = u64;
type guint32 = u32;
type guint16 = u16;
type guint8 = u8;
type guint = c_uint;
type gint = c_int;
type guchar = c_uchar;
type gchar = c_char;

/* for windows only */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_in {
    __data: [u8; 4]
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_in6 {
    __data: [u8; 4]
}

#[repr(C)]
pub union sockaddr_storage {
    v4: sockaddr_in6,
    v6: sockaddr_in6
}
type socklen_t = u32;
type SSIZE_T = i64;
/* end for windows only */

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
