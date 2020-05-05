
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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
#[derive(Debug)]
enum Coin {

    Penny,
    Nickel,
    Dime,
    Queater(UsState),
}


fn value_in_cents(coin: Coin) -> u8{

    match coin {
        
        Coin::Penny => 1,        
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Queater(state) => {
            println!("State Cetavo de: {:?}", state);
            25
        },
    }
}

fn plus_one(x: std::option::Option<i32>) -> std::option::Option<i32> {

    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}




fn main() {
    println!("Enumeraciones Rust");

    let m = Message::Write(String::from("Hola Mundo Enumeracion!!!"));
    

    let x: std::option::Option<u32> = Some(2);
    println!("{}", x.is_some());
    println!("{}", x.is_none());

    value_in_cents(Coin::Queater(UsState::Alaska));


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);


}
