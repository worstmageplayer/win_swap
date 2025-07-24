use windows::core::Error;
use std::fmt;

#[derive(Debug)]
pub enum HotkeyError {
    RegisterFail(Error),
}

impl fmt::Display for HotkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HotkeyError::RegisterFail(e) => write!(f, "Hotkey registration failed: {e}"),
        }
    }
}
