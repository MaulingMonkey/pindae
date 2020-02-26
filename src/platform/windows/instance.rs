use winapi::shared::minwindef::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;

use std::ptr::null_mut;

use crate::platform::windows::window::Window;
use crate::platform::{InstanceDesc, WindowDesc, Events};

pub struct Instance {
    hinstance: HINSTANCE,
    description: InstanceDesc,
    window: Window,
}

impl Instance {
    pub fn new(instance_desc: InstanceDesc, window_desc: WindowDesc) -> Option<Self> {
        unsafe {
            let hinstance = GetModuleHandleW(null_mut());

            let window = Window::new(window_desc, &hinstance);
            return match window {
                Some(window) => {
                    let i = Instance {
                        hinstance,
                        description: instance_desc,
                        window,
                    };
                    Some(i)
                }
                None => None
            }
        }
    }

    pub fn poll_events(&mut self) -> Vec<Events> {
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            while PeekMessageA(&mut msg, self.window.hwnd, 0, 0, PM_REMOVE) > 0 {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
        self.window.events.split_off(0)
    }

}