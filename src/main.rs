use std::env;
use std::process;

use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    ActivateKeyboardLayout, GetKeyboardLayout, KLF_ACTIVATE, KLF_SETFORPROCESS, LoadKeyboardLayoutW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowThreadProcessId, HWND_BROADCAST, PostMessageW,
    WM_INPUTLANGCHANGEREQUEST,
};
use windows::core::w;

fn locale_to_klid(locale: &str) -> windows::core::PCWSTR {
    match locale {
        "1033" => w!("00000409"),
        "2052" => w!("00000804"),
        _ => {
            eprintln!("Invalid locale: {locale}, only 2052 or 1033 are allowed");
            process::exit(1);
        }
    }
}

fn current_locale() -> Option<i32> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_invalid() {
            return None;
        }

        let tid = GetWindowThreadProcessId(hwnd, None);
        if tid == 0 {
            return None;
        }

        let hkl = GetKeyboardLayout(tid);

        // HKL 的低 16 位是 LANGID：
        // 0x0409 = 1033 = English (United States)
        // 0x0804 = 2052 = Chinese (Simplified)
        Some((hkl.0 as usize & 0xFFFF) as i32)
    }
}

fn switch_input_method(locale: &str) {
    unsafe {
        let klid = locale_to_klid(locale);

        let hkl = LoadKeyboardLayoutW(klid, KLF_ACTIVATE).expect("LoadKeyboardLayoutW failed");

        let _ = ActivateKeyboardLayout(hkl, KLF_SETFORPROCESS);

        let wparam = WPARAM(0);
        let lparam = LPARAM(hkl.0 as isize);

        let hwnd = GetForegroundWindow();

        if !hwnd.is_invalid() {
            let _ = PostMessageW(Some(hwnd), WM_INPUTLANGCHANGEREQUEST, wparam, lparam);
        }

        let _ = PostMessageW(
            Some(HWND_BROADCAST),
            WM_INPUTLANGCHANGEREQUEST,
            wparam,
            lparam,
        );
    }
}

fn toggle_input_method() {
    match current_locale() {
        Some(1033) => switch_input_method("2052"),
        Some(2052) => switch_input_method("1033"),
        Some(locale) => {
            eprintln!("Current locale is {locale}, fallback to 1033");
            switch_input_method("1033");
        }
        None => {
            eprintln!("Cannot detect current locale, fallback to 1033");
            switch_input_method("1033");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => toggle_input_method(),
        2 => switch_input_method(&args[1]),
        _ => {
            println!("Usage:");
            println!("  im-select-rs.exe        # toggle 1033 <-> 2052");
            println!("  im-select-rs.exe 1033   # switch to English");
            println!("  im-select-rs.exe 2052   # switch to Chinese");
            process::exit(1);
        }
    }
}
