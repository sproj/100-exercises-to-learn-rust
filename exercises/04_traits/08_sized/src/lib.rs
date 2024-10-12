pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    // std::mem::size_of::<str>(); // size of String or str is not known - they are DST and do not have the `Sized` marker trait.
    std::mem::size_of::<&str>(); // size of a &str is known - it is a fat pointer to a string -> it's size is 2 `usize`: 
    // one for the pointer, one for the length of the referenced string.
}
