extern crate android_glue;
extern crate ffi_glue;
extern crate libc;

pub mod logger;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub fn add(a: i32, b: i32) -> i32 {
  a * b
}
