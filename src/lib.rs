#[macro_export]
macro_rules! winstr {
    ($s:expr) => {{
        $s.encode_utf16()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>()
            .as_ptr()
    }};
}

pub trait WinStr {
    fn to_winstr(self) -> Vec<u16>;
}

impl WinStr for &str {
    fn to_winstr(self) -> Vec<u16> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        OsStr::new(self)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect()
    }
}
