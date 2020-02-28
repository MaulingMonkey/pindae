use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::errhandlingapi::{GetLastError, SetLastError};
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

pub unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_CREATE {
        let create_struct = lparam as * mut CREATESTRUCTW;
        let window_state_ptr = create_struct.as_ref().unwrap().lpCreateParams as * mut Window;

        println!("YE {0}", (*window_state_ptr).description.name);

        SetLastError(0);
        if SetWindowLongPtrW(hwnd, GWLP_USERDATA, window_state_ptr as isize ) == 0 {
            println!("YIKES: {0}", GetLastError());
        }
    }

    let window_state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as * mut Window;
    let window_state_option = window_state_ptr.as_mut();
    if window_state_option.is_none() {
        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }

    let window_state = window_state_option.unwrap();
    match msg {
        WM_CREATE => {
            println!("Create: {0}", window_state.description.name);
            window_state.on_event(Events::OnUICreate);
        }
        WM_CLOSE => {
            println!("Close: {0}", window_state.description.name);
            //window_state.on_event(Events::OnUIClose);
            DestroyWindow(hwnd);
        }
        WM_DESTROY => {
            println!("Destroy: {0}", window_state.description.name);
            //window_state.on_event(Events::OnUIDestroy);
            PostQuitMessage(0);
        }
        WM_MOUSEMOVE => {
            println!("MouseMove: {0}", window_state.description.name);
            //window_state.on_event(Events::OnMouseMove);
        }

        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
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
        let title = to_wstring(&self.description.title);

        let hwnd = CreateWindowExW(
            0,
            name.as_ptr(),
            title.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            self.description.pos_x,
            self.description.pos_y,
            self.description.width,
            self.description.height,
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

    fn on_event(&mut self, event: Events) {
        println!("on_event: {0} ",  self.description.name);
        self.events.push(event);
    }
}

