#![allow(unused_variables)] // Don't worry about this. This is for the LSP.
#![allow(dead_code)]

// Structs are used to package together and name multiple related values that define an meaningful
// group.
// Here, we are creating a `User` struct which has 4 values associated with it.
struct User {
    // This is the syntax to create a `struct` in rust
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn create_new_user(username: String, email: String) -> User {
    User {
        // Create a new `User` instance and return it.
        active: true,
        username, // Shorthand for `username: username`
        email,    // Shorthand for `email: email`
        sign_in_count: 1,
    }
}
fn main() {
    let new_user1 = create_new_user(
        String::from("Hrigved1"),
        String::from("iamhrigved@gmail.com"),
    );
    println!(
        "Username: {}, Email: {}",
        new_user1.username, new_user1.email,
    );

    // We can also create new structs from existing struct's data.
    // new_user2 has the same data as new_user1 except the username
    let new_user2 = User {
        username: String::from("Hrigved2"),
        active: new_user1.active,
        email: new_user1.email,
        sign_in_count: new_user1.sign_in_count,
    };

    // There is also a shortcut for all of this!
    let new_user3 = User {
        email: String::from("iamhrigved3@gmail.com"),
        ..new_user2 // This means that all values of new_user3 are same as new_user2 except for the
                    // email.
    };

    // NOTE: When we create new instances for User struct using the data in already declared
    // instances, the `username` and the `email` values are "moved" because String datatype does
    // not implement the `Copy` trait which means, new_user1 and new_user2 are NOT valid after we
    // create new_user3.
    // println!("new_user2 email: {}", new_user1.email); // error

    // There are also structs which do not require a `name` field!
    // They are called "tuple structs"
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // These are useful when you want your tuple to have a name and a type "different" from other
    // similar tuples.
    // For example, suppose if you want to make a function which excepts or returns a specific type
    // of tuple (say a Color), then if you try to pass a different type of tuple to it, (say Point)
    // it will give an error.

    // NOTE: When defining the User struct, we specified the datatype of `email` and `username` to be
    // `String` and not `&str`. This is because we want the each instance of this struct to own all
    // of its data to make the variables valid as long as the instance itself is valid.
    // If we want to use a reference as a value of a struct, it will require the use of `lifetimes`
    // which we will cover in Chapter 10.

    // Similar to the `tuple` datatype, the value of a struct can be "partially" borrowed.
    let new_user4 = User {
        username: String::from("Hrigved4"),
        email: String::from("iamhrigved4@gmail.com"),
        active: true,
        sign_in_count: 1,
    };
    let email_new_user4 = new_user4.email;
    // println!("email of new_user4: {}", new_user4.email); // error
    println!("email_new_user4: {}", email_new_user4); // no error
}
