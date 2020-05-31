// see: https://gist.github.com/TheSatoshiChiba/6dd94713669efd1636efe4ee026b67af
use core::mem::MaybeUninit;

use crate::winapi;

use std::ffi::OsStr;
use std::ffi::CString;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::mem;
use std::mem::zeroed;
use std::ptr::null_mut;
use std::io::Error;
use std::os::raw::c_int;

use std::sync::Arc;
use std::thread;
use std::convert::TryInto;

use winapi::shared::windef::{
    HWND, 
    HBRUSH,
    RECT,
    HDC,
    COLORREF,
    HGDIOBJ,
};
use winapi::shared::minwindef::{
    UINT, 
    WPARAM,
    LPARAM, 
    LRESULT,
    BOOL,
    BYTE,
};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winnt::{
    LPCWSTR,
    LPCSTR,
    LONG,
};

use winapi::um::wingdi::{
    TextOutA,
    SetBkMode,
    SetBkColor,
    SetTextColor,
    RGB,
    CreateFontA,
    ANSI_CHARSET,
    DEFAULT_CHARSET,
    RUSSIAN_CHARSET,
    VARIABLE_PITCH,
    DEFAULT_PITCH,
    FF_ROMAN,
    TRANSPARENT,
    SelectObject,
    DeleteObject,
};

use winapi::um::winuser::{
    DefWindowProcW,
    RegisterClassW,
    CreateWindowExW,
    TranslateMessage,
    DispatchMessageW,
    GetMessageW,
    PostQuitMessage,
    BeginPaint,
    EndPaint,
    GetClientRect,
    DrawTextA,
    FillRect,
    MessageBoxA,
    SetWindowPos,
    GetWindowLongA,
    SetWindowLongA,
    SetLayeredWindowAttributes,
    GetSysColor,
    FindWindowExA,
};

use winapi::um::winuser::{
    MSG,
    WNDCLASSW,
    CS_OWNDC,
    CS_HREDRAW,
    CS_VREDRAW,
    CW_USEDEFAULT,
    WS_OVERLAPPEDWINDOW,
    WS_VISIBLE,
    WS_THICKFRAME,
    WS_CAPTION,
    WS_SYSMENU,
    WS_POPUP,
    WS_MINIMIZEBOX,
    WS_EX_APPWINDOW,
    WS_EX_TRANSPARENT, 
    WS_EX_NOREDIRECTIONBITMAP,
    WS_EX_LAYERED,
    WS_EX_TOOLWINDOW,
    DT_SINGLELINE,
    DT_CENTER,
    DT_VCENTER,
    WM_HOTKEY,
    WM_CLOSE,
    PAINTSTRUCT,
    COLOR_WINDOW,
    COLOR_WINDOWTEXT,
    COLOR_DESKTOP,
    COLOR_BTNFACE,
    COLOR_ACTIVECAPTION,
    COLOR_HIGHLIGHT,
    COLOR_HIGHLIGHTTEXT,
    MB_OK, 
    MB_ICONINFORMATION,
    HWND_TOPMOST,
    SWP_NOMOVE,
    SWP_NOSIZE,
    GWL_STYLE,
    GWL_EXSTYLE,
    LWA_ALPHA,
    LWA_COLORKEY,
};

#[derive(Debug)]
#[cfg(windows)]
pub struct MarkerWnd {
  pub handle : HWND,
  //_text: Vec<u16>,
}

//static SZ_TEXT: &'static Vec<u16> = OsStr::new("Hello!").encode_wide().collect();
//&'static [u16] = OsStr::new("Hello!").encode_wide();
//OsStr::new("Hello!").encode_wide().chain( once( 0 ) ).collect();
static SZ_TEXT: &'static [u8] = b" F      W        E       S     P ";
static SZ_FONTNAME: &'static [u8] = b"Consolas"; // Consolas Segoe UI
static PRNT_CLASSNAME: &'static [u8] = b"win_gnome";

#[cfg(windows)]
impl MarkerWnd {
    #[cfg(windows)]
    pub fn create_window( x:i32, y:i32, width:i32, heigh:i32 ) -> Result<MarkerWnd, Error> {
        let classname:Vec<u16> = OsStr::new("MarkerWnd").encode_wide().chain( once( 0 ) ).collect();
         // encode_wide does NOT add a null terminator
        //name.push(0);

        //let title:Vec<u16> = OsStr::new("").encode_wide().chain( once( 0 ) ).collect();

        unsafe {
            let hinstance = GetModuleHandleW( null_mut() );
            let wnd_class = WNDCLASSW {
                style : CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc : Some ( MarkerWnd::wnd_proc ),  //( DefWindowProcW ), MarkerWnd::wnd_proc
                hInstance : hinstance,
                lpszClassName : classname.as_ptr(),
                cbClsExtra : 0,
                cbWndExtra : 0,
                hIcon: null_mut(),
                hCursor: null_mut(),
                hbrBackground: null_mut(),
                lpszMenuName: null_mut(),
            };

            RegisterClassW( &wnd_class );

            // get parrent hwnd to hide window
            //let prntClsNm:Vec<u16> = OsStr::new(crate::IDENTIFIER).encode_wide().chain( once( 0 ) ).collect();
            //let prntClsNm = CString::new("win_gnome").unwrap(); // CString::new(crate::IDENTIFIER);
            //let parrentHWND: HWND = FindWindowExA( null_mut(), null_mut(), 
            //    PRNT_CLASSNAME.as_ptr() as *const i8, null_mut() );
            //println!("parrentHWND: {:?}", &parrentHWND);    

            let handle = CreateWindowExW(   // WS_EX_TRANSPARENT | WS_EX_LAYERED
                WS_EX_TRANSPARENT | WS_EX_LAYERED | WS_EX_TOOLWINDOW ,  // transparent for inputs | transparrent | WS_EX_NOREDIRECTIONBITMAP
                classname.as_ptr(),
                0 as LPCWSTR,
                WS_VISIBLE | WS_POPUP, // without titlebar see: https://docs.microsoft.com/en-us/windows/win32/winmsg/window-styles
                x as c_int, // CW_USEDEFAULT,
                y as c_int,
                width as c_int,
                heigh as c_int,
                null_mut(),  //parrentHWND, // parrent should be setet up
                null_mut(),
                hinstance,
                null_mut() );

            // set window on top
            SetWindowPos(handle, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);

            // exclude from alt tab

            // Transparent background
            SetLayeredWindowAttributes(handle, GetSysColor(COLOR_WINDOW), 10, LWA_COLORKEY);
            
            if handle.is_null() {
                Err( Error::last_os_error() )
            } else {
                Ok( MarkerWnd { 
                    handle 
                    } )
            }
        }
    }

    unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            winapi::um::winuser::WM_DESTROY => {
                PostQuitMessage(0);
            },
            winapi::um::winuser::WM_PAINT => {
                let mut ps = PAINTSTRUCT {
                    hdc: null_mut(),
                    fErase: 0 as BOOL,
                    rcPaint: RECT {
                        left: 0 as LONG, top: 0 as LONG,
                        right: 0 as LONG, bottom: 0 as LONG
                    },
                    fRestore: 0  as BOOL,
                    fIncUpdate: 0  as BOOL,
                    rgbReserved: [0 as BYTE; 32], 
                };
    
                let hdc: HDC = BeginPaint(hwnd, &mut ps);
                // colors: https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-getsyscolor
                FillRect(hdc, &ps.rcPaint, GetSysColor(COLOR_WINDOW) as HBRUSH);
                // GetSysColor(COLOR_ACTIVECAPTION)
                SetTextColor(hdc, RGB(255, 82, 51) as COLORREF); 
                let font = CreateFontA(25, 0, 0, 0, 100, 0, 0, 0, ANSI_CHARSET, 
                    0, 0, 0, VARIABLE_PITCH , SZ_FONTNAME.as_ptr() as *const i8);
                    // VARIABLE_PITCH | FF_ROMAN
                    // "Consolas\0"
                SelectObject(hdc, font as HGDIOBJ);    
                TextOutA(hdc, 5, 5,
                    SZ_TEXT.as_ptr() as *const i8,
                    SZ_TEXT.len() as i32
                );
                DeleteObject(font as HGDIOBJ);
                EndPaint(hwnd, &mut ps);
                
                // MessageBoxA(
                //     null_mut(),
                //     SZ_TEXT.as_ptr() as *const i8,
                //     SZ_TEXT.as_ptr() as *const i8,
                //     MB_OK | MB_ICONINFORMATION
                // );                
            },
            winapi::um::winuser::WM_ERASEBKGND => {
                SetBkColor((wparam as HDC), GetSysColor(COLOR_WINDOW));
                SetLayeredWindowAttributes(hwnd, GetSysColor(COLOR_WINDOW), 100, LWA_COLORKEY);
            },
            _ => {
                return DefWindowProcW(hwnd, msg, wparam, lparam);
            }
        }
        return 0;
    }

    // Create message handling function with which to link to hook window to Windows messaging system
    // More info: https://msdn.microsoft.com/en-us/library/windows/desktop/ms644927(v=vs.85).aspx
    pub fn handle_message(window: &MarkerWnd, 
            on_hot_key: impl Fn() -> bool, 
            on_close: impl Fn() -> bool) -> bool {
        unsafe {
            let mut message: MSG = mem::uninitialized();
    
            // Get message from message queue with GetMessageW
            if GetMessageW(&mut message as *mut MSG, window.handle, 0, 0) > 0 {
                TranslateMessage(&message as *const MSG); // Translate message into something meaningful with TranslateMessage
                DispatchMessageW(&message as *const MSG); // Dispatch message with DispatchMessageW
                if message.message == WM_HOTKEY {
                    return on_hot_key();
                } else if message.message == WM_CLOSE {
                    return on_close();
                }
                return true;
            } else {
                return false;
            }
        }
    }    

}