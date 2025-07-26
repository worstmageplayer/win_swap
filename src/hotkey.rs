use super::*;
use error::HotkeyError;
use win_info::{
    WinInfo,
    get_window_titles,
};

pub fn hotkey<F>(callback: F) -> Result<(), HotkeyError>
where
    F: Fn() + 'static,
{
    let result = unsafe { RegisterHotKey(None, 1, MOD_CONTROL | MOD_NOREPEAT, 'J' as u32) };
    if let Some(e) = result.err() {
        return Err(HotkeyError::RegisterFail(e));
    }
    println!("RegisterHotKey successful");

    let mut msg = MSG::default();

    while unsafe { GetMessageW(&mut msg, None, 0, 0).into() } {
        if msg.message == WM_HOTKEY {
            let id = msg.wParam.0 as i32;
            if id == 1 {
                callback();
            }
        }
    }

    Ok(())
}
