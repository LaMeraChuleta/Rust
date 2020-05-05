use cajas_modulos::operaciones::suma_operacion::suma_decimal;

enum varios_tipos_vector {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {

    println!("Funcion Main");
    suma_decimal(23.50);

    let mut v: Vec<i32> = Vec::new();
    let mut vec_inicializado = vec![1,2,3,4,5,6];

    //Metodo Push
    v.push(3);
    v.push(2);
    v.push(1);

    let third: &i32 = &vec_inicializado[2];
    println!("EL VALOR DE LA POSISCION 2 ES: {}", third);

    match vec_inicializado.get(2) {
        Some(expr) => println!("el numero es: {}", expr),
        None => println!("Sin elemento!!"),
    }

    for i in &vec_inicializado {

        println!("{}", i);
    }

    for i in &mut vec_inicializado {

        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        varios_tipos_vector::Int(1),
        varios_tipos_vector::Text(String::from("Blue one")),
        varios_tipos_vector::Float(59.99),        
    ];

    let vector_ceros = vec![0;5];

    for itme in &vector_ceros {

        println!("{}",itme);
    }

    let mut vec1 = Vec::with_capacity(1);


    vec1.resize(2, 69);

    for item in &vec1 {

        println!("{}", item);
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // Prints 3, 2, 1
        println!("{}", top);
    }

    println!("Tamaño del vec1: {}", vec1.len());

    let mut vec: Vec<i32> = Vec::with_capacity(10);

    

    for i in 0..10 {
        vec.push(i);
    }
    
    // ...but this may make the vector reallocate
    vec.push(11);


    for element in &vec {

        println!("Otro Vector {}", element);
    }


    let mut vec_tod = vec![1];
    vec_tod.reserve(10);
    println!("CApacidadd tod: {}", vec_tod.capacity());

    for i in &vec_tod {

        println!("{}", i);
    }

    vec_tod.shrink_to_fit();
    println!("CApacidadd tod: {}", vec_tod.capacity());


}