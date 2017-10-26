use std::env; // Libreria que se necesira para los argumentos en linea de comando

fn main() {
    let args: Vec<_> = env::args().collect(); // Los argumentos se guardan en un vector.
    if args.len() > 1 {
        println!("Los argumentos son: {:?}", args); // Imprimer el vertor de los argumentos.
    }else{
        println!("No se envio argumento.") // Mensaje si no se envia argumentos.
    }
}
