use std::{ thread, time };

fn main() {

    println!("Hola, mundo!");
    let segundos = time::Duration::from_millis(500);
    println!("...");
    thread::sleep(segundos);
    println!("Adios Mundo");

    let caja = Box::new(10);
    println!("caja = {}", caja);    

    
}
