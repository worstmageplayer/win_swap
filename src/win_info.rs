use super::*;

#[derive(Debug)]
pub struct WinInfo {
    pub hwnd: HWND,
    pub title: String,
    pub exe_name: String,
}

unsafe extern "system" fn enum_window_proc(hwnd: HWND, l_param: LPARAM) -> BOOL {
    unsafe {
        let windows = &mut *(l_param.0 as *mut Vec<WinInfo>);

        let mut cloaked = 0u32;
        let this = DwmGetWindowAttribute(
            hwnd,
            DWMWA_CLOAKED,
            &mut cloaked as *mut u32 as *mut c_void,
            size_of::<u32>() as u32,
        );
        if this.is_ok() && cloaked != 0 {
            return true.into();
        }

        if !IsWindowVisible(hwnd).as_bool() {
            return true.into();
        }

        let mut title_buffer = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut title_buffer);
        if len == 0 {
            return true.into();
        }

        let title = String::from_utf16_lossy(&title_buffer[..len as usize]);

        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        let exe_name = {
            let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid);
            if let Ok(handle) = handle {
                let mut buffer = [0u16; 260];
                let mut size = buffer.len() as u32;

                let result = QueryFullProcessImageNameW(
                    handle,
                    PROCESS_NAME_FORMAT(0),
                    PWSTR(buffer.as_mut_ptr()),
                    &mut size,
                );
                let _ = CloseHandle(handle);

                if result.is_ok() {
                    let full_path = String::from_utf16_lossy(&buffer[..size as usize]);
                    full_path.rsplit(['\\', '/']).next().unwrap_or("").to_string()
                } else {
                    "<unknown>".to_string()
                }
            } else {
                "<unknown>".to_string()
            }
        };

        windows.push(WinInfo {
            hwnd,
            title,
            exe_name,
        });

        true.into()
    }
}

pub fn get_window_titles() -> Vec<WinInfo> {
    let mut titles = Vec::new();
    unsafe {
        let lparam = LPARAM(&mut titles as *mut _ as isize);
        let _ = EnumWindows(Some(enum_window_proc), lparam);
    }
    titles
}
