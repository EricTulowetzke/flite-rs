use std::ffi::CString;
use std::ptr;
mod voices;
type CstVoice = flite_sys::cst_voice;

pub enum Voice {
    Slt,
    Kal,
}

pub fn flite_init() -> i32 {
    unsafe { flite_sys::flite_init() }
}

pub fn register_cmu_us_slt() -> *mut CstVoice {
    unsafe { flite_sys::register_cmu_us_slt(ptr::null()) }
}

pub fn register_cmu_us_kal() -> *mut CstVoice {
    unsafe { flite_sys::register_cmu_us_kal(ptr::null()) }
}

pub struct Flite {
    voice_name: Voice,

    voice: *mut CstVoice,
}

impl Flite {
    pub fn new(voice: Voice) -> Self {
        flite_init();
        let v: *mut CstVoice = match voice {
            Voice::Kal => register_cmu_us_kal(),
            _ => register_cmu_us_slt(),
        };
        Self {
            voice_name: voice,
            voice: v,
        }
    }

    pub fn unregister_cmu_us_slt(&mut self) {
        unsafe { flite_sys::unregister_cmu_us_slt(self.voice) }
    }

    pub fn unregister_cmu_us_kal(&mut self) {
        unsafe { flite_sys::unregister_cmu_us_kal(self.voice) }
    }
    pub fn flite_text_to_speech(&self, text: &str, outtype: &str) -> f32 {
        let input: CString = CString::new(text).expect("CString::new failed");
        let output: CString = CString::new(outtype).expect("CString::new failed");
        if self.voice.is_null() {
            -1.0
        } else {
            unsafe { flite_sys::flite_text_to_speech(input.as_ptr(), self.voice, output.as_ptr()) }
        }
    }
}

impl Drop for Flite {
    fn drop(&mut self) {
        match &self.voice_name {
            Voice::Kal => self.unregister_cmu_us_kal(),
            _ => self.unregister_cmu_us_slt(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let f: Flite = Flite::new(Voice::Kal);
        f.flite_text_to_speech("hi how are you eric?", "play");
        assert_eq!(4, 4);
    }
}
