#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("DATOS DEL RECTANGULO {:?}", rect1);
    println!("EL AREA DEL RECTANGULO ES: {}", calcula_area_rectangulo(&rect1));

}

fn calcula_area_rectangulo(datos: &Rectangle) -> u32 {

    datos.height * datos.width
}