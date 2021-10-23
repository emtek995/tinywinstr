#[macro_export]
macro_rules! winstr {
    ($s:expr) => {{
        $s.encode_utf16()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>()
            .as_ptr()
    }};
}
