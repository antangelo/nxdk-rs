// SPDX-License-Identifier: MIT

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

pub mod stdlib {
    include!(concat!(env!("OUT_DIR"), "/bindings_stdlib.rs"));
}

pub mod stdio {
    include!(concat!(env!("OUT_DIR"), "/bindings_stdio.rs"));
}

pub mod time {
    include!(concat!(env!("OUT_DIR"), "/bindings_time.rs"));
}

pub mod string {
    include!(concat!(env!("OUT_DIR"), "/bindings_string.rs"));
}
