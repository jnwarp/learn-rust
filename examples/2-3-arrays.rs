use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element: {}", slice[0]);
    println!("slice has {} elements", slice.len());
}

fn main() {
    // Fixed-sized array
    let xs: [i32; 4] = [1, 2, 3, 4];

    // All elements can be initialized to same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first: {}", xs[0]);
    println!("second: {}", xs[1]);

    // Print length of array
    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can automatically be borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
}
