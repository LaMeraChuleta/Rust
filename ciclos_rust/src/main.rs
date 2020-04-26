fn main() {
    println!("Ciclos en Rust");

    println!("LOOP");
    ciclo_loop();

    println!("WHILE");
    ciclo_while();

    println!("FOR");
    ciclo_for();

    
}

fn ciclo_loop(){


    
    let mut i = 0;

    let result = loop {
      
        i += 1;

        if i == 10 {

            break i * 3;
        }
    };

    println!("{}", result);
}

fn ciclo_while(){

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LISTA_OFF!!!");
}

fn ciclo_for(){

    for number in (1..11).rev() {
        
        println!("{}!", number);

    }

    println!("LISTA_OFF!!!");

}
