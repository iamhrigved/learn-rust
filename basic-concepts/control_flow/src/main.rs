fn main() {
    let number = 3;

    // basic if-else syntax
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    };

    // we can also use if-else in a let statement
    // BUT the datatypes of the values returned must be SAME
    // to make handling the returned value safe
    let condition = true;
    let num = if condition { 5 } else { 7 };

    println!("the number is: {num}");

    // we can also use loop with a let statement which returns a particular value.
    let age: u8 = loop {
        let mut msg = String::new();
        println!("Please enter your age: ");
        std::io::stdin().read_line(&mut msg).expect("Oops!");
        match msg.trim().parse() {
            Ok(num) => {
                break num;
            }
            Err(_) => {
                println!("Please enter a valid age!");
                continue;
            }
        };
    };
    println!("Your age is: {age}");

    // if we have various nested loops, we can break even an outer from an inner loop
    // using labels on loops
    let mut i = 1;
    'outerloop: loop {
        if i > 3 {
            break;
        }
        println!("Outerloop");
        let mut j = 1;
        loop {
            if j > 3 {
                break;
            }
            // try to uncomment these lines and see what happens
            // if i == 2 && j == 3 {
            //     break 'outerloop;
            // }
            println!("Innerloop");
            j += 1;
        }
        i += 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // an example of regular while loop
    while index < 5 {
        println!("the value from `while` loop is: {}", a[index]);
        index += 1;
    }

    // better to use a for loop in this case
    for element in a {
        println!("the value from `for` loop is: {element}")
    }

    // for loop can also be used in custom ranges just like python :)
    // `1..5` is equivalent to range(1, 5) in python
    // also, `(1..5).rev()` is equivalant to range(4, 0, -1)
    for number in 1..5 {
        print!("{number} ")
    }
}
