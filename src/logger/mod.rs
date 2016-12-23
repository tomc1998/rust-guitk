//use android_glue;
use std::ffi::CString;
use libc::{c_int, c_char};
use ffi_glue::ffi;

extern { 
  pub fn __android_log_write(prio: c_int, 
                             tag: *const c_char, 
                             text: *const c_char) -> c_int; 
}

pub enum LogPriority {
  DEBUG,
  DEFAULT,
  ERROR,
  FATAL,
  INFO,
  SILENT,
  UNKNOWN,
  VERBOSE,
  WARN,
}

impl LogPriority {
  pub fn value(&self) -> i32 {
    match *self {
      LogPriority::UNKNOWN => 0,
      LogPriority::DEFAULT => 1,
      LogPriority::VERBOSE => 2,
      LogPriority::DEBUG   => 3,
      LogPriority::INFO    => 4,
      LogPriority::WARN    => 5,
      LogPriority::ERROR   => 6,
      LogPriority::FATAL   => 7,
      LogPriority::SILENT  => 8,
    }
  }
}

static mut DEFAULT_LOG_PRIORITY : LogPriority = LogPriority::DEFAULT;
static mut DEFAULT_LOG_TAG : &'static str = "rust-guitk-app";

/// Log message to logcat
pub fn log(tag: &str, priority: LogPriority, message: &str) {
  let cmessage = CString::new(message).unwrap();
  let cmessage = cmessage.as_ptr();
  let ctag = CString::new(tag).unwrap();
  let ctag = ctag.as_ptr();
  unsafe {
    ffi::__android_log_write(priority.value(), ctag, cmessage);
  }
}

/// Log with the default priority and default tag set
pub fn log_default(message: &str) {
  unsafe {
    let cmessage = CString::new(message).unwrap();
    let cmessage = cmessage.as_ptr();
    let ctag = CString::new(DEFAULT_LOG_TAG).unwrap();
    let ctag = ctag.as_ptr();
    ffi::__android_log_write(DEFAULT_LOG_PRIORITY.value(), ctag, 
                             cmessage);
  }
}

/// Set the tag to be used when logging with log_default(...)
pub fn set_default_log_tag(tag: &'static str) {
  unsafe {
    DEFAULT_LOG_TAG = tag;
  }
}

/// Set the priority to be used when logging with log_default(...)
pub fn set_default_log_priority(priority: LogPriority) {
  unsafe {
    DEFAULT_LOG_PRIORITY = priority;
  }
}
