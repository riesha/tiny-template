//! Wrappers for WinAPI functions

use winapi::um::{
    consoleapi::AllocConsole,
    wincon::{FreeConsole, SetConsoleTitleA},
};
#[macro_export]
macro_rules! to_cstr {
    ($rstring:expr) => {
        alloc::format!("{}\0", $rstring).as_ptr() as *const i8
    };
    ($rstring:literal) => {
        concat!($rstring, "\0").as_ptr() as *const i8
    };
}
pub fn alloc_console(title: &str) -> bool
{
    let alloc_console = unsafe { AllocConsole() };
    if alloc_console == 0
    {
        return false;
    }

    let console_title = unsafe { SetConsoleTitleA(to_cstr!(title)) };
    console_title != 0
}
pub fn free_console() -> bool
{
    let free_console = unsafe { FreeConsole() };
    free_console != 0
}
