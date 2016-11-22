use std::fmt; // Import `fmt`

// aruments are in the reverse()
// returns are after the ->
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to a variable
    let (integer, boolean) = pair;

    (boolean, integer)
}

// Define the debug output to display for Matrix
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// Implement `Display` for `Matrix`
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    // A long tutple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Values can be extracted from the tuple using indexing
    println!("first value: {}", long_tuple.0);
    println!("second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    // Print the tuple
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // Print and reverse a pair of tuples
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("reversed pair is {:?}", reverse(pair));

    // Create a one element tuple, the comma is needed to tell them apart
    println!("one element tuple: {:?}", (8u32,));
    println!("just an integer: {:?}", (8u32));

    let matrix = Matrix(1.1, 1.2, 1.3, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);
}
