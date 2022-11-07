use crate::flite_bindings;
extern "C" {
    pub fn register_cmu_us_kal(
        voxdir: *const ::std::os::raw::c_char,
    ) -> *mut flite_bindings::cst_voice;

    pub fn register_cmu_us_slt(
        voxdir: *const ::std::os::raw::c_char,
    ) -> *mut flite_bindings::cst_voice;
}
