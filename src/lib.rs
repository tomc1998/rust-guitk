extern crate android_glue;
extern crate ffi_glue;
extern crate libc;

pub mod logger;

pub fn add(a: i32, b: i32) -> i32 {
  a * b
}
