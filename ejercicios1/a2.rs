fn main() {  // En rust tenemos dos tipos de cadena de texto &str y String.

  let texto1 = "mundo"; // Enlace a variable del tipo "&str"
  println!("{}",texto1);  // Forma basica de impresión en consola.
  
  let mut texto2 = "hola ".to_string(); // Enlace a variable mutable (que se puede sobreescribir) de tipo String se declara con "to_string()"
  texto2 = texto2 + texto1; // Concadenacion y sobreescritura de dos enlaces a variables, en el caso de cadenas de texto una debe ser de tipo String.
  println!("{}",texto2); 
  
  // En rust los enlaces a variables por defecto no pueden ser sobreescritos, para poderlos sobre escribir es necesario especificarlos con "mut"

}