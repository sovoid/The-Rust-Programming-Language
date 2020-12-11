fn main() {
    // * Rust is statically typed with type inference
    let x = 5;

    // * Need to annote type where type is not clear
    let forty_two: u32 = "42".parse().expect("Not a number!");

    // * Architecture dependent Integer types
    let y: usize = 32;
    let z: isize = 32;

    // * Floating points
    let not_an_integer = 4.0; // f64
    let also_not_an_integer: f32 = 3.1415;

    // * Numeric operations
    let sum = 5 + 10;
    let difference = 15.5 - 3.5;
    let product = 4 * 30;
    let quotient = 3.2 / 5.8;
    let remainder = 43 % 5;

    // * Boolean types
    let rust_is_awesome: bool = true;
    let rust_is_boring = false;

    // * Character types
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // * Tuples
    let tup: (i32, f32, char) = (3, 4.8, '😻');

    // Destructuring a tuple
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Indexing
    let three = tup.0;
    let cat_emoji = tup.2;

    // * Arrays
    let first_five = [1, 2, 3, 4, 5]; // fixed length and same type

    // With annotation
    let next_five: [i32; 5] = [6, 7, 8, 9, 10];
    let threes = [3, 5]; // [3, 3, 3, 3, 3]

    // Indexing
    let two = first_five[1];

    // ! Error: index out of bound
    // let eleven = next_five[10];
}
