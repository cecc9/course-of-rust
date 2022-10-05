// CADENAS DE CARACTERES
// STRINGS OF CHARACTERS
fn main() {
    // Rust tiene dos tipos de cadenas de caracteres: String y &str
    // pero cual es la diferencia ?

    // &str es una cadena de caracteres simple que reside en la pila.
    // Cuando se escribe let mi_variable = "¡Hola, mundo!" se crea una &str. Este tipo es muy rápido.
    let my_variable = "Hola Mundo";
    println!("{}", my_variable);
    // cuando creo una nueva variable por defecto(default) es de tipo &str

    // String es un tipo de dato más complejo. Es un poco más lento, pero tiene más funciones.
    // Un String es un puntero que almacena los datos en la memoria dinámica.
    let my_variable_two = String::from("Hola Mundo 2");
    println!("{}", my_variable_two);
    // cuando creo una variable string mas compleja es de tipo String

    let emoji = "😂";
    println!("Mi emoji real es {} ", emoji);

    // -> TAMAÑOS DE LOS TIPOS DE DATOS...
    println!(
        "Un String siempre ocupa {:?} bytes. Es de tamaño fijo.",
        std::mem::size_of::<String>()
    );
    println!(
        "Y un i8 siempre ocupa {:?} bytes. Es de tamaño fijo.",
        std::mem::size_of::<i8>()
    );
    println!(
        "Y un f64 siempre ocupa {:?} bytes. Es de tamaño fijo.",
        std::mem::size_of::<f64>()
    );

    let variable = "hola";
    println!(
        "¿Y un &str? Puede ocupar cualquier tamaño. variable ocupa {:?} bytes. No es de tamaño fijo.",
        std::mem::size_of_val(variable)
    );

    let number = 34;
    let number_string = number.to_string();
    println!("{}", number_string);
    // to_string() convierte cualquier cosa en string....
}
