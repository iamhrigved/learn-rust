use std::io;

fn main() {
    println!("Hello, world!");

    another_function();

    println!("Please enter a number: ");

    let mut msg = String::new();

    io::stdin()
        .read_line(&mut msg)
        .expect("Opps! An ERROR occured");

    let msg: u32 = match msg.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number!");
            return;
        }
    };
    echonum(msg);

    // let y = (let x = 2); // `let x = 2` is a "Statement" which does not return anything so, this
    // is an error

    let y = {
        // here, the code block inside {} is an "Expression",  which evaluates something and
        // returns it.
        let x = 2;
        x + 2 // Notice that this line has no semicolon in the end. It means that this is an
              // expression, which returns the value 4. If we add a semicolon, this code block
              // will not return anything, but to make this return someting, we can also use
              // regular return statement that is, `return x + 2;` << this has the semicolon.
    };

    println!("the value of y is: {y}");

    println!("the funcion sum() returned: {}", sum(23, 32)); // calling the function
}

fn another_function() {
    println!("This is the output from `another_function`")
}
fn echonum(num: u32) {
    // you HAVE to declare the types of each parameter in rust which means you
    // almost do not have to specify their types again in the scope of the function.
    println!("Your number was: {num}")
}
fn sum(x: i32, y: i32) -> i32 {
    // you also HAVE to declare the type of the return value, that is,
    // the value the function is going to return
    // you can specify the datatype of return value using an arrow
    // symbol `->`

    // return x + y; << can also use this
    x + y // see line 32
}
