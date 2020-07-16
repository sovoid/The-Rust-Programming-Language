fn main() {
    // Scalar types hold only one value

    let _my_int: u32 = 56;
    let _my_float: f64 = 3.0;
    let _my_char: char = '💡';
    let _my_bool: bool = true;
    println!("Hello, world!");

    // Compound types hold multiple values of same or different types

    // Tuple can hold different types
    let _my_tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let (_x, _y, _z) = _my_tup; // Destructuring

    println!("The value of y is: {}", _y);
    println!("The value of z is: {}", _my_tup.2); // Indexing

    // Arrays need to be of the same type
    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let my_arr: [i32; 5] = [1, 2, 3, 4, 5]; // Declare +  Initialize
    let _all_falses = [false; 5]; // = [false, false, false, false, false]

    println!("The second element of my_arr is: {}", my_arr[1]);
}
