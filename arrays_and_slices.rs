use std:mem;

// This function borrows a slice.
fn analyze_slice(slice: &[i32]) {
	println!("first element of the slice: {}", slice[0]);
	println!("the slice has {} elements.", slice.len());
}

fn main() {
	// Fixed-size array (type signature is superfluos)
	let xs: [i32; 5] = [1, 2, 3, 4, 5];
