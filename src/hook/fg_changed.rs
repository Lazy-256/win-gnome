/*
    Adapted from: https://github.com/CasualX/external/tree/master/src/hook
*/

use std::ptr::null_mut;
use super::{WinContext, InvokeWin, WinHook};
//use super::InvokeWin;
//use external::hook::{HookContext, Hook};
use winapi::um::winuser::{
	SetWinEventHook, 
    EVENT_SYSTEM_FOREGROUND, WINEVENT_OUTOFCONTEXT, WINEVENT_SKIPOWNPROCESS,
    WH_MOUSE_LL
};
use winapi::shared::windef::HWND;
//WPARAM, LPARAM, MSLLHOOKSTRUCT, c_int, LRESULT
use winapi::um::consoleapi::*;
use winapi::um::errhandlingapi::*;
use winapi::um::handleapi::*;
use winapi::um::memoryapi::*;
use winapi::um::processthreadsapi::*;
use winapi::um::profileapi::*;
use winapi::um::psapi::*;
use winapi::um::synchapi::*;
use winapi::um::tlhelp32::*;
use winapi::um::winbase::*;
use winapi::um::wincon::*;
use winapi::um::wingdi::*;
use winapi::um::winnt::*;
use winapi::um::winuser::*;
use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
//use winapi::shared::ntdef::*;
use winapi::shared::ntdef::UNICODE_STRING;
use winapi::shared::windef::*;
use winapi::shared::winerror::*;
use winapi::ctypes::*;

//pub use ntapi::ntexapi::*;
//pub use ntapi::ntldr::*;

use crate::errors::ErrorCode;
//use external::winapi::*;
use external::hook::HookContext;

pub struct FgWinEvent(WinContext);

impl FgWinEvent {
    pub fn get_hwnd(&self) -> HWND {
        return self.0.hwnd as HWND;
    }
}

pub trait CallFg: InvokeWin {
    fn callback(arg: &mut FgWinEvent);
    /// Registers the low-level mouse hook.
    fn register() -> Result<WinHook, ErrorCode> {
        unsafe {
            let hook = SetWinEventHook(
                EVENT_SYSTEM_FOREGROUND,
                EVENT_SYSTEM_FOREGROUND,
                null_mut(),
                Some(Self::thunk),
                0,
                0,
                WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS,
            );
            if hook.is_null() {
                Err(ErrorCode::last())
            } else {
                Ok(WinHook(hook))
            }
        }
    }
}

// unsafe impl HookContext for FgWinEvent {
// 	fn hook_type() -> c_int {
// 		WH_MOUSE_LL
// 	}
// 	unsafe fn from_raw(code: c_int, w_param: WPARAM, l_param: LPARAM) -> Self {
// 		let message = w_param as u32;
//         let info = l_param as *mut MSLLHOOKSTRUCT;
        
//         FgWinEvent { 
//                 0: WinContext { 
//                         dwEvent: 0,  
//                         dwEventThread: 0,
//                         dwmsEventTime: 0,
//                         idObject: 0,
//                         idChild: 0,
//                         hWinEventHook: 0,
//                         hwnd: 0
//                     }  }
// 	}
// 	unsafe fn call_next_hook(&self) -> LRESULT {
//         if self.0.hWinEventHook != 0 {
// 			self.0.hWinEventHook
// 		}
// 		else {
// 			let w_param = self.0.message as WPARAM;
// 			let l_param = self..info as LPARAM;
// 			CallNextHookEx(ptr::null_mut(), self.code, w_param, l_param)
// 		}
// 	}
// }