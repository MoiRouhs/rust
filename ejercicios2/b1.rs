fn main() { // Funciones 

  hola();
//  parametros_entrada(64);  
//  println!("{}",parametros_io(8));
}

fn hola(){ // Función sin parametros y no devuelve nada.
  println!("Hola, vuelve al script\ndescomenta la linea bajo esta\nCompila y ejecuta.");
}

fn parametros_entrada(x: i64 ){ // Los argumentos de las funciones deben ser de un tipo específico.
  let y: i64 = x + x;
  println!("X es {} y el doble es {}", x, y);    
}

fn parametros_io(x: i64) -> String{ // Así como se esperamos argumentos de tipos específico, también es necesario específicar el de el elemento que se retorna.
  if x >= 5 {
    "Es mayor o igual a 5, ahora intenta compilar con uno menor.".to_string() // No es necesario aclarar el elemento a devolver rust lo intenta deducir.
  }else{
    return "Es menor a 5".to_string(); // Si no es facíl de deducir o desea específicarlo lo puede hacer.
  }
}