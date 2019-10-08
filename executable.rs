// Link to `library`, import items under the `rary` module
extern crate rary;

fn main() {
    rary::public_function();

    // Error! `private_function` in private
    // rary::private_function();

    rary::indirect_access();
}
