extern crate widestring;

use std::char::{decode_utf16, DecodeUtf16Error};
use widestring::WideCString;

use std::ptr::null;

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn init() {
    unsafe {
        bindings::AU3_Init();
    }
}

pub fn error() -> i32 {
    // TODO: figure out what AU3_error() is used for
    unsafe { bindings::AU3_error() }
}

pub fn mouse_move(x: i32, y: i32, speed: Option<i32>) {
    // TODO: check return value
    let _ = unsafe { bindings::AU3_MouseMove(x, y, speed.unwrap_or(10)) };
}

pub fn mouse_get_pos() -> (i32, i32) {
    let mut lp = bindings::tagPOINT { x: 0, y: 0 };
    unsafe { bindings::AU3_MouseGetPos(&mut lp) };
    (lp.x, lp.y)
}

pub fn win_exists(title: &str, text: Option<&str>) -> bool {
    let title_wide = WideCString::from_str(title).unwrap();

    let r = match text {
        Some(t) => {
            let text_wide = WideCString::from_str(t).unwrap();
            unsafe { bindings::AU3_WinExists(title_wide.as_ptr(), text_wide.as_ptr()) }
        },
        None => {
            unsafe { bindings::AU3_WinExists(title_wide.as_ptr(), null()) }
        },
    };

    r == 1
}

pub fn win_get_text(title: &str, text: Option<&str>, buf_len: Option<usize>) -> Result<String, DecodeUtf16Error> {
    let title_wide = WideCString::from_str(title).unwrap();

    let buf_len = buf_len.unwrap_or(1024);

    let mut buf = Vec::with_capacity(buf_len as usize);
    let buf_ptr = buf.as_mut_ptr();
    
    match text {
        Some(t) => {
            let text_wide = WideCString::from_str(t).unwrap();
            unsafe {
                bindings::AU3_WinGetText(title_wide.as_ptr(), text_wide.as_ptr(), buf_ptr, buf_len as i32);
                buf.set_len(buf_len as usize);
            }
        },
        None => {
            unsafe {
                bindings::AU3_WinGetText(title_wide.as_ptr(), null(), buf_ptr, buf_len as i32);
                buf.set_len(buf_len as usize);
            }
        },
    }

    decode_utf16(buf.iter().cloned().take_while(|x| *x != '\0' as u16))
    .collect::<Result<String, DecodeUtf16Error>>()
}

pub fn win_wait(title: &str, text: Option<&str>, timeout: Option<i32>) {
    let title_wide = WideCString::from_str(title).unwrap();
    let timeout = timeout.unwrap_or(0);

    match text {
        Some(t) => {
            let text_wide = WideCString::from_str(t).unwrap();
            unsafe { bindings::AU3_WinWait(title_wide.as_ptr(), text_wide.as_ptr(), timeout) };
        },
        None => {
            unsafe { bindings::AU3_WinWait(title_wide.as_ptr(), null(), timeout) };
        },
    }
}

pub fn set_option(option: &str, value: i32) {
    let option_wide = WideCString::from_str(option).unwrap();
    
    unsafe {
        bindings::AU3_AutoItSetOption(option_wide.as_ptr(), value);
    };
}

pub fn win_get_handle(title: &str, text: Option<&str>) -> *mut bindings::HWND__ {
    let title_wide = WideCString::from_str(title).unwrap();

    let r = match text {
        Some(t) => {
            let text_wide = WideCString::from_str(t).unwrap();
            unsafe { bindings::AU3_WinGetHandle(title_wide.as_ptr(), text_wide.as_ptr()) }
        },
        None => {
            unsafe { bindings::AU3_WinGetHandle(title_wide.as_ptr(), null()) }
        },
    };
    
    r
}

pub fn win_set_on_top(title: &str, text: Option<&str>, flag: i32) {
    let title_wide = WideCString::from_str(title).unwrap();

    match text {
        Some(t) => {
            let text_wide = WideCString::from_str(t).unwrap();
            unsafe { bindings::AU3_WinSetOnTop(title_wide.as_ptr(), text_wide.as_ptr(), flag) };
        },
        None => {
            unsafe { bindings::AU3_WinSetOnTop(title_wide.as_ptr(), null(), flag) };
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Command, Child};

    fn launch_notepad() -> Child {
        Command::new("notepad.exe").arg("tests\\rs-autoit test1.txt").spawn().unwrap()
    }

    #[test]
    fn test_without_notepad() {
        mouse_move(0, 0, Some(0));
        assert_eq!(mouse_get_pos(), (0, 0));

        mouse_move(50, 50, Some(0));
        assert_eq!(mouse_get_pos(), (50, 50));
    }

    #[test]
    fn test_autoit() {
        assert!(!win_exists("rs-autoit test1", None));

        let mut notepad = launch_notepad();

        win_wait("rs-autoit test1", None, Some(10));
        win_wait("rs-autoit test1", Some("aéèê"), Some(10));

        assert!(win_exists("rs-autoit test1", None));
        assert!(win_exists("rs-autoit test1", Some("aéèê")));
        assert!(!win_exists("rs-autoit test1", Some("aéèêT")));
        assert_eq!(win_get_text("rs-autoit test1", None, None).unwrap(), "aéèê\n");
        assert_eq!(win_get_text("rs-autoit test1", Some("aéèê"), None).unwrap(), "aéèê\n");
        assert_ne!(win_get_text("rs-autoit test1", Some("aéèêT"), None).unwrap(), "aéèê\n");

        notepad.kill().unwrap();
    }
}
