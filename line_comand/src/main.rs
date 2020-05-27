extern crate getopts;
use getopts::Options;
use getopts::Matches;



use std::str;
use std::error::Error;
use std::env;

use std::io::{ BufReader };
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;

extern crate prettytable;
use prettytable::{Table, Row, Cell};
use prettytable::{Attr, color};


fn confirmar_cita(v: &String, indice: u8) -> String {

    let info_cupo = v.clone();    
    let mut bytes = info_cupo.into_bytes();        
    let refbytes  = bytes.as_mut_slice();
    let mut condator_indice = 1;

    for item in 0..refbytes.len() {
        //println!("{}",item);
        if refbytes[item] != 44 {
            if indice == condator_indice{   
                
                if refbytes[item] == 49 {                   
                    println!("YA ESTA OCUPADO ESE HORARIO.\n\n");
                    std::process::exit(1);
                }
                else{
                    refbytes[item] = 49;
                }                
            }
            condator_indice += 1;
        }      
    }    
    let s = match String::from_utf8(bytes){
        Ok(l) => l,
        Err(e) =>panic!("{}",e)
    };    
    s
    
}
fn obtener_indice(param: &Vec<String>) -> u8 {

    let param_copy = param.clone();
    let dia = String::from(&param_copy[1]);
    let hora = String::from(&param_copy[2]);

    let mut index = 0;
    
    if hora == "8-9"   { index = 1;  }
    if hora == "9-10"  { index = 7;  }
    if hora == "10-11" { index = 13; }
    if hora == "11-12" { index = 19; }
    if hora == "12-1"  { index = 25; }

    if dia == "lunes"    { index += 0; }
    if dia == "martes"   { index += 1; }
    if dia == "miercoles"{ index += 2; }
    if dia == "jueves"   { index += 3; }
    if dia == "viernes"  { index += 4; }
    if dia == "sabado"   { index += 5; }

    index
}
fn reservar_cita(fichero: String, parametros: Vec<String>) -> Result<(), Box<dyn Error>>{

    //BUFFER DEL FICHERO MANDADO
    let contenido = OpenOptions::new()
                      .read(true)                      
                      .open(&fichero)?;
                                            
    let mut buf = BufReader::new(&contenido);
    let  buf = buf.fill_buf()?;    
    let line = buf.lines().into_iter().map(|l| {                        
                            l.unwrap()                            
                    });
    //BUFFER DEL FICHERO MANDADO
    let mut otro_contenido = OpenOptions::new()                        
                        .write(true)
                        .open(&fichero)?;
                        
    let mut row_count = 0;
    for item in line {  
                
            if row_count == 2 {                
                let indice = obtener_indice(&parametros);
                let nuevo = confirmar_cita(&item,indice);
                let nuevo_parrafo = format!("{}\n",nuevo);
                otro_contenido.write_all(&nuevo_parrafo.into_bytes())?;
            }
            else{  
                if row_count != 8{
                    let nuevo_parrafo = format!("{}\n",&item);            
                    otro_contenido.write_all(&nuevo_parrafo.into_bytes())?;
                }
                else{
                    let nuevo_parrafo = format!("{}",&item);            
                    otro_contenido.write_all(&nuevo_parrafo.into_bytes())?;
                }
            }            
        row_count += 1;                    
    }
    Ok(())
}
fn get_nombre_fichero(id: &str, matches: &Matches) -> String {

    let v = [id.to_string()];      

    let id_consultorio = match matches.opts_str(&v){
        Some(arg) => arg,
        None => {
            println!("Sin Argumentos\n");
            std::process::exit(1);
        }
    };     
    let id_consultorio = match id_consultorio.parse::<u8>(){
        Ok(id) => id,
        Err(_) => {
            println!("Ingrese un numero!!\n");
            std::process::exit(1);
        }
    };
    let mut nombre_fichero = String::from("consultorio");
        
    match id_consultorio {
        1 =>  nombre_fichero.push('1'),
        2 =>  nombre_fichero.push('2'),
        3 =>  nombre_fichero.push('3'),
        4 =>  nombre_fichero.push('4'),
        5 =>  nombre_fichero.push('5'),
        _ =>  {                        
            println!("No existe ese consultorio!!\n");
            std::process::exit(1);
        }
    }   
    let nombre_fichero = format!("{}.txt",nombre_fichero);
    nombre_fichero
}
fn new_opcion() -> Options {

    let mut opts = Options::new();
    opts.optflag(
                "", 
                "ayuda", 
                "lista de opciones disponibles."
            );
    opts.optflag(
                "c", 
                "consultorio", 
                "lista de consultorios."
            );
    opts.optopt(
                "h", 
                "horario", 
                "horarios de consultorio","n°"
            );
    opts.optopt(
                "r", 
                "", 
                "reserve una cita ejemplo: 1 lunes 8-9",
                "n° dia hora"); 
    return opts  
}
fn generar_tabla(fichero: &str) -> Result<(), Box<dyn Error>> {

    //BUFFER DEL FICHERO MANDADO
    let  contenido = File::open(fichero)?;
    let mut buf = BufReader::new(&contenido);
    let  buf = buf.fill_buf()?;
    let mut line = buf.lines().map(|l| { l.unwrap() }); 
    //CONSTRUCTOR DE LA TABLA DINAMICA
    let mut table = Table::new();

    let titulo = match line.next(){
        Some(title) => title,
        None =>  String::from("Sin Titulo"),
    };
    let row_info = match line.next(){
        Some(col) => col,
        None =>  String::from("Sin Row Info"),
    };
    let row_info: Vec<&str> = row_info.split(',').collect();    
    let max_col = row_info.len();

    let row_active = match line.next(){
        Some(act) => act,
        None => String::from("Sin Row Active"),
    };
    let row_active: Vec<&str> = row_active.split(",").collect();

    //CREACION DE TITULO Y ROW INFO
    table.set_titles(Row::new(vec![
        Cell::new(&titulo)
            .with_hspan(max_col)
            .with_style(Attr::ForegroundColor(color::CYAN)),            
    ]));
    let mut vec_info_row: Vec<Cell> = Vec::new();
    for col_info in row_info {
        let new_col = Cell::new(&col_info)
                    .with_style(Attr::ForegroundColor(color::BLUE));        
        vec_info_row.push(new_col);
    }
    table.add_row(Row::new(vec_info_row));
         
    //CREACION DEL CUERPO DE LA TABLA
    let mut contador_fin = 0;   

    for item in line {
        let mut vec_info_body_row: Vec<Cell> = Vec::new();
        let row_body: Vec<&str> = item.split(',').collect();
        for col_body in row_body {
                if row_active[contador_fin] == "0" {            
                    let new_col = Cell::new(&col_body)
                        .with_style(Attr::ForegroundColor(color::WHITE));        
                    vec_info_body_row.push(new_col);
                }
                else{      
                    let new_col = Cell::new(&col_body)
                        .with_style(Attr::ForegroundColor(color::RED));        
                    vec_info_body_row.push(new_col);
                }            
            contador_fin += 1;    
        }   
        table.add_row(Row::new(vec_info_body_row));     
    }           
    table.printstd();
    Ok(())
}

fn main() {

    let  args: Vec<String> = env::args().collect();    
    let opts = new_opcion(); 
    let matches = match opts.parse(&args[1..]) {
        Ok(m) =>  m,
        Err(_) => {
            println!("\n{}",opts.usage("Aplicacion: Cita de Hospital"));
            println!("Opcion Invalida!!\n");
            std::process::exit(1);
        }
    };    
    if matches.opt_present("ayuda") {                  
        println!("\n{}",opts.usage("Aplicacion: Cita de Hospital"));
    }
    if matches.opt_present("c") {
        if let Err(e) = generar_tabla("info_consultorio.txt"){
            println!("Ocurrio un error: {}", e);     
        }
    }
    if matches.opt_present("r"){            
        let nombre_fichero = get_nombre_fichero("r", &matches);
        let parametros = matches.free.clone();
        if let Err(e) = reservar_cita(nombre_fichero, parametros){
            println!("Ocurrio un error: {}",e);
        }
    }
    if matches.opt_present("h") {         
        let nombre_fichero = get_nombre_fichero("h", &matches);        
        if let Err(e) = generar_tabla(&nombre_fichero){
            println!("Ocurrio un error: {}", e);
        }        
    }
}
