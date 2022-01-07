// extern crate rary; // May be required for Rust 2015 edition or earlier

fn main() {
    lib::public_function();

    // Error! `private_function` is private
    //lib::private_function();

    lib::indirect_access();
}
