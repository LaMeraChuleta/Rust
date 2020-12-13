
fn main() {
    
    let mut x: f64 = 1.2343f64;
    let mut y: f64 = 1.001f64;
    
    std::mem::swap(&mut x, &mut y);

    println!("{} {}", x, y);
}
