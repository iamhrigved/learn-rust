fn main() {
    // suppose we need the create a long array which requires huge memory

    let arr1 = [0; 1_000];
    let arr2 = arr1;
    // at this point, there are two copies of the same array in the stack
    // which wastes memory

    // to prevent this, we can use pointers

    let arr3 = Box::new([0; 1_000]);
    // here, `Box::new([0; 1_000])` allocates the memory of the array in the heap
    // and returns a pointer to that allocated memory which is then stored in `arr3`

    // but now, if we do
    let arr4 = arr3;
    // the pointer to that array is now stored in `arr4`
    // but what about arr3?
    // arr3 is removed from existence :(
    // but this is what makes rust so safe :)
    // let x = arr3[0]; // error

    // also, suppose we no longer need the array which is stored in the heap
    // can we do something like `free(arr4)`?
    // no, rust does not permit manual memory management unlike many other low level languages
    // like c and c++
    // rust automatically deallocates the memory of a box when the variable owning the box
    // (the variable which points to the box), goes out of scope
    create_and_drop();

    // NOTE: `String`, `Vec`, and `HashMap` data structures use this `Box` method by default
    // which means, they store their value in heap and return a pointer (just like `String::from("Hello")`)
    // go back to ownership_basics_1 and see if you understand better :)
}

fn create_and_drop() {
    let a_box = Box::new(5); // memory is allocated in the heap
} // the variable a_box along with the memory is deallocated
