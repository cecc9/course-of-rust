// IMPRIMIENDO TEXTO EN LA CONSOLA...
// TO PRINT TEXT TO THE CONSOLE...
fn main() {
    // si queremos declarar una variable de tipo char usamos comillas simples('') ...
    // si queremos declarar una variable de tipo string usamos comillas dobles("")...

    println!("to print text string: {}", "hola");
    println!("to print text char: {}", 'c');

    // => FUNCTIONS -> FUNCIONES
    println!("calling the number function: {}", number());
    println!("calling the number_two function: {}", number_two());

    subtraction(48, 23);

    let result_multiplication = multiplication(4, 7);
    println!("result multiplication {}", result_multiplication);

    // => BLOQUES DE COGIDO...

    let mi_numero = {
        let segundo_num = 8;
        segundo_num + 9 // sin punto y coma, por lo que el
                        // bloque de código devuelve 8 + 9 = 17
    };
    println!("¡Hola, mundo número {}!", mi_numero);

    let mi_numero_two = {
        let segundo_num = 8; // declara el segundo número
        segundo_num + 9; // suma 9 con el segundo número
                         // pero no se devuelve
                         // segundo_num desaparece aquí
    };
    println!("¡Hola, mundo número {:?}!", mi_numero_two); // mi_numero_two es ()
}

fn number() -> u16 {
    // si vamos a retornar un valor ponemos [-> (y el tipo de dato)]
    4 // si el numero que contiene esta funcion no tiene punto y coma, se devuelve este valor...
      // 5; si tiene un punto y coma no devuelve nada, (devolveria  parentesis())...
}
fn number_two() -> u8 {
    return 45; // tambien podemos utilizar el return para devolver un dato...
}
fn subtraction(num_one: i8, num_two: i8) {
    let result = num_one - num_two;
    println!("=== subtraction ===");
    println!("the result is: {}", result); // podemos declarar una nueva variable para la suma e imprimirlo...
    println!("{}", num_one - num_two); // o tambien podemos imprimirlo directamente...
}
fn multiplication(number_one: u8, number_two: u8) -> u8 {
    // number_one * number_two
    return number_one * number_two;
}
