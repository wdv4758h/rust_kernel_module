////////////////////////////////////////
// make some C types
////////////////////////////////////////

#[allow(non_camel_case_types)]
pub type c_char = i8;

////////////////////////////////////////
// get some kernel functions
////////////////////////////////////////

#[cfg(feature = "Linux")]
extern {
    pub fn printk(fmt: *const c_char);
}

#[cfg(feature = "FreeBSD")]
extern {
    pub fn uprintf(fmt: *const c_char);
}

////////////////////////////////////////
// wrapper
////////////////////////////////////////

#[cfg(feature = "Linux")]
pub fn kprint(fmt: *const c_char) {
    unsafe { printk(fmt); }
}

#[cfg(feature = "FreeBSD")]
pub fn kprint(fmt: *const c_char) {
    unsafe { uprintf(fmt); }
}
