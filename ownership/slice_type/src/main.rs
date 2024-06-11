#![allow(unused_variables)]
fn main() {
    // if suppose we want to get the first word from a String
    let name = String::from("Hrigved Chess");

    // we could write a function, which returns a reference to the first word
    // of a String
    // But how?

    let first = first_word(&name);
    println!("{}", first);

    // a `string slice` is a reference to a part of a String
    // we can declare slices using the `s[start..end]` syntax
    // where `start` is inclusive and `end` is exclusive
    let first_name = &name[..7]; // starting 0 can be omitted
    let last_name = &name[7..]; // ending can also be omitted

    // NOTE: the string is NOT cloned in any way when we create a slice.
    //  the slice i.e. &str just stores the memory address of the first
    //  element of the slice and also it's length (but not it's capacity)
    // That means, it's just a pointer to the first element of the slice

    // Also, a function which takes &String as a parameter, CANNOT take &str
    // BUT, a function which takes &str, CAN take &String also
    // so, it would be a great idea to change &String to &str in the function
    // declaration
    assert_eq!(&name, &name[..]); // same

    // same can be done to a vector or an array
    // you can do it yourself ;)
}

fn first_word(s: &String) -> &str {
    // change &String to &str
    // see the return type
    let bytes = s.as_bytes(); // converting `s` to an array of bytes

    for (index, &val) in bytes.iter().enumerate() {
        // we will look at iterators in the upcoming chapters
        // for basic understanding, just remember that
        // iter() iterates over the elements, and enumerate()
        // returns a tuple with index, and a reference to the value of that index

        if val == b' ' {
            // byte literal syntax
            return &s[..index]; // this is the way to declare slices
        }
    }

    &s[..] // equivalent to &s[0..s.len()]
}
