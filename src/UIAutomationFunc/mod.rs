use winapi-i686-pc-windows-gnu;


#[derive(Debug)]
#[cfg(windows)]
pub struct UIAutomationFunc {
  //pub handle : HWND,
  //_text: Vec<u16>,
}

#[cfg(windows)]
impl UIAutomationFunc {
  pub fn init(pAutomation: &IUIAutomation) -> bool {

  }
}