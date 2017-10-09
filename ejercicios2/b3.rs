use std::collections::HashMap; // Hashmap, Se necesita esta librería para que funcione los hashmap

fn main() { 

  let mut contactos = HashMap::new(); // Así se crea un hashmap vacio.
    contactos.insert("Daniel", 7981364); 
    contactos.insert("Ashley", 6457689); // El metodo "insert" es utilizado para agregar nuevas entradas al hashmap
    contactos.insert("Katie", 4358291);
    contactos.insert("Robert", 9561745); // La llave y su respectivo valor se separan por una coma.

  let timber_resources: HashMap<&str, i32> = // Otra forma de declara un hashmap
    [("Norway", 100),
     ("Denmark", 50),
     ("Iceland", 10)]
     .iter().cloned().collect();   
  
  println!("Cantidad de contactos: {}, Número de Daniel:{}", contactos.len(), contactos["Daniel"]); // Para consultar un valos se puede hace mediante corchetes cuadrados y la llave, también se puede ver su largo con len()

 for contacto in timber_resources{ // Itera el hashmap en forma de tupla ("a", "b") 
   println!("{:?}", contacto); // Imprime la llave y su valor en forma  de tupla
 };
 
 for (contacto, numero) in contactos.iter(){ // iter() nos permite diferenciar entre la llave y la entrada para iterar por ellos.
   println!("Nombre: {}, número: {}",contacto,numero); // Imprime contacto que es la llave del hashmap y numero que es el valor.
 };
 
}
