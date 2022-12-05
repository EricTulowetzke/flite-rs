use std::ffi::CString;

pub type CstVoice = flite_sys::cst_voice;
pub struct Voice {
    name: String,
    voice: Box<*mut CstVoice>,
    unregister: fn(Box<*mut CstVoice>),
}

impl Voice {
    pub fn new<F1>(
        name: String,
        voxdir: String,
        register: F1,
        unregister: fn(Box<*mut CstVoice>),
    ) -> Self
    where
        F1: Fn(String) -> Box<*mut CstVoice>,
    {
        let voice: Box<*mut CstVoice> = register(voxdir);
        Voice {
            name,
            voice,
            unregister,
        }
    }
}

impl Drop for Voice {
    fn drop(&mut self) {
        (self.unregister)(self.voice.to_owned());
    }
}
pub fn register_cmu_us_slt(voxdir: String) -> Box<*mut CstVoice> {
    let vox: CString = CString::new(voxdir).unwrap();
    Box::new(unsafe { flite_sys::register_cmu_us_slt(vox.as_ptr() as *const i8) })
}

pub fn unregister_cmu_us_slt(cst_voice: Box<*mut CstVoice>) {
    unsafe { flite_sys::unregister_cmu_us_slt(*cst_voice.as_ref()) }
}

pub fn register_cmu_us_kal(voxdir: String) -> *mut CstVoice {
    let vox: CString = CString::new(voxdir).unwrap();
    unsafe { flite_sys::register_cmu_us_kal(vox.as_ptr() as *const i8) }
}

pub fn unregister_cmu_us_kal(cst_voice: Box<*mut CstVoice>) {
    unsafe { flite_sys::unregister_cmu_us_kal(*cst_voice.as_ref()) }
}
