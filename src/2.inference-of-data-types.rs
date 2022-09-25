// INFERENCIA DE TIPOS DE DATOS
// INFERENCE OF DATA TYPES
fn main() {
    println!("Numbers integers");

    // __La inferencia de tipos de datos consiste en asignarle un tipo cuando declaremos una variable..

    let number: u8 = 34;
    // en este caso asignamos el tipo de dato u8;

    let number_two = 12;
    // si no le asignamos un tipo de dato, el compilador elige por defecto(default) el i32..

    let number_three = 23u8;
    // tambien le podemos asignar un tipo de dato, despues del numero declarado...

    let number_four = 14_u16;
    // para mas claridad, podemos poner un guion bajo, esto no afecta al numero declarado...

    println!("{}", number);
    println!("{}", number_two);
    println!("{}", number_three);
    println!("{}", number_four);

    // NUMEROS DECIMALES..
    println!("=== Numbers decimales ===");

    // si no le asignamos un tipo de dato el compilador elegi por defecto(default) el f64 (f de float)...
    let number_decimal = 124.2;

    let number_decimal_two: f32 = 23.4;
    let number_decimal_three: f64 = 46.5;

    // si queremos realizar una suma de dos numeros decimales, siempre tienen que ser el mismo tipo de dato...
    let number_decimal_addition = number_decimal_two + number_decimal_three as f32;

    println!("{}", number_decimal);
    println!("{}", number_decimal_two);
    println!("addition: {}", number_decimal_addition);
}
