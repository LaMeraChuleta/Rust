fn main() {
    println!("Estructura de Datos 'String'");

    let mut cadena = String::new();

    let hello = String::from("السلام عليكم");

    println!("{}", hello);

    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }


 
}
