fn main() {

    let str1 = String::from("Hola, Mundo!!!");
    let len = calcula_len(&str1);
    println!("LA CADENA ES: {} Y SU TAMAÑO ES: {}", str1, len);


    {
        let ref1 = &str1;
        println!("Referencia a una variable en ambito diferente:  {}", ref1);
    }

    let ref2 = &str1;
    println!("Referencai fuera del ambito local: {}", ref2);


    let mut str2 = String::from("Adios, Mundo!!!");

    let refmut1 = &str2;
    let refmut2 = &str2;    


    println!("LA CADENA MUT REFMUT1 ES: {}  REFMUT2 {}", refmut1, refmut2);

    let refmut3 = &mut str2;

    refmut3.push_str(" Cambiamos el String");

    println!("REFERENCIA MUT:  {}", refmut3);

    println!("{}", str2);


    
}

fn calcula_len(str1: &String) -> usize {

    str1.len()
}
