// Define a function `printer` that takes a generic type `T` which
// must implement trait `Display`.
//fn printer<T: Display>(t: T) {
//    println!("{}", t);
//}

//struct S<T: Display>(T);

// Error!, `Vec<T>` does not implement `Display`. This
// specialization will fail.
//let s = S(vec![1]);


// A trait which implements the print marker: `{:?}`.
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

// This is my own implementation of HasArea for a Triangle.
impl HasArea for Triangle {
    fn area(&self) -> f64 { (self.length * self.height) / 2.0 }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
//#[allow(dead_code)]
#[derive(Debug)]
struct Triangle { length: f64, height: f64 }

// The generic `T` must implement `Debug`. Regardless
// of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any type which meets
// the bound can access `HasArea`'s function `area`.
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    print_debug(&_triangle);
    println!("Area: {}", area(&_triangle));
    // ^ TODO: Try uncommenting these.
    // | Error: Does not implement either `Debug` or `HasArea`
}
