fn main() {
    let logical: bool = true;
    println!("logical {:?}", logical);
    
    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation
    println!("a_float {:?}", a_float);
    println!("an_integer {:?}", an_integer);

    // Default types
    let default_float   = 2.0;
    let default_integer = 12;
    println!("default_float {:?}", default_float);
    println!("default_integer {:?}", default_integer);

    // Cannot change type of a variable
    let mut mutable = 12;
    println!("mutable {:?}", mutable);
    mutable = 11;
    println!("mutable {:?}", mutable);
}
