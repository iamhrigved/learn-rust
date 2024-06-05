fn main() {
    let mut y = 5; // with `mut`, you can not change the data type of the variable but it can be
                   // reassigned
    let x = 5; // here, every time `let x = {something}` is happening, we are effectively creating
               // a new variable which means we can even change the datatype of `x`

    y = y + 1;
    let x = x + 1;

    {
        y = y * 2;
        let x = x * 2;
        println!("In the inner scope,\nx = {x}\ny = {y}");
        let x = "This is a string!";
        println!("The value of x in the inner scope is `{x}`")
    }

    println!("In the outer scope,\nx = {x}\ny = {y}");
}
