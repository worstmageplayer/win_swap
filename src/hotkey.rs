use super::*;
use error::HotkeyError;

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
                println!("Hotkey pressed");
            }
        }
    }
    Ok(())
}
