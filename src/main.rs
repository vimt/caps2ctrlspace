#![windows_subsystem = "windows"]
use windows::{
    core::*, Win32::Foundation::*, Win32::System::LibraryLoader::*,
    Win32::UI::WindowsAndMessaging::*, Win32::UI::Input::KeyboardAndMouse::*,
};

static mut HOOK_ID: HHOOK = HHOOK(0);
const KEY_CAPS_LOCK: u32 = 20;
const KEY_CONTROL: u32 = 17;
const KEY_SPACE: u32 = 32;

unsafe extern "system" fn proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < 0 || wparam.0 as u32 != WM_KEYDOWN {
        return CallNextHookEx(HOOK_ID, code, wparam, lparam);
    }
    let hook: *const KBDLLHOOKSTRUCT = lparam.0 as _;

    if (*hook).vkCode != KEY_CAPS_LOCK {
        return CallNextHookEx(HOOK_ID, code, wparam, lparam);
    }

    let caps_state = GetKeyState(KEY_CAPS_LOCK as i32);
    let caps_lock_is_on = caps_state & 1 == 1;
    if caps_lock_is_on {
        // set caps_lock off
        keybd_event(KEY_CAPS_LOCK as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(KEY_CAPS_LOCK as u8, 0, KEYEVENTF_KEYUP, 0);
    }

    // 发送ctrl+space
    keybd_event(KEY_CONTROL as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
    keybd_event(KEY_SPACE as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
    keybd_event(KEY_CONTROL as u8, 0, KEYEVENTF_KEYUP, 0);
    keybd_event(KEY_SPACE as u8, 0, KEYEVENTF_KEYUP, 0);
    return LRESULT(1);
}

fn main() -> Result<()> {
    unsafe {
        let hmod = GetModuleHandleA(PCSTR::null()).unwrap();
        HOOK_ID = SetWindowsHookExA(WH_KEYBOARD_LL, Some(proc), hmod, 0).unwrap();

        if HOOK_ID.0 == 0 {
            panic!("SetWindowsHookExA");
        }

        // block (from lswitch.c)
        let mut msg = MSG { ..Default::default() };
        while GetMessageA(&mut msg, HWND::default(), 0, 0).0 != 0 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        UnhookWindowsHookEx(HOOK_ID);
        Ok(())
    }
}
