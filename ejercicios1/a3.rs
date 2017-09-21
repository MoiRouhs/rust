fn main() {  // Sentencia IF y cenversión de &str <=> String. 

  let texto1: &str = "hola";
  let texto2 = "hola".to_string();
  
  let text1 = &texto1.to_string(); // Conversión de &str a String es inportanteno olvidar el "&"
  let text2: &str = &texto2; // Conversión de String a &str es importante declarar el tipo de enlace a variable y ponerle el "&"
  
  if texto1 == texto2{ // El IF de Rust tiene un comportamiento muy similar al de los lenguajes de tipado dinámico o tipado debil.
    println!("Son iguales {}",text1);
  }else if text1.len() == text2.len() { // .len() sirve para saber el largo de una cadena en este caso.
    println!("Este es el else if de rust");
  }else{
    println!("Son diferente {}",text2);
  };
}