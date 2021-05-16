use psp::sys;
use psp::dprintln;

//Defino los elementos necesarios para utilizar los controles
//Esto me fallaba porque no utilizaba sys, los tutoriales estan desactualizados
use psp::sys::{SceCtrlData, CtrlButtons, CtrlMode};

pub fn input(){

    //uso el modo unsafe ya que muchos de las llamadas 
    //y metodo que deseamos hacer con el psp son unsafe
    unsafe{

        sys::sceCtrlSetSamplingCycle(0);

        //defino el modo de control, se recomienda siempre analogo
        sys::sceCtrlSetSamplingMode(CtrlMode::Analog);

        //guardamos los datos del control
        let ctrl_data = &mut SceCtrlData::default();

        loop {

             sys::sceCtrlReadBufferPositive(ctrl_data, 1);

             if ctrl_data.buttons.contains(CtrlButtons::CROSS) {

                 dprintln!("X Pressed!");
             }
        }
    }
}