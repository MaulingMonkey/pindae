use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;
use winapi::shared::windef::*;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winuser::*;

use std::ptr::null_mut;
use winapi::ctypes::c_void;
use crate::platform::{WindowDesc, Events};

fn to_wstring(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;

    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub unsafe fn pwstr_to_string(ptr: PWSTR) -> String {
    use std::slice::from_raw_parts;
    let len = (0_usize..)
        .find(|&n| *ptr.add(n) == 0)
        .expect("Couldn't find null terminator");
    let array: &[u16] = from_raw_parts(ptr, len);
    String::from_utf16_lossy(array)
}

pub unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_CREATE {
        let create_struct = lparam as * mut CREATESTRUCTW;
        let window_state_ptr = create_struct.as_ref().unwrap().lpCreateParams as * mut Window;
        let window_state : &mut Window = window_state_ptr.as_mut().unwrap();

        SetWindowLongPtrW(hwnd, GWLP_USERDATA, window_state_ptr as isize );
    }

    let window_state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as * mut Window;
    let window_state_option = window_state_ptr.as_mut();


    if window_state_option.is_some() {
        let window_state = window_state_option.unwrap();
        match msg {
            WM_CREATE => {
                window_state.OnEvent(Events::OnUICreate);
            }
            WM_CLOSE => {
                window_state.OnEvent(Events::OnUIClose);
                //DestroyWindow(hwnd);
            }
            WM_DESTROY => {
                window_state.OnEvent(Events::OnUIDestroy);
                //PostQuitMessage(0);
            }
            WM_MOUSEMOVE => {

            }

            _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
    else {
        return DefWindowProcW(hwnd, msg, wparam, lparam)
    }
    0
}

pub struct Window {
    description: WindowDesc,
    pub hwnd: HWND,
    pub events: Vec<Events>,
}

impl Window {
    pub fn new(description: WindowDesc, hinstance: &HINSTANCE) -> Option<Self> {
        let hwnd = null_mut();

        let mut window = Window {
            description,
            hwnd,
            events: Vec::new(),
        };

        unsafe {
            if window.register_window(&hinstance) == false {
                println!("[platform/windows/window.rs] Error in register_window");
            }

            match window.create_window(&hinstance) {
                Ok(hwnd) => window.hwnd = hwnd,
                Err(error_code) => {
                    println!("[platform/windows/window.rs] Error in create_window with error code {0}", error_code);
                    return None;
                }
            };
        }

        Some(window)
    }

    unsafe fn register_window(&self, hinstance: &HINSTANCE) -> bool {
        let name = to_wstring(&self.description.name);
        let title = to_wstring(&self.description.name);

        let wnd_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: *hinstance,
            hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: COLOR_WINDOW as HBRUSH,
            lpszMenuName: null_mut(),
            lpszClassName: name.as_ptr(),
            hIconSm: LoadIconW(null_mut(), IDI_APPLICATION),
        };

        if RegisterClassExW(&wnd_class) == 0 {
            false
        }
        else {
            true
        }
    }

    unsafe fn create_window(&mut self, hinstance: &HINSTANCE) -> Result<HWND, u32> {
        let name = to_wstring(&self.description.name);
        let title = to_wstring(&self.description.name);

        let hwnd = CreateWindowExW(
            0,
            name.as_ptr(),
            title.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            630,
            470,
            null_mut(),
            null_mut(),
            *hinstance,
            self as *mut Window as *mut c_void,
        );

        if hwnd.is_null() {
            return Err(GetLastError());
        }

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        Ok(hwnd)
    }

    fn OnEvent(&mut self, event: Events) {
        self.events.push(event);
    }
}

