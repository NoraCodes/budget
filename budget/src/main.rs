
fn main() {
    use std::env;
    // Collect arguments into a vector
    let mut arguments = Vec::new();
    for argument in env::args() {
        arguments.push(argument);
    }
}
