use windows:: {
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
mod hotkey;
use hotkey::hotkey;
mod create_window;
mod error;

unsafe extern "system" fn enum_window_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    // Buffer for GetWindowTextW
    let mut title = [0u16; 512];

    unsafe {
        // Remove the SystemStuff.exe Application.exe Explorer.exe stuff
        let mut cloaked = 0u32;
        let hr = DwmGetWindowAttribute(
            hwnd,
            DWMWA_CLOAKED,
            &mut cloaked as *mut _ as *mut _,
            std::mem::size_of::<u32>() as u32
        );
        if hr.is_ok() && cloaked != 0 {
            return true.into()
        }

        if !IsWindowVisible(hwnd).as_bool() {
            return true.into()
        }

        let len = GetWindowTextW(hwnd, &mut title);
        if len > 0 {
            let title = String::from_utf16_lossy(&title[..len as usize]);

            let mut pid = 0u32;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            let process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).unwrap();

            let exe_name = if !process_handle.is_invalid() {
                let mut buffer = [0u16; 260];
                let mut size = buffer.len() as u32;

                QueryFullProcessImageNameW(
                    process_handle,
                    PROCESS_NAME_FORMAT(0),
                    PWSTR(buffer.as_mut_ptr()),
                    &mut size,
                ).unwrap();

                let _ = CloseHandle(process_handle);

                let full_path = String::from_utf16_lossy(&buffer[..size as usize]);
                full_path.rsplit(['\\','/']).next().unwrap_or("").to_string()
            } else {
                "<unknown>".to_string()
            };

            println!("HWND: {:?} | Title: {} | App: {}", hwnd.0, title, exe_name);
        }
    }

    true.into()
}

fn main() {
    unsafe {
        // Ignore results for now
        let _ = EnumWindows(Some(enum_window_proc), LPARAM(0));
        match hotkey() {
            Ok(()) => {}
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
}
