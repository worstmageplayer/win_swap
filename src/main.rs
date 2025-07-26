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
use win_info::{
    get_window_titles,
};
mod fuzzy;
use fuzzy::fuzzy_match;

fn main() {
    match hotkey(|| {
        let mut input = String::new();
        println!("Enter Input:");
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_string();
        let windows = get_window_titles();
        let matched = fuzzy_match(input, windows);
        match matched {
            Some(win) => println!("Exe name: {}\nHWND: {:?}\nTitle: {}\n",
                win.exe_name,
                win.hwnd.0,
                win.title
            ),
            None => println!("No window found.")
        }
        println!("-------------------");
    }) {
        Ok(()) => {
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
    println!("Program ended");
}
