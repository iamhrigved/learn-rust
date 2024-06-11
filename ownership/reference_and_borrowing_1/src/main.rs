fn main() {
    // in ownership_basics_1, the last function used a tuple to return the string back
    // this is not good practice because it is difficult to read the code
    // this can be replaced by `reference`

    let s1 = String::from("Hello, World");
    let length = calculate_length(&s1); // we send a reference to the fn by prepending '&' in front
                                        // of the variable name
    println!("The length of \"{s1}\" is {length}"); // s1 and length both are valid

    // NOTE: the &s1 syntax lets us to create a reference which refers to the value of s1
    //  but not own it
    //  which means, when s goes out of scope, the value of s1 still remains
    // also, the process of making a reference is called "borrowing"

    // now, what if try to modify the value we are borrowing?
    // it doesn't work because like variables, references are immutable by default
    // to make them mutable, we can do something like this

    let mut s2 = String::from("Hrigved"); // to create a mutable reference,
                                          // we need a mutable variable
    add_suffix(&mut s2);
    println!("s2: {s2}"); // s2 is valid and also updated with the new value

    // NOTE: there can only be 1 mutable reference at a time
    //  this is done to prevent "data races" at compile time
    let _s2ref1 = &mut s2;
    let s2ref2 = &mut s2; // _s2ref1 is discarded and only s2ref2 is valid

    // println!("{}, {}", _s2ref1, s2ref2); // error (using both references at the same time)
    // println!("{}", _s2ref1); // error
    println!("{}", s2ref2); // no error

    // NOTE: we also can not have immutable and mutable references at the same time
    //  because users of an immutable reference don't expect the value to suddenly change
    let s2ref3 = &s2;

    // println!("{}, {}", s2ref2, s2ref3); // error (using mut and immut at the same time)
    println!("s2: {}", s2ref3); // no error

    // in other languages, you can easily (accidently) create a dangling pointer,
    // which is pointer that references a location in memory that may have been given
    // to someone else
    let reference_to_nothing = dangle();
    println!("{reference_to_nothing}");

    // NOTE: in the fn, dangle(), when we return `s`, there is no error because
    //  it is a `String` struct, and when we return it, the value's ownership
    //  is transfered to the variable `reference_to_nothing` and the value is NOT
    //  dropped by the program after the function scope ends
    //  but, when we try to return the reference of `s` (&s), the value's ownership
    //  does NOT change and when `s` goes out of scope, the value gets deallocated
}
fn calculate_length(s: &String) -> usize {
    // this fn accepts a reference to a `String` and returns
    // `usize` (by default, `len()` returns `usize` instead of `u32)
    let length = s.len();
    length
}
fn add_suffix(s: &mut String) {
    // this fn accepts a reference to a mutable `String`
    s.push_str("Chess");
}
fn dangle() -> String {
    // if we used `dangle() -> &String`, and returned `&s` that would cause an error
    let s = String::from("This is a reference");
    s // no error
}
