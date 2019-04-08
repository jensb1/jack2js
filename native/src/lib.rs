#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


#[macro_use]
extern crate neon;


mod cjack2;
mod jack2;

register_module!(mut m, {
    m.export_class::<jack2::JsJack2>("Jack2")
});
