// PRINTS
// IMPRESIONES
fn main() {
    // Si se añade \n se imprimirá una nueva línea. Si se añade \t se insertará un tabulador:
    print!("\tComienza con un tabulador\ny salta a una nueva línea");

    // A veces se necesitan muchos " y caracteres de escape en el texto,
    // por lo que Rust proporciona un método más simple para ignorarlos:
    // se añade r# al comienzo y # al final de la cadena de caracteres.
    println!(
        r#"Él dijo, "Puedes encontrar el fichero en c:\files\my_documents\file.txt". Y así fue como lo encontré."#
    );

    // si se dispone de una referencia se puede usar {:p} para imprimir la dirección del puntero.
    // Es decir, el lugar de la memoria del ordenador a la que apunta la referencia.
    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);

    // Los valores numéricos se pueden imprimir en binario, hexadecimal u octal:
    let number = 555;
    println!(
        "Binario: {:b}, hexadecimal: {:x}, octal: {:o}",
        number, number, number
    );

    // También se pueden añadir números entre las llaves para indicar qué variable utilizar,
    // teniendo en cuenta que la primera tiene como índice el 0, la segunda el 1 y así sucesivamente.
    let nombre = "Víctor";
    let apellido = "González";
    println!("Mi Nombre es: {0} {1},", nombre, apellido);

    println!(
        "name: {name} first_name: {first_name} name again: {name}",
        name = "Elvis",
        first_name = "Choque"
    )
}
