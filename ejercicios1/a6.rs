fn main() {  // Ciclos while, for, Enumerate
  
let mut x = 5;
let mut completado = false; // La variable mutable completado es de tipo booleano, es decir que solo tiene dos valores: false o true.

println!("Ciclo while");
  while !completado { // El ciclo while termina cuando la sentencia se cumpla en este caso cuando completado sea diferente a false, es decir verdadero.
    x += x;
      println!("{}", x);
    if x == 160 {
        completado = true;
    };
  };

// El ciclo for de rust luce mas parecido al de ruby.
println!("Ciclo for");
  for i in 0..6 { // Este ciclo empiza por el primer número y termina uno antes del indicado, para este caso empieza en 0 y termina en 5
    println!("{}", i);
  };
  
println!("Ciclo for con enumerate()");    
  for (i,j) in (5..11).enumerate() { // enumerate cuenta las veces que se iterao se hace el ciclo, es importante respetar los parentensis.
    println!("i = {} y j = {}", i, j); // i imprime el número de iteración y j el rango en el que se esta iterando.
  };
}