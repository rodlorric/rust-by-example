struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}
// This is my own implementation.
trait Gray {}

impl Red for Cardinal {}
impl Blue for BlueJay {}
// This is my own implementation.
impl Gray for Turkey {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }
// This is my own implementation.
fn gray<T: Gray>(_: &T) -> &'static str { "gray" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    println!("A turkey is {}", gray(&_turkey));
    // ^ TODO: Try uncommenting this line.
}
