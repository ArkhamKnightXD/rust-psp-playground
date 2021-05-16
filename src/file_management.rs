use psp::sys::{IoOpenFlags};
use core::ffi::c_void;

//esto solo puede ser probado en el psp
pub fn file(){

    unsafe {

        //Aqui creamos el archivo en el siguiente directorio, si ya existe no lo crea, sino simplemente lo abre
        let file_data_to_write = psp::sys::sceIoOpen(b"ms0:/PSP/GAME/Rust/test.txt\0".as_ptr(), IoOpenFlags::CREAT|IoOpenFlags::WR_ONLY, 0777 );

        //Aqui escribimos los que deseamos en el archivo para luego cerrar
        //cerrar es necesario, pues sino lo cerramos mas adelante no podremos seguir trabajando con el archivo
        psp::sys::sceIoWrite(file_data_to_write, b"1 2 3! Hello PSP!\n\0".as_ptr() as *const c_void , 19);
        psp::sys::sceIoClose(file_data_to_write);
 
        //Aqui abrimos de nuevo el archivo
        let file_data_to_read = psp::sys::sceIoOpen(b"ms0:/PSP/GAME/Rust/test.txt\0".as_ptr(), IoOpenFlags::RD_ONLY, 0777 );
       
        //aqui creamos un array con un lenght de 19 que almacenara la data del archivo
        let array: [u8; 19] = [0; 19];
 
        //Aqui leemos los datos del archivo y lo guardamos en el array ya creado
        //y finalmente cerramos el archivo
        psp::sys::sceIoRead(file_data_to_write, array.as_ptr() as *mut c_void, 19);
        psp::sys::sceIoClose(file_data_to_read);
 
        //Aqui imprimiremos los datos del archivo
        if let Ok(file_data) = core::str::from_utf8(&array[0..]) {

            psp::dprintln!("{}", file_data);
        }
    }
}