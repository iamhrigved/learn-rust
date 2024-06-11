#![allow(unused_variables)]
fn main() {
    // in the previous chapter, `reference_and_borrowing_1`, we passed a reference
    // of a variable stored in the heap, to a function
    // but what if we want the actual value of the variable inside the function?
    // in that case, we use `dereferencing`

    let mut x: Box<i32> = Box::new(10); // note the datatype
    let a: i32 = *x; // y is the actual value of x i.e. 10
    *x += 1; // now, x points to the value 11

    let r1: &Box<i32> = &x; // `r1` is a reference to a Box,
                            // whose owner is x
    let b: i32 = **r1; // we need 2 dereferencing to get the actual value

    let r2: &i32 = &*x; // `r2` is a pointer/reference to the value of x
    let c: i32 = *r2; // only requires 1 dereference

    let y: Box<i32> = Box::new(-1);
    let x_abs1: i32 = i32::abs(*y); // explicit dereference
    let x_abs2: i32 = y.abs(); // implicit dereference (automatically done by the function)
    assert_eq!(x_abs1, x_abs2); // both are equal

    let r: &Box<i32> = &y; // r is a reference to the owner of the Box
    let r_abs1: i32 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2: i32 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    // the `Vec` data structure (vector)
    // `Vec` are like arrays (only allow 1 datatype) but they do not have a fixed lenght
    // `Vec` also store their data on the heap so methods like Vec::push(),
    // can alter the data stored

    let mut v: Vec<i32> = vec![1, 2, 3]; // notice the datatype
                                         // `vec!` macro creates a vector
    let v_len1: usize = v.len();
    let v_len2: usize = Vec::len(&v);
    assert_eq!(v_len2, v_len1);

    // when a vector is altered i.e. when its length is increased beyond its capacity,
    // deallocates all the elements in it, and move to a new place on the heap with more capacity
    let num: &i32 = &v[2]; // num is a reference to the third element of v
    v.push(4);

    // println!("Third element is {}", *num); // error
    // here, when a new element is pushed in the vector, num, which is a pointer to the
    // third element of the vector, is left dangling (see reference_and_borrowing_1)

    // the elements of vector and array datatypes cannot be partially moved, i.e.
    // if we create a vector with a `String` (or any other datatype which do not
    // implement the 'Copy' trait) then we try to take ownership of that String,
    // it doesn't work

    let mut v2 = vec![String::from("IamHrigved")];
    // let v2_own = v2[0]; // error
    let v2_ref = &v2[0]; // this is an alternative
    let v2_own = v2.remove(0); // this is an alternative (but v2 is empty after this)
    assert!(v2.len() == 0);

    // if try to do this with a tuple instead, we have,
    let tup = (String::from("IamHrigved"),);
    let mut t_own = tup.0; // no error
    t_own.push_str(" Hi!"); // can also mutate because t_own "owns" the String
    println!("{}", t_own);

    // but, can we not just borrow a part of a vector, or an array, if really have to?
    // we will look at this in the next part, `slice_type`
}
