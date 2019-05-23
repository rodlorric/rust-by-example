#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Struct can be reused as fields of another struct.
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn square(p0: Point, len: f32) -> Rectangle {
	let pf = Point { x: p0.x + len, y: p0.y + len };
	Rectangle { p1: p0, p2: pf }
}

fn rect_area(rect: Rectangle) -> f32 {
	let x_abs = (rect.p1.x - rect.p2.x).abs();
	let y_abs = (rect.p1.y - rect.p2.y).abs();
	x_abs * y_abs
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point.
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one.
    let new_point = Point { x: 0.1, ..point};

    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding.
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too.
        p1 : Point { x: my_y, y: my_x },
        p2 : point,
    };

    // Instantiate a unit struct.
    let _nil = Nil;

    // Instantiate a tuple struct.
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct.
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct.
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let pnt_1 = Point { x: 0.0, y: 0.0 };
    let pnt_2 = Point { x: 4.0, y: 2.0 };
    println!("area: {}", rect_area(Rectangle {p1: pnt_1, p2: pnt_2}));
    println!("square: {:?}", square(Point {x: 0.0, y: 0.0}, 8.0));
    println!("area square: {:?}", rect_area(square(Point {x: 0.0, y: 0.0}, 8.0)));
}
