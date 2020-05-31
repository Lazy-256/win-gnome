use crate::winapi;

// see: https://github.com/gabdube/native-windows-gui/blob/master/examples
// see: https://github.com/rust-windowing/winit/tree/master/examples

// winit 

use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

#[derive(Debug)]
#[cfg(windows)]
pub struct LabelWnd {
  pub handle : HWND,
  
}

#[cfg(windows)]
impl LabelWnd {
  pub fn create_window( x:i32, y:i32, width:i32, heigh:i32 ) -> Result<LabelWnd, Error> {
    let handle = ...;
    
    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();    

    if handle.is_null() {
        Err( Error::last_os_error() )
    } else {
        Ok( LabelWnd { 
            handle 
            } )
    }
  }
}