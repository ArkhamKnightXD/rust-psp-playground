
//Necesario para trabajar con el tiempo
use core::mem::MaybeUninit;

pub fn count_time(){

    unsafe {

        let mut tick = 0;
        psp::sys::sceRtcGetCurrentTick(&mut tick);

        // Convert the tick to an instance of `ScePspDateTime`
        let mut date = MaybeUninit::uninit();
        
        psp::sys::sceRtcSetTick(date.as_mut_ptr(), &tick);
        let date = date.assume_init();

        psp::dprintln!(
            "Current time is {:02}:{:02}:{:02} UTC",
            date.hour,
            date.minutes,
            date.seconds
        );
    }
}