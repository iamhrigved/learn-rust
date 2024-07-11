#[derive(Debug)] // used to debug (print) non-primitive datatypes
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width: u32 = 10;
    let height: u32 = 20;
    println!(
        "The are of rectangle using normal variables is {}",
        area(width, height)
    );

    let dimensions: (u32, u32) = (10, 20);
    println!(
        "The are of rectangle using tuples is {}",
        area_dim(dimensions)
    );

    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    println!(
        "The are of rectangle using structs is {}",
        area_struct(&rect)
    );

    // To print a struct (a non-primitive datatype), we first need to implement the `Debug` trait to
    // our struct which makes it printable
    println!("Printed using \"{{:?}}\" {:?}", rect); // but it's not really pretty :(
    println!("Printed using \"{{:#?}}\" {:#?}", rect); // now that's cool :)

    // To print this struct, we can also use the `dbg!` macro.
    // The only problem with this macro is that it takes the ownership of the variable (but it also
    // returns it). So, to prevent this from happening, we can pass a reference to rect.
    let rect = dbg!(rect); // no error
    dbg!(&rect); // no error

    // We will learn about more traits in Chapter 10.
    // Also, if you have noticed, the area method is very specific to our struct `Rectangle`. Can
    // we tie this function "very" close to the struct? We can!
    // We will implement this behaviour in the next section `method_implementation`
}

// less readable
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// more readable
fn area_dim(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// best
fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
