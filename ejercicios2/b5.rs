use std::io::{stdin,stdout,Write}; // Libreria para las entradas de teclado
use std::string::String; // Libreria para manejo de cadenas de texto.

fn main() {
    let mut s=String::new();
    print!("Entre su texto: ");
    let _=stdout().flush(); // Permite que el ingreso del texto sea despues del mensaje impreso.
    stdin().read_line(&mut s).expect("No entro un texto correcto"); // Captura la entrada en la variable s.
    s = s.to_uppercase(); // Pasa la cadena de texto a mayusculas.
    println!("Su texto es: {}",s); // Imprimer la captura en mayusculas.
}