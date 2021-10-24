# tinywinstr

Tiny library for handling rust strings in windows.

```rust
#![cfg(target_os = "windows")]
#![windows_subsystem = "windows"]

use std::ptr::null_mut;
use tinywinstr::winstr;
use winapi::um::winuser::{MessageBoxW, MB_OK};

fn main() {
    unsafe {
        MessageBoxW(null_mut(), winstr!("My message"), winstr!("My title"), MB_OK);
    }
}
```