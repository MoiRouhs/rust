use std::process::Command; // Libreria utilizada para los comandos de terminal.
fn main() {
    let output = Command::new("ls").arg("-a").output().unwrap_or_else(|e| { //Codigo para ejecutar el comando en la tarminal cons u argumento
        panic!("failed to execute process: {}", e)
    });
    println!("{}", String::from_utf8_lossy(&output.stdout)); // codigicacion en utf-8 de la salida de la consola.
}
 