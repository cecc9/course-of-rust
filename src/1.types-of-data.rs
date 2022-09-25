// TIPOS DE DATOS...
// TYPES OF DATA...

fn main_types() {
    // NUMBERS...
    // let number: i32 = 100;
    let mut number_two: String = String::new();
    std::io::stdin().read_line(&mut number_two).unwrap();

    let number_two_int: u16 = number_two.trim().parse().unwrap();
    // let number_u8 = number as u8;
    // let character = number_u8 as char;

    // STRINGS...
    let len: String = String::from("elvis");
    let len_two: &str = "choque";
    // let len_string: String = String::new();

    // println!("number_u8 {}", number_u8);
    // println!("number_u8: {}, character: {}", number_u8, character);
    // println!("{}", number_u8);
    // println!("{}", number as u8 as char);

    println!(
        "cantidad de caracteres de: {} {}",
        len_two,
        len_two.chars().count();
    );

    println!(
        "number_two: {}, bytes: {}",
        number_two.trim(),
        number_two.len()
    );
    println!("number_two_int: {}", number_two_int);

    println!("=========---=========");
    println!("Tamaño de un char: {} bytes", std::mem::size_of::<char>());
    println!("Tamaño de una cadena que contiene la 'ß': {}", "ß".len());
    println!("Tamaño de un string: {}", len.len());
}
