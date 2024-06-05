fn main() {
    let num: u64 = "42".parse().expect(""); // `u` and `i` are prefix used to tell apart
                                            // `unsigned` or `signed` integer data types
                                            // there can be 8, 16, 32(default), 64, 128 and arch(64
                                            // or 32 depending on the architecture of the system)
                                            // -bit integer types.
    println!("{num}");

    let ft: f32 = "3.1415".parse().expect(""); // `f` is the prefix used to tell that the number is
                                               // a floating point number. There are two floating
                                               // data types: f32, f64(default)
    println!("{ft}");

    let sum = 5 + 10;

    let difference = 95.5 - 4.3; // subtraction

    let product = 4 * 30; // multiplication

    let quotient = 56.7 / 32.2; // division

    let truncated = -5 / 3; // Results in -1

    let remainder = 43 % 5; // remainder

    println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");

    let f = false;
    let t: bool = true;

    println!("{t}, {f}");

    let c = 'z'; // `char` data type has 4bytes = 32-bits of storage and are used to store unicode
                 // symbols. They are denoted by surrounding them with `'` (single quotes)
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{c}, {z}, {heart_eyed_cat}");

    let tup: (i32, f32, bool) = (1234, 27.123, true); // `tuple` data type is a compound data type
                                                      // which can store multiple values having
                                                      // multiple data types. They have a fixed
                                                      // length
    let (x, y, z) = tup;

    println!("{x}, {y}, {z}");

    let subtup = tup.2; // the elemens of tuple can be accesed using `tup.{index}`

    println!("{subtup}");

    let arr: [i32; 3] = [1, 12, 123]; // `array` data type is also compound data type which is very
                                      // similar to `tuple` but can only store a single type of
                                      // data inside. These also have a fixed length.
    let [a, b, c] = arr;

    println!("{a}, {b}, {c}");

    let subarr = arr[2]; // the elements of an array can be accesed using `arr[{index}]`

    println!("{subarr}");
}
