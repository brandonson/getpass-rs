#![feature(libc, convert)]

extern crate libc;

use std::ffi::{CStr, OsStr};
use std::str;


pub fn get_pass(prompt: &str) -> Option<String> {
  let os_str_ref:&OsStr = prompt.as_ref();
  os_str_ref.to_cstring().map(
    |cstr| {
      unsafe {
        let pass = getpass(cstr.as_ptr());
        str::from_utf8(CStr::from_ptr(pass).to_bytes()).unwrap().to_string()
      }
    }
  )
}

extern "C" {
  fn getpass(prompt: *const libc::c_char) ->  *const libc::c_char;
}
