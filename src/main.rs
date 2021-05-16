//Playground de varias funciones simples de psp en rust
// No incluiremos librerias standard ni funcion main, sino psp_main
#![no_std]
#![no_main]

//De esta forma importo un archivo a mi main
mod overclock;
mod time;
//mod std_print_test;
mod control;
use psp::dprintln;
use psp::enable_home_button;

//Esta es la description para el OS del psp, esto seria el nombre y version de nuestra app
psp::module!("tutorial-hello", 1,0);

//De esta form se escribe una funcion publica en rust
pub fn psp_main(){
    //Activo el home button del psp los :: equivalen a un . en otros lenguajes
    //Esto es necesario en todos los proyectos
    enable_home_button();


    dprintln!("Testing");
    dprintln!("");

    //De esta forma llamo la funcion que esta en mi archivo overclock
    overclock::change_psp_clock_speed();

    dprintln!("");

    time::count_time();

    dprintln!("");

   // std_print_test::std_testing();

   for number in 1..10 {
       
        //si el numero es par imprimelo
        if number % 2 == 0 {

            dprintln!("number: {}", number);
        }
   }

    control::input();
}