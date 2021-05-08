
//funcion encargada de cambiar la velocidad del psp
pub fn change_psp_clock_speed() {


    //El unsafe es necesario para trabajar con la frecuencia del procesador del psp
    unsafe {
        
        //almaceno los valores del cpu en dos variables, rust al igual que javascript no es tipado
        let cpu = psp::sys::scePowerGetCpuClockFrequency();
        let bus = psp::sys::scePowerGetBusClockFrequency();
    
        psp::dprintln!("PSP is operating at {}/{}MHz", cpu, bus);
        psp::dprintln!("Setting clock speed to maximum...");
    
        psp::sys::scePowerSetClockFrequency(333, 333, 166);
    
        let cpu = psp::sys::scePowerGetCpuClockFrequency();
        let bus = psp::sys::scePowerGetBusClockFrequency();
    
        psp::dprintln!("PSP is now operating at {}/{}MHz", cpu, bus);
    }
}