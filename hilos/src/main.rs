
fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.7:5000").expect("No se creo el servidor");
    std::thread::spawn(move || loop {
        match listener.accept() {
            Ok((_socket, addr)) => println!("Nuevo Cliente: {:?}", addr),
            Err(e) => println!("No se obtuvo el cliente: {:?}", e),
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    });
    let cliente = std::net::TcpStream::connect("127.0.0.7:5000").expect("No se conecto al servidor"); 
    loop {           
        println!("Me conecte");
        std::thread::sleep(std::time::Duration::from_secs(10));
        cliente.shutdown(std::net::Shutdown::Both).expect("No se pudo desconectar");
        println!("Me desconecte");        
        std::thread::sleep(std::time::Duration::from_secs(3));    
    }
}