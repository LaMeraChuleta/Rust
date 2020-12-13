use std::collections::HashMap;
use std::collections::hash_map::RandomState;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Disco {

    nombre: String,
    artista: String,
    year: i32
}

impl  Disco {
    
    fn new(nombre: &str, artista: &str, year: i32) -> Disco {

        Disco{
            nombre: nombre.to_string(),
            artista: artista.to_string(),
            year: year
        }
    }
}

fn main() {

    println!("APRENDIENDO HASHMAP");
    
    let s = RandomState::new();
    let c = RandomState::new();
    //Declaramos el hashmap
    let mut discos_info: HashMap<Disco, i32> = HashMap::new();
    //Declaracion con capacidad
    let mut discos_info: HashMap<Disco, i32> = HashMap::with_capacity(2);
    //Declaracion random
    let mut discos_info: HashMap<Disco, i32> = HashMap::with_hasher(s);
    //Declaracion radom y capacidad
    let mut discos_info: HashMap<Disco, i32> = HashMap::with_capacity_and_hasher(2,c);
        
    //Inicializacion metodo insert
    discos_info.insert(Disco::new("Monotron", "King Gizzard", 2002), 1);
    discos_info.insert(Disco::new("El diablito", "Caifanes", 1998), 2);
        
    for (disco, key) in &discos_info{

        println!("{:?} tiene el id {}", disco, key);                
    }
    //Inicializacion con un arrat -> tuplas
    let saludos: HashMap<&str, i32> = [("hola",1), ("hello",2)].iter().cloned().collect();

    for (saludo, key) in &saludos {

        println!("{} con id ---> {}", saludo, key);
    }    
    let asher: &RandomState = discos_info.hasher();
    println!("{:?}", asher);
}
