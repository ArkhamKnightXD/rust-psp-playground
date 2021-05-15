use psp::sys;
use psp::dprintln;

use psp::{SceCtrlData, CtrlButtons, CtrlMode};

pub fn input(){

    unsafe{

        sys::sceCtrlSetSamplingCycle(0);
        sys::sceCtrlSetSamplingMode(CtrlMode::Analog);

        let ctrl_data = &mut SceCtrlData::default();

        loop {

             sys::sceCtrlReadBufferPositive(ctrl_data, 1);

             if ctrl_data.buttons.contains(CtrlButtons::Cross) {

                 dprintln!("X Pressed!");
             }
        }
    }
}