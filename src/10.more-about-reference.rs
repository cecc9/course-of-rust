// MAS SOBRE REFERENCIAS
// MORE ABOUT REFERENCE
fn main() {
    let pais = String::from("Austria");
    let ref_uno = &pais;
    let ref_dos = &pais;

    println!("{:p}", ref_uno);
    println!("{:p}", ref_dos);

    // STRING
    let info_name = return_str();
    println!("{:p}", info_name);

    // let info_country = return_str_two();
    // println!("{}", info_country);

    // NUMBER
    let info_number = return_u8();
    println!("{}", info_number);
}

fn return_str() -> &'static str {
    let name = "elvis";
    let name_reference = &name;
    return name_reference;
}

// fn return_str_two() -> &str {
//     let pais = String::from("Austria");
//     let pais_ref = &pais;
//     pais_ref
// }

fn return_u8() -> u8 {
    let number = 45;
    return number;
}
