//           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                   Version 2, December 2004
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

#![allow(non_upper_case_globals)]

extern crate libc;
#[macro_use] extern crate lazy_static;

pub mod linux_tcp;

use libc::{SOL_TCP, c_int, c_void, size_t, ssize_t, socklen_t, getsockopt};
use linux_tcp::{TCP_INFO, tcp_info};
use std::ffi::CString;
use std::mem;

type SendFn = extern "C"
    fn(socket: c_int, buffer: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
type RecvFn = extern "C"
    fn(socket: c_int, buffer: *mut c_void, len: size_t, flags: c_int) -> ssize_t;

lazy_static! {
    static ref orig_send: SendFn = get_next_sym("send");
    static ref orig_recv: RecvFn = get_next_sym("recv");
}

fn get_next_sym<T>(sym_name: &str) -> T {
    let sym_name = CString::new(sym_name).unwrap();

    unsafe {
        libc::dlsym(libc::RTLD_NEXT, sym_name.as_ptr())
            .as_ref()
            .map(|sym|
                 mem::transmute_copy(&sym)
                ).unwrap()
    }
}

unsafe fn dump_tcp_info(socket: c_int) {
    let info: tcp_info = mem::zeroed();
    let mut size = mem::size_of::<tcp_info>() as socklen_t;

    getsockopt(socket, SOL_TCP, (TCP_INFO as i32), mem::transmute(&info), &mut size);
    println!("{:#?}", info);
}

#[no_mangle]
pub extern "C" fn send(socket: c_int, buffer: *mut c_void, len: size_t, flags: c_int) -> ssize_t {
    unsafe {
        dump_tcp_info(socket);
    }

    orig_send(socket, buffer, len, flags)
}

#[no_mangle]
pub extern "C" fn recv(socket: c_int, buffer: *mut c_void, len: size_t, flags: c_int) -> ssize_t {
    unsafe {
        dump_tcp_info(socket);
    }

    orig_recv(socket, buffer, len, flags)
}
