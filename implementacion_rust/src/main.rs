struct Rectangulo {

    altura: u32,
    anchor: u32

}
impl Rectangulo { 
    
    fn calcular_area(&self) -> u32 {

        self.altura * self.anchor
    }
    fn rectangulo_mayor(&self, otro: &Rectangulo) -> bool{

        self.altura >= otro.altura && self.anchor >= otro.anchor
    }
}

fn main() {
    println!("IMPLEMENTACIONES EN RUST!!!");
    let rect1 = Rectangulo {

        altura: 10,
        anchor: 10        
    };
    let rect2 = Rectangulo {

        ..rect1
    };
    let rect3 = Rectangulo {

        altura: 25,
        anchor: 25
    };

    let area = rect1.calcular_area();
    println!("EL AREA ES:  {}", area);
    println!("El rectangulo 1 es mayor al rec2 {}", rect1.rectangulo_mayor(&rect2));
    println!("El rectangulo 1 es mayor al rec3 {}", rect1.rectangulo_mayor(&rect3));
}
