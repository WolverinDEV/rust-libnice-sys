mod fixup;
/* Sadly most IDEs can not resolve this import... */
/*
use fixup::*;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
 */

mod bindings;
pub use bindings::*;