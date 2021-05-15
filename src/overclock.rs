//definire estos elementos para evitar tener que escribir la libreria psp siempre
use psp::sys;
use psp::dprintln;

//funcion encargada de cambiar la velocidad del psp
pub fn change_psp_clock_speed() {

    //El unsafe es necesario para trabajar con la frecuencia del procesador del psp
    unsafe {
        
        //almaceno los valores del cpu en dos variables
        let cpu = sys::scePowerGetCpuClockFrequency();
        let bus = sys::scePowerGetBusClockFrequency();
        
    
        dprintln!("PSP is operating at {}/{}MHz", cpu, bus);
        dprintln!("Setting clock speed to maximum...");
    
        sys::scePowerSetClockFrequency(333, 333, 166);
    
        let cpu = sys::scePowerGetCpuClockFrequency();
        let bus = sys::scePowerGetBusClockFrequency();
    
        dprintln!("PSP is now operating at {}/{}MHz", cpu, bus);
    }
}