#![allow(unused_imports)]
use windows::{
    core::{
        BOOL,
        PWSTR,
    },
    Win32::{
        Foundation::{
            HWND,
            LPARAM,
            CloseHandle,
        },
        System::{
            Threading::{
                OpenProcess,
                PROCESS_QUERY_LIMITED_INFORMATION,
                PROCESS_NAME_FORMAT,
                QueryFullProcessImageNameW,
            }
        },
        UI::{
            Input::KeyboardAndMouse::{
                RegisterHotKey,
                MOD_CONTROL,
                MOD_NOREPEAT,
            },
            WindowsAndMessaging::{
                EnumWindows,
                GetMessageW,
                GetWindowTextW,
                GetWindowThreadProcessId,
                IsWindowVisible,
                SetForegroundWindow,
                MSG,
                WM_HOTKEY,
            }
        },
        Graphics::{
            Dwm::{
                DwmGetWindowAttribute,
                DWMWA_CLOAKED,
            },
        }
    }
};
use std::{
    ffi::c_void,
    io::{
        stdin,
        stdout,
        Write,
    },
    mem::{
        size_of,
    },
};
mod hotkey;
use hotkey::hotkey;
mod create_window;
mod error;
mod win_info;

fn main() {
    match hotkey() {
        Ok(()) => {
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
    println!("Program ended");
}
