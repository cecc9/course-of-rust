// MUTABILIDAD y BLOQUEO
// MUTABILITY AND SHADOWING
fn main() {
    // cuando declaramos una variable por defecto es inmutable...
    let variable = 34;
    // si queremos cambiar su valor, nos dara error...
    // variable = 23;
    println!("{}", variable);

    // para convertir una variable a mutable, agregamos mut...
    let mut variable_two = 23;
    println!("variable_two: {}", variable_two);

    variable_two = 12;
    println!("variable_two: {}", variable_two);

    // PERO OJO: No se le puede cambiar el tipo de dato, como por ejemplo de number a string...
    let mut variable_three = 21;
    println!("variable_three: {}", variable_three);

    // variable_three = "hola";
    // println!("variable_three {}:" variable_three)
    // HACER ESTO ES UN ERROR...

    // => SHADOWING
    let number = 6;
    let cadena = "elvis";
    println!("number {}", number);
    println!("cadena {:p}", cadena);

    // Si declaramos una variable igual, estamos aplicando el shadowing (ocultar variable) (Bloquear variable)
    // Pero no deja de existir, como vemos en el ejemplo anterior estan en diferentes
    // referencias de memoria...

    let number = 5.6;
    let cadena = "cristian";
    println!("number {}:", number);
    println!("cadena {:p}:", cadena);
}
