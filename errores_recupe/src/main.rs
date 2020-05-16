use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs;

fn leer_nombre_fichero_more_reduce_more() -> Result<String, io::Error>{
    fs::read_to_string("hello.txt")
}

fn leer_nombre_fichero_more_reduce() -> Result<String, io::Error>{

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn leer_nombre_fichero_reduce() -> Result<String, io::Error>{

    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn leer_nombre_fichero() -> Result<String, io::Error> {

    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e), 
    }
}


fn main() {
    println!("Errores Recuperables con Result<T, E>");

    let result = leer_nombre_fichero();

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
            Ok(fc) => fc,
            Err(e) => panic!("Problema creando el fichero {:?}", e),
        },
        other_error => {
            panic!("Problema opening the file: {:?}", other_error)
        }
    },


    };

    let g = File::open("world.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("world.txt").unwrap_or_else(|error|{
                panic!("Problema al crear el fichero {:?}", error);
            })
        }
        else{
            panic!("Problema abriendo el fichero {:?}", error);
        }

    });

    
}


 