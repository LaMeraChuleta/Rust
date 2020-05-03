
enum Message {

    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self){

        print!("Metodo Call");
        
    }
}



fn main() {
    println!("Enumeraciones Rust");

    let m = Message::Write(String::from("Hola Mundo Enumeracion!!!"));
    m.call();


}
