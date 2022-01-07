pub fn public_function() {
    println!("Cool Date Lib");
}

fn private_function() {
    println!("sub routine");
}

pub fn indirect_access() {
    print!("Cool Date API");

    private_function();
}
