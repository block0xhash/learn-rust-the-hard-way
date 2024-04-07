use std::fmt;

// tuple is a collection of different types
// constructed using parentheses ()
// type signature (T1, T2,...TN) where TN is the members types

// can be used as function argument and return types
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // bind members of a tuple to variables using `let`
    let (init_param, bool_param) = pair;

    (bool_param, init_param)
}

// the following struct is for the activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn transpose(matrix: Matrix) -> Matrix {
    // Swap rows and columns
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:.1} {:.1})\n({:.1} {:.1})",
            self.0, self.1, self.2, self.3
        )
    }
}

fn main() {
    // tuple with bunch of different types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    //values can be extracted from the tuple using tuple indexing
    println!("first value: {}", long_tuple.0);
    println!("2nd value: {}", long_tuple.0);

    //Tuples can be tuple members also
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    //Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples.0 .2);

    // But long Tuples (more than 12 elements) cannot be printed.
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);
    println!("tuple is {:?}", tuple);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("{:?}", matrix); // Debug
    println!("{\n}", matrix); // Display
    println!("Transpose:\n{}", transpose(matrix)); //Display
}
