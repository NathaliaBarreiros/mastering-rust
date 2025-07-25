use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

// The problem is that each write! generates a fmt::Result.
// Proper handling of this requires dealing with all the results.
// Rust provides the ? operator for exactly this purpose.

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // index in `index`.
        for (index, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", index, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")?;
        Ok(())
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
