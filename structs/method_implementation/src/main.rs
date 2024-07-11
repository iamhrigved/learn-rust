#![allow(dead_code)] // This is for the LSP

#[derive(Debug)] // Our same old struct `Rectangle`
struct Rectangle {
    width: u32,
    height: u32,
}

// `Methods` are similar to functions, but they only defined in the context of the struct.
// The Methods tied to a struct, take the struct instance (here, Rectangle) as a parameter.
// This parameter is usually named `self`. In this case, we take the reference to the Rectangle
// instance.

// This is the `impl` (implementation) block for the struct Rectangle.
impl Rectangle {
    // `&self` is short for `self: &Self`. In rust, inside an impl block, `Self` is an alias for
    // the actual type that the impl block is for.
    // So, you can write any of these: `&self` or `self: &Self` or even `self: &Rectangle`
    // Notice, that the name of the variable SHOULD be `self` to let rust know that this is the
    // instance variable.
    // Since, we only want to read from self, we don't need to own it, hence the `&`.
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // We can also create a method with the same name as a field in the struct.
    // When we follow the name with parenthesis, rust recognizes that it is a method, and not a field!
    fn width(&self) -> bool {
        self.width == 0
    }

    // Can a rectangle hold another rectangle?
    fn can_hold(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // All functions inside the impl block are "associated" to the type for which the impl block is
    // for. We can also define functions which do not require the `self` parameter. These functions
    // are NOT methods. They are often used for constructors that will return a new instance of the
    // type. They can be used like `Self::new()` using the `::` syntax.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // Setting the width.
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // Returning a new Rectangle which is made by combining the max width and height of two
    // Rectangles.
    fn max(self, other: &Self) -> Self {
        Self {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
    fn ramesh() {
        println!("Ramesh");
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("The area of this rectangle is {}", rect1.area());
    println!(
        "The fact that the width of this rectangle is 0 is \"{}\"",
        rect1.width()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 10,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let square1 = Rectangle::square(10);
    println!("Area of square1 is {}", square1.area());

    // If we do something like,
    let square2 = &Box::new(Rectangle::square(9)); // Reference to a Box containing a Rectangle
                                                   // object.
    let area1 = square2.area(); // To get to the actual object, we need to dereference twice.
                                // But, we do not do that. The rust compiler will add as many
                                // reference or dereference operators to match the type when using
                                // the `.` operator.
    let area2 = Rectangle::area(square2); // We only need to care about the `&` and `*` we
                                          // EXPLICITLY declared. Hence, we do NOT use the `&`
                                          // operator when we passed in square2 because it is
                                          // already a reference to something (it doesn't matter if
                                          // it is a Box).
    assert_eq!(area1, area2);

    // Also,
    let square3 = Box::new(Rectangle::square(8)); // Box containing a Rectangle object.
    let area3 = square3.area();
    let area4 = Rectangle::area(&square3); // Here, we DO need to pass a reference because in the
                                           // declaration of square3, we did NOT specify that it is
                                           // a reference!
    assert_eq!(area3, area4);

    // NOTE: Calling a method which expects `self` as a parameter, will MOVE the object.
    let rect = Rectangle {
        width: 15,
        height: 17,
    };
    let other_rect = Rectangle {
        width: 21,
        height: 13,
    };
    let max_rect = rect.max(&other_rect);
    // dbg!(rect); // error because the value is moved to max_rect
    dbg!(max_rect); // no error
    dbg!(other_rect); // no error
}
