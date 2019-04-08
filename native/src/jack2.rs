#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


//#[path = "cjack2.rs"] mod cjack2;
#[link(name = "jack")]
extern {
	pub fn jack_client_open(
		client_name: *const ::std::os::raw::c_char,
		options: jack_options_t,
		status: *mut jack_status_t,
		...
		) -> *mut jack_client_t;
	pub fn jack_activate(client: *mut jack_client_t) -> ::std::os::raw::c_int;
	pub fn jack_get_ports(
		client: *mut jack_client_t,
		port_name_pattern: *const ::std::os::raw::c_char,
		type_name_pattern: *const ::std::os::raw::c_char,
		flags: ::std::os::raw::c_ulong,
		) -> *mut *const ::std::os::raw::c_char;

    pub fn jack_connect(
        client: *mut jack_client_t,
        source_port: *const ::std::os::raw::c_char,
        destination_port: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

	pub fn jack_free(
		ptr: *mut std::ffi::c_void
		);
}



use neon::prelude::*;
use std::ptr;
use std::ffi::{CString, CStr};

use cjack2::{jack_client_t, jack_options_t, jack_status_t,JackOptions_JackNullOption};
use neon::prelude::*;

pub struct Jack2 {
	client: *mut jack_client_t,
}

type Unit = ();

declare_types! {
	pub class JsJack2 for Jack2 {
		init(mut cx) {

			let options: jack_options_t = JackOptions_JackNullOption;
			let mut status: jack_status_t = 0;

				
			let client = unsafe {jack_client_open(CString::new("testclient").unwrap().as_ptr(), options, &mut status)};
			unsafe {jack_activate(client)};

			Ok(Jack2 {
				client: client
			})
		}

		method get_ports(mut cx) {

			let mut ports = Vec::new();
			let this = cx.this();
		
			unsafe {
				let guard = cx.lock();
	            let jack2 = this.borrow(&guard);

				let cports = jack_get_ports(jack2.client, 
					CString::new("").unwrap().as_ptr(), 
					CString::new("").unwrap().as_ptr(), 
					0);

				let mut i = 0;

				while *cports.offset(i) != ptr::null() {
					let port = CStr::from_ptr(*(cports.offset(i)));
					ports.push(port.to_string_lossy());
					i=i+1;
				}
				jack_free(cports as *mut std::ffi::c_void);
			};

			let js_ports = JsArray::new(&mut cx, ports.len() as u32);
			ports.iter().enumerate().for_each(|e| {
				let (i, obj) = e;
				let js_string = cx.string(obj);
				let _ = js_ports.set(&mut cx, i as u32, js_string);
			});

			Ok(js_ports.upcast())
		}

		method connect(mut cx) {
			let p1 = cx.argument::<JsString>(0)?;
			let p2 = cx.argument::<JsString>(1)?;
			let this = cx.this();

			let res;
			{
				let guard = cx.lock();
	            let jack2 = this.borrow(&guard);
	            
	            res = unsafe { jack_connect(jack2.client, 
	            		CString::new(p1.value()).unwrap().as_ptr(), 
	            		CString::new(p2.value()).unwrap().as_ptr())
	        		  };
	        }
            match res {
            	0 => Ok(cx.undefined().upcast()),
            	_ => panic!(format!("Failed to connect ports, code: {}", res)),
            }

		}


		method get(mut cx) {
			let attr: String = cx.argument::<JsString>(0)?.value();

			let this = cx.this();

			match &attr[..] {
				"client" => {

					Ok(cx.string("hello world!").upcast())
				}
				_ => cx.throw_type_error("property does not exist")
			}
		}

		method panic(_) {
			panic!("User.prototype.panic")
		}
	}
}
//register_module!(mut m, { m.export_class::<JsUser>("User") });
