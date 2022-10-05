// CONSTANTES Y ESTATICOS
// CONST AND STATIC

const NUMBER: u8 = 45;
const VARIABLE_STRING: &str = "Hola Mundo";

static NUMBER_TWO: u8 = 34;
static DAYS: [&str; 4] = ["lunes", "martes", "miercoles", "jueves"];
static NUMBERS: [u8; 3] = [1, 2, 3];
fn main() {
    // Además de let, existen dos maneras más de declarar valores. const y static

    // const: se utiliza para los valores que no cambian
    // NORMALMENTE SE SUELEN ESCRIBIR FUERA DEL MAIN, PARA QUE EXISTAN EN TODO EL PROGRAMA
    // Y POR CONVENCION SE SUELEN ESCRIBIR EN MAYUSCULAS...

    println!("{}", NUMBER);
    println!("{}", VARIABLE_STRING);

    // static: define una posición fija en memoria que puede actuar como una variable global
    // TAMBIEN SE SUELEN ESCRIBIR FUERA DEL MAIN...

    println!("{}", NUMBER_TWO);
    println!("{:?}", DAYS);
    println!("{}", DAYS[0]);

    println!("{:?}", NUMBERS);
}
