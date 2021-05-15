//necesario a la hora de utilizar std con rust psp pues esta limitada ahora mismo
//falla pero dejare el codigo por aqui para cuando logren implementar el std con rust psp
#![feature(restricted_std)]

use std::string::String;


pub fn std_testing(){

    //Aqui creo un string utilizando la libreria standard
    let yeet = String::from("Yeeteth! I am inside a String!");

    psp::dprintln!("{}", yeet);

    let people = vec!["Karvin", "Samuel", "El pepe"];

    for person in people {

        let message = format!(
            "Hello, {}! I'm coming to you live from the standard library!\n",
            person
        );

        psp::dprint!("{}", message);
    }

}