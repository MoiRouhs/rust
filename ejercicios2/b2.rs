fn main() { // Apuntadores a funciones.

  let f: fn(i32) -> i32 = uno_mas; // Apuntando a una función sin inferencia de tipo.
  let k = uno_mas; // Apuntando a una función con inferencia de tipo.
  let resultado_f = f(4); // Haciendo uso del apuntador para correr la función
  let resultado_k = k(5); // Haciendo uso del apuntador para correr la función
  
  println!("Funciónpuntador de función f:{}, k:{}", resultado_f, resultado_k);
}

fn uno_mas(x: i32) -> i32{
  x + 1
}