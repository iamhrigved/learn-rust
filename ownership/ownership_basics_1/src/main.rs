fn main() {
    // concept of scope
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
        println!("{s}")
    } // this scope is now over, and s is no longer valid

    // the `String` datatype is used when we do not know how much space a variable is going to take
    // at compile time
    // so, this datatype stores data on the heap which makes it able to store an amount of text,
    // which is not known at compile time
    // string literals such as `let s = "Hello World!"` are immutable but `String` is mutable

    let mut s = String::from("hello"); // we can create a `String` form a string, using the `from`
                                       // function

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // next, read the `Memory and Allocation` section of ch-04

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
        println!("{s}")
    } // this scope is now over, and s is no longer valid
      //just after the closing curly brace, a function `drop` is called automatically
      // by rust which gives back the memory to the allocator

    // here, both s1 and s2 are pointing to the same data on the heap
    // the thing which is being copied here is nothing but the `String` data that is, the pointer
    // to the actual data on the heap
    // if we instead copied the actual data, this process would be very expensive

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1: {s1}") // Error because s1 is now no longer valid and the value of s1 that is,
    // "hello", has moved to s2
    println!("s2: {s2}"); // no error

    // since rust invalidates the first variable, this process is called a `move`
    // instead of a `copy`

    let s3 = String::from("hello");
    let s4 = s3.clone(); // creates a deep copy manually

    println!("s3: {s3}, s4: {s4}"); // both, s3 and s4 are valid

    // rust by default clones only the variables whose sizes are know at compile time
    // because these values are stored in the stack making the process inexpensive
    let str1 = "Hello";
    let str2 = str1; // same goes for integers and float
    println!("str1: {str1}, str2: {str2}"); // no error

    let str = String::from("Hello");
    takes_ownership(str); // the ownership of str is given to the fn
                          // `println!("{str}")` gives error

    let num = 5;
    makes_copy(num); // a copy of num is sent to the fn
                     // `println!("num: {num}")` gives no error

    let str2 = String::from("World");
    let str3 = takes_and_gives_back(str2); // ownership comes back to str3
                                           // but str2 is still invalid

    println!("str3: {str3}");

    // to get the ownership back and also to prevent copying, we can use this kind of tuple
    let (str4, length) = calculate_length(str3);
    println!("str4: {}, length: {}", str4, length);

    // you will understand more in the 'ownerwhip_basics_2' and 'reference_and_borrowing' sections
}

fn takes_ownership(str: String) {
    println!("str: {str}")
} // str is dropped

fn makes_copy(num: i32) {
    println!("num: {num}")
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string // ownership is returned
}
fn calculate_length(str: String) -> (String, usize) {
    let length = str.len();
    (str, length) // calculate and also give back the ownership
}
