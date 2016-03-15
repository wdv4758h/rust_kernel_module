////////////////////////////////////////
// convenient macros
////////////////////////////////////////

macro_rules! cstr {
    ($arg:expr) => (concat!($arg, "\0"))
}

macro_rules! print {
    ($str:expr) => ({
        use core::str::StrExt;
        let cstring = cstr!($str);
        let ptr = cstring.as_ptr() as *const $crate::kernel::c_char;
        $crate::kernel::kprint(ptr);
    })
}

macro_rules! println {
    ($str:expr) => (print!(concat!($str, "\n")))
}
