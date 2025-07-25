use super::*;
use error::HotkeyError;
use win_info::{
    WinInfo,
    get_window_titles,
};

pub fn hotkey() -> Result<(), HotkeyError>{
    let result = unsafe { RegisterHotKey(None, 1, MOD_CONTROL | MOD_NOREPEAT, 'J' as u32) };

    match result {
        Ok(_) => {
            println!("RegisterHotKey successful");
        }
        Err(e) => {
            return Err(HotkeyError::RegisterFail(e));
        }
    }

    let mut msg = MSG::default();

    while unsafe { GetMessageW(&mut msg, None, 0, 0).into() } {
        if msg.message == WM_HOTKEY {
            let id = msg.wParam.0 as i32;
            if id == 1 {
                let windows = get_window_titles();
                for win in windows {
                    println!("Exe name: {}\nHWND: {:?}\nTitle: {}\n", win.exe_name, win.hwnd.0, win.title);
                };
            }
        }
    }
    Ok(())
}
