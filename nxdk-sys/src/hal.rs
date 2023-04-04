// SPDX-License-Identifier: MIT

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

pub mod video {
    include!(concat!(env!("OUT_DIR"), "/bindings_video.rs"));
}

pub mod debug {
    include!(concat!(env!("OUT_DIR"), "/bindings_debug.rs"));
}
