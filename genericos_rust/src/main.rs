fn mas_grande<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largo = list[0];

    for &item in list {
        if item > largo {
            largo = item
        }
    }
    return largo;
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn mezclado<V, W>(self, otro: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: otro.y,
        }
    }
}

fn main() {
    println!("Genericos en Rust");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = mas_grande(&number_list);
    println!("El numero mas grande es: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = mas_grande(&char_list);
    println!("La letra mas grande es: {}", result);

    let enteros = Point { x: 5, y: 10 };
    print!("{} \n", enteros.x());
    let decimal = Point { x: 12.5, y: 12.6 };
    print!("{} \n", decimal.x());
    let mix = Point { x: 10, y: 12.5 };
    print!("{} \n", mix.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mezclado(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
