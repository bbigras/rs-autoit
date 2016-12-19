use std::char::decode_utf16;
use std::ptr::null;

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    #[allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn str_to_lpcwstr(str_in: &str) -> bindings::LPCWSTR {
    let null_terminator = vec!['\0' as u16];
    str_in.encode_utf16().chain(null_terminator).collect::<Vec<u16>>().as_ptr()
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
    let title_ptr = str_to_lpcwstr(title);
    let text_ptr = if let Some(t) = text {
        str_to_lpcwstr(t)
    } else {
        null()
    };

    let r = unsafe { bindings::AU3_WinExists(title_ptr, text_ptr) };
    r == 1
}

pub fn win_get_text(title: &str, text: Option<&str>, buf_len: Option<usize>) -> String {
    let title_ptr = str_to_lpcwstr(title);
    let text_ptr = if let Some(t) = text {
        str_to_lpcwstr(t)
    } else {
        null()
    };

    let buf_len = buf_len.unwrap_or(1024);

    let mut buf = Vec::with_capacity(buf_len as usize);
    let buf_ptr = buf.as_mut_ptr();

    unsafe {
        bindings::AU3_WinGetText(title_ptr, text_ptr, buf_ptr, buf_len as i32);
        buf.set_len(buf_len as usize);
    };

    decode_utf16(buf.iter().cloned().take_while(|x| *x != '\0' as u16))
        .map(|y| y.unwrap() as char)
        .collect::<String>()
}

pub fn win_wait(title: &str, text: Option<&str>, timeout: Option<i32>) {
    let title_ptr = str_to_lpcwstr(title);
    let text_ptr = if let Some(t) = text {
        str_to_lpcwstr(t)
    } else {
        null()
    };

    let timeout = timeout.unwrap_or(0);

    unsafe {
        bindings::AU3_WinWait(title_ptr, text_ptr, timeout);
    };
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
        assert!(!win_exists("rs-autoit test1.txt", None));

        let mut notepad = launch_notepad();

        win_wait("rs-autoit test1.txt", None, Some(10));

        assert!(win_exists("rs-autoit test1.txt", None));
        assert_eq!(win_get_text("rs-autoit test1.txt", None, None), "aéèê\n");

        notepad.kill().unwrap();
    }
}
