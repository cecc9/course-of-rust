// VISUALIZACION Y DEPURACION

fn main() {
    // std => libreria estandar de RUST
    let no_number = ();

    println!("{:?}", no_number); // Para la impresion en depuracion (para devs)...
    println!("{:#?}", no_number); // Tambien se puede usar esto... (conocida como impresion atractiva)

    // => NUMERO MENOR Y NUMERO MAYOR...

    println!(
        "El menor i8 es {} y el mayor i8 es {}.",
        std::i8::MIN,
        std::i8::MAX
    );
    println!(
        "El menor u8 es {} y el mayor u8 es {}.",
        std::u8::MIN,
        std::u8::MAX
    );
    println!(
        "El menor i16 es {} y el mayor i16 es {}.",
        std::i16::MIN,
        std::i16::MAX
    );
    println!(
        "El menor u16 es {} y el mayor u16 es {}.",
        std::u16::MIN,
        std::u16::MAX
    );
}
