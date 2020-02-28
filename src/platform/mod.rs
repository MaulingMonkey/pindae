#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;


pub struct InstanceDesc {
}

impl InstanceDesc {
    pub fn new() -> Self {
        InstanceDesc {
        }
    }
}

pub struct WindowDesc {
    name: String,
    title: String,
    width: i32,
    height: i32,
    pos_x: i32,
    pos_y: i32,
}

impl WindowDesc {
    pub fn new(name: String, title: String, width: i32, height: i32, pos_x: i32, pos_y: i32) -> Self {
        WindowDesc {
            name,
            title,
            width,
            height,
            pos_x,
            pos_y,
        }
    }
}

pub enum Keycodes {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    Backspace,
    Tab,
    Enter,
    Escape,
    Space,
}

pub enum MouseButton {

}

pub enum Events {
    OnUICreate,
    OnUIClose,
    OnUIDestroy,

    OnMouseDown {
        mouse_button: MouseButton
    },
    OnMouseUp {
        mouse_button: MouseButton
    },
    OnMouseMove,

    OnKeyUp {
        key: Keycodes,
    },
    OnKeyDown {
        key: Keycodes,
    },
}