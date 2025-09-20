mod utils;

fn main() {
    // Example usage of lib.rs
    RustInitialPattern::hello();

    // Example usage of utils.rs
    let sum = utils::add(2, 3);
    println!("Sum from utils: {}", sum);
}
