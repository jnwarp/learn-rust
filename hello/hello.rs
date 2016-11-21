// Run 'rustc hello.rs' to compile this file

// Create a new macro called say_hello
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument
    () => (
        // Everything inside this block is expanded
        println!("Hello world!");
    )
}

// This is the main function
fn main() {
    say_hello!()
}
