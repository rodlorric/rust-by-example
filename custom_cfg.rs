#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

//rustc -o a.out --cfg some_condition custom_cfg.rs && ./a.out
fn main() {
    conditional_function();
}
