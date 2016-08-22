extern crate winapi;
extern crate user32;

use winapi::*;

fn main() {
    use std::thread;
    use std::time::Duration;
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();

    thread::spawn(move || {
        let mut current = INVALID_HANDLE_VALUE as HWND;
        loop {
            let next = get_foreground_window();
            if current != next {
                current = next;
                tx.send(get_window_title(next)).unwrap();
            }
            thread::sleep(Duration::from_millis(500));
        }
    });

    loop {
        if let Ok(buff) = rx.recv() {
            println!("current:  {}", buff);
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn get_foreground_window() -> HWND {
    unsafe { user32::GetForegroundWindow() }
}

fn get_window_text_length(whdl: HWND) -> i32 {
    unsafe { user32::GetWindowTextLengthW(whdl) }
}

fn get_window_title(whdl: HWND) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    let length = get_window_text_length(whdl);
    let mut title = vec![ 0u16; (length + 1) as usize ];
    unsafe {
        user32::GetWindowTextW(whdl, title.as_mut_ptr(), length + 1);
    }
    title.truncate(length as usize);
    OsString::from_wide(&title).to_string_lossy().into_owned()
}
