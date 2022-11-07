#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
mod flite_bindings;
mod register_voices;

pub use flite_bindings::*;
pub use register_voices::*;

#[test]
fn test() {
    use std::ffi::CString;
    use std::ptr;
    let p: *const i8 = ptr::null();
    let text: CString = CString::new("hi").expect("CString::new failed");
    let output: CString = CString::new("play").expect("CString::new failed");

    unsafe {
        flite_bindings::flite_init();
        let v = register_voices::register_cmu_us_slt(p);
        flite_bindings::flite_text_to_speech(text.as_ptr(), v, output.as_ptr());
    }
    assert_eq!(1, 1);
}
