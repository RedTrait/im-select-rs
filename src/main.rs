use std::env;
use std::process;

use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    ActivateKeyboardLayout, KLF_ACTIVATE, KLF_SETFORPROCESS, LoadKeyboardLayoutW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, HWND_BROADCAST, PostMessageW, WM_INPUTLANGCHANGEREQUEST,
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: im-select 2052 | 1033");
        process::exit(1);
    }

    switch_input_method(&args[1]);
}
