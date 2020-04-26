struct Rectangulo {

    altura: u32,
    anchor: u32

}
impl Rectangulo { 
    
    fn calcular_area(&self) -> u32 {

        self.altura * self.anchor
    }
}

fn main() {
    println!("IMPLEMENTACIONES EN RUST!!!");

    let rect1 = Rectangulo {
        altura: 10,
        anchor: 10        
    };

    let area = rect1.calcular_area();
    println!("EL AREA ES:  {}", area);
}
