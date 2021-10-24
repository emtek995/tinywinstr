#[macro_export]
macro_rules! winstr {
    ($s:expr) => {{
        use std::os::windows::ffi::OsStrExt;
        std::ffi::OsStr::new($s)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>()
            .as_ptr()
    }};
}
