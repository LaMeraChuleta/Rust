struct User {

    username: String,
    email: String,
    registrado: bool,
    id: i32
}


//ESTRUCTURA TUPLA
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {

    println!("ESTRUCTURAS EN RUST!!!");

    let mut user1 = User {
        id: 1,
        username: String::from("LaMeraChuleta"),
        email: String::from("geoffreyytorres25@gmail.com"),
        registrado: true
    };

    println!("{}", user1.email);

    user1.email = String::from("othermail@gmail.com");

    println!("{}", user1.email);

    let user2 = User {
        ..user1
    };

    //let user2 = nuevo_user("Paola Itzel", "paoitzy25@gmail.com");
    println!("{}", user2.username);
    println!("{}", user2.email);

    let color1 = Color(1,0,1);
    let origin = Point(0,1,0);

    println!("DATOS DE LA TUPLA COLOR: {}  {}  {}", color1.0, color1.1, color1.2);    

    let name = String::from("Juanito Alimaña");
    let mail = String::from("lalo98.0@gmail.com");

    let user3 = nuevo_user(name, mail);

    println!("{}", user3.username);
    println!("{}", user3.email);
    



}

fn nuevo_user(username: String, email: String) -> User {

    User {
        username,
        email,
        id: 2,
        registrado: false
    }

}

