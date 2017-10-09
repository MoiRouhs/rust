use std::collections::HashMap; // Hashmap, Se necesita esta librria para que funcione los hashmap

fn main() { 

  let mut contacts = HashMap::new(); // Así se crea un hashmap vacio, por el momento no se ocmo crearlo con elemento predeterminados.
    contacts.insert("Daniel", 7981364); 
    contacts.insert("Ashley", 6457689); // El metodo "insert" es utilizado para agregar nuevas entradas al hashmap
    contacts.insert("Katie", 4358291);
    contacts.insert("Robert", 9561745); // La llave y su respectivo valor se separan por una coma.
    
  println!("{:?}, {}", contacts["Daniel"], contacts.len()); // Para consultar un valos se puede hace mediante corchetes cuadrados y la llave, también se puede ver su largo con len()

}
