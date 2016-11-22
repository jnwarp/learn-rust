fn main() {
    // The {} is replaced with any arguments.
    println!("{} days", 30);

    // Positional arguments can be used in the {}.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments:
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`
    println!("{} of {:b} people know binary", 1, 2);

    // You can right-align text with a specified width
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeros
    println!("{number:>0width$}", number=1, width=6);
}
