use psp::sys;
use psp::dprintln;

//Defino los elementos necesarios para utilizar los controles
//Esto me fallaba porque no utilizaba sys, los tutoriales estan desactualizados
use psp::sys::{SceCtrlData, CtrlButtons, CtrlMode};

pub fn input(){

   // the code is unsafe due to the fact that it calls into unsafe code within the PSP OS and driver modules. 
    unsafe{

        //To detect input we first need to set up the sceCtrl module's sampling cycle
        // (when it checks for input) via sceCtrlSetSamplingCycle(0) where 0 is the default argument
        sys::sceCtrlSetSamplingCycle(0);

        //defino el modo de control para el analog stick, se recomienda siempre analogo
        sys::sceCtrlSetSamplingMode(CtrlMode::Analog);

        //Creamos el control data donde guardaremos los datos del control
        //este almacenara la struct que contiene los datos del control
        let ctrl_data = &mut SceCtrlData::default();

        loop {

            //Aqui leemos los datos del control mediante sceCtrlReadBufferPositive
            //y guardamos en la variable que contiene la struct de ctrl data
             sys::sceCtrlReadBufferPositive(ctrl_data, 1);

             //Finalmente comparamos que boton se presion utilizando el
             //enum de ctrlbuttons

             //intente crear una funcion aparte utilizando match para simplificar codigo
             //pero no encontre forma de hacerla funcionar, ni tampoco llevando los ifs
             //a una funcion para simplificar codigo

             
             if ctrl_data.buttons.contains(CtrlButtons::CROSS) {

                 dprintln!("X Pressed!");
             }

             if ctrl_data.buttons.contains(CtrlButtons::CIRCLE) {

                dprintln!("O Pressed!");
            }

            if ctrl_data.buttons.contains(CtrlButtons::SQUARE) {

                dprintln!("SQUARE Pressed!");
            }

            if ctrl_data.buttons.contains(CtrlButtons::TRIANGLE) {

                dprintln!("TRIANGLE Pressed!");
            }
        }
    }
}